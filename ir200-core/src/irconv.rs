//! WAV impulse response → the 3354 float32 values an IR-200 user-IR slot
//! stores (the coefficient block at slot base + `02 24`).
//!
//! A reimplementation of what IR Loader computes, verified bit-exact on a real
//! slot (see `docs/ir-conversion.md`). The pedal plays a sparse FIR at 96 kHz
//! (1640 taps chosen from the first 4080) plus a 24-segment tail model; the
//! WAV itself is never stored.
//!
//! Layout of the result:
//!
//! | index | count | content |
//! |---|---|---|
//! | 0 | 1640 | tap values |
//! | 1640 | 1640 | tap positions × 2⁻¹⁴ (padding 4080 × 2⁻¹⁴) |
//! | 3280 | 24 | tail delays − 2 |
//! | 3304 | 24 | tail sources × 2⁻¹⁴ |
//! | 3328 | 24 | tail gains |
//! | 3352 | 1 | lead trim (96 kHz samples) |
//! | 3353 | 1 | 1.0 |

use realfft::RealFftPlanner;

/// Internal sample rate.
const FS: f64 = 96000.0;
/// At most 0.5 s at 96 kHz is used.
const MAX_SAMPLES: usize = 48000;
/// Leading samples below this (judged on the first channel) are dropped.
const SILENCE: f64 = 1.0 / 512.0;
/// IR Loader's π literal (not `std::f64::consts::PI`); needed for exactness.
#[allow(clippy::approx_constant)]
const PI: f64 = 3.14159265358979;
const SINC_HALF_WIDTH: usize = 1024;
const NORM_LO: f64 = 0.0020833333333333333;
const NORM_HI: f64 = 0.003125;
const HB_HALF_WIDTH: usize = 100;
const MAX_TAPS: usize = 4080;
/// Non-zero FIR taps stored.
pub const NUM_TAPS: usize = 1640;
/// Tail-model segments.
pub const TAIL_SEGMENTS: usize = 24;
const TAIL_SEARCH: i64 = 2880;
const TAIL_STEP: i64 = 72;
const POS_SCALE: f64 = 1.0 / 16384.0;
/// Values in a slot's coefficient block.
pub const NUM_COEFS: usize = 2 * NUM_TAPS + 3 * TAIL_SEGMENTS + 2;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum WavError {
    #[error("not a WAV file")]
    NotWav,
    #[error("the WAV file has no audio data")]
    NoData,
    #[error("{0} channels: only mono and stereo WAV files can be loaded")]
    Channels(u16),
    #[error(
        "unsupported WAV format (format {tag}, {bits} bit); use 16, 24 or 32 bit PCM, or 32 bit float"
    )]
    Format { tag: u16, bits: u16 },
    #[error("the WAV file is silent")]
    Silent,
}

/// A decoded WAV: samples per channel, full scale = 1.0.
#[derive(Debug, Clone)]
pub struct Wav {
    pub format_tag: u16,
    pub channels: u16,
    pub rate: u32,
    pub bits: u16,
    /// `samples[ch][i]`.
    pub samples: Vec<Vec<f64>>,
}

impl Wav {
    pub fn frames(&self) -> usize {
        self.samples.first().map_or(0, Vec::len)
    }
}

fn u16le(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}
fn u32le(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}

/// Read a RIFF/WAVE file the way IR Loader accepts it: format 1 (PCM 8/16/24/32
/// bit) or 3 (32-bit float), 1 or 2 channels, any sample rate.
/// WAVE_FORMAT_EXTENSIBLE is rejected, as IR Loader does.
pub fn read_wav(data: &[u8]) -> Result<Wav, WavError> {
    if data.len() < 12 || &data[0..4] != b"RIFF" || &data[8..12] != b"WAVE" {
        return Err(WavError::NotWav);
    }
    let mut fmt: Option<&[u8]> = None;
    let mut pcm: Option<&[u8]> = None;
    let mut pos = 12;
    while pos + 8 <= data.len() {
        let id = &data[pos..pos + 4];
        let size = u32le(data, pos + 4) as usize;
        let end = (pos + 8).saturating_add(size).min(data.len());
        let body = &data[pos + 8..end];
        if id == b"fmt " && fmt.is_none() {
            fmt = Some(body);
        } else if id == b"data" && fmt.is_some() && pcm.is_none() {
            pcm = Some(body);
        }
        pos = pos.saturating_add(8 + size + (size & 1));
    }
    let (fmt, pcm) = match (fmt, pcm) {
        (Some(f), Some(p)) if f.len() >= 16 => (f, p),
        _ => return Err(WavError::NoData),
    };
    let tag = u16le(fmt, 0);
    let ch = u16le(fmt, 2);
    let rate = u32le(fmt, 4);
    let align = u16le(fmt, 12) as usize;
    let bits = u16le(fmt, 14);
    if ch != 1 && ch != 2 {
        return Err(WavError::Channels(ch));
    }
    let width = match (tag, bits) {
        (3, 32) | (1, 32) => 4,
        (1, 24) => 3,
        (1, 16) => 2,
        (1, 8) => 1,
        _ => return Err(WavError::Format { tag, bits }),
    };
    if align < width * ch as usize || rate == 0 {
        return Err(WavError::Format { tag, bits });
    }
    let n = pcm.len() / align;
    let mut samples = vec![Vec::with_capacity(n); ch as usize];
    for i in 0..n {
        for (c, out) in samples.iter_mut().enumerate() {
            let s = &pcm[i * align + c * width..];
            let v = match (tag, bits) {
                (3, 32) => f32::from_le_bytes([s[0], s[1], s[2], s[3]]) as f64,
                (1, 8) => (s[0] as f64 - 128.0) * (1.0 / 128.0),
                (1, 16) => i16::from_le_bytes([s[0], s[1]]) as f64 * (1.0 / 32768.0),
                (1, 24) => {
                    let v = i32::from_le_bytes([0, s[0], s[1], s[2]]) >> 8;
                    v as f64 * (1.0 / 8388608.0)
                }
                _ => i32::from_le_bytes([s[0], s[1], s[2], s[3]]) as f64 * (1.0 / 2147483648.0),
            };
            out.push(v);
        }
    }
    if n == 0 {
        return Err(WavError::NoData);
    }
    Ok(Wav {
        format_tag: tag,
        channels: ch,
        rate,
        bits,
        samples,
    })
}

/// Drop leading silence (judged on channel 0), pick a channel, limit to 0.5 s.
/// Returns the samples and the lead trim in 96 kHz samples.
fn trim_and_select(wav: &Wav, channel: usize) -> (Vec<f64>, u32) {
    let fs = wav.rate as f64;
    let left = &wav.samples[0];
    let skipped = left
        .iter()
        .position(|x| x.abs() >= SILENCE)
        .unwrap_or(left.len());
    let mut count = left.len() - skipped;
    let lead_trim = (skipped as f64 * FS / fs) as u32;
    let limit = (fs * MAX_SAMPLES as f64 / FS + 0.5) as usize;
    if count > limit {
        count = limit;
    } else if count > MAX_SAMPLES {
        count = MAX_SAMPLES;
    }
    let ch = if wav.channels == 2 { channel.min(1) } else { 0 };
    (
        wav.samples[ch][skipped..skipped + count].to_vec(),
        lead_trim,
    )
}

fn sinc_at(a: f64) -> f64 {
    if a == 0.0 { 1.0 } else { a.sin() / a }
}

/// Truncated-sinc resampler (rectangular window, ±1024 input taps).
fn resample_to_96k(x: &[f64], fs: u32) -> Vec<f64> {
    let fsd = fs as f64;
    if fsd == FS {
        return x.to_vec();
    }
    let mut org = x.len();
    let mut conv = (FS / fsd * org as f64 + 0.5) as usize;
    if conv > MAX_SAMPLES {
        conv = MAX_SAMPLES;
        org = (fsd * MAX_SAMPLES as f64 / FS + 0.5) as usize;
    }
    let down = org > conv;
    (0..conv)
        .map(|n| {
            let pos = n as f64 * fsd / FS;
            let c = (pos + 0.5) as i64;
            let lo = (c - SINC_HALF_WIDTH as i64).max(0);
            let hi = (c + SINC_HALF_WIDTH as i64).min(org as i64 - 1);
            if lo > hi {
                return 0.0;
            }
            let mut acc = 0.0;
            for j in lo..=hi {
                let jf = j as f64;
                let a = if down {
                    (jf * FS / fsd - n as f64) * PI
                } else {
                    (jf - pos) * PI
                };
                acc += sinc_at(a) * x[j as usize];
            }
            if down { acc } else { (fsd / FS) * acc }
        })
        .collect()
}

/// Scale so the mean FFT magnitude over 200–300 Hz is 1 (FFT zero-padded to
/// 2^ceil(log2(35·len)) points), via the same forward/inverse round trip.
fn normalise(y: &[f64]) -> Vec<f64> {
    let m = y.len();
    let ex = ((m as f64 * 35.0).ln() / 2f64.ln()).ceil();
    let n = 2f64.powf(ex) as usize;
    let mut planner = RealFftPlanner::<f64>::new();
    let fwd = planner.plan_fft_forward(n);
    let inv = planner.plan_fft_inverse(n);
    let mut buf = vec![0.0; n];
    buf[..m].copy_from_slice(y);
    let mut spec = fwd.make_output_vec();
    fwd.process(&mut buf, &mut spec).expect("fft sizes match");
    let lo = (n as f64 * NORM_LO + 0.5) as usize;
    let hi = (n as f64 * NORM_HI + 0.5) as usize;
    let mut total = 0.0;
    for s in &spec[lo..hi] {
        total += (s.re * s.re + s.im * s.im).sqrt();
    }
    let r = (hi - lo) as f64 / total;
    for s in spec.iter_mut() {
        *s *= r;
    }
    // A real signal's DC and Nyquist bins are real; drop rounding residue.
    spec[0].im = 0.0;
    let last = spec.len() - 1;
    spec[last].im = 0.0;
    inv.process(&mut spec, &mut buf).expect("fft sizes match");
    let scale = 1.0 / n as f64;
    buf[..m].iter().map(|v| v * scale).collect()
}

/// Half-band low-pass (sinc(πk/2), Hann window over ±100 taps), evaluated on
/// even taps only; odd taps are zero.
fn halfband(y: &[f64]) -> Vec<f64> {
    let m = y.len();
    let w = 2 * HB_HALF_WIDTH + 1;
    let kernel: Vec<(f64, f64)> = (0..w)
        .map(|i| {
            let a = (i as f64 - HB_HALF_WIDTH as f64) * PI;
            (sinc_at(a * 0.5), (a / 100.0).cos() * 0.5 + 0.5)
        })
        .collect();
    let mut out = vec![0.0; m];
    for i in (0..m).step_by(2) {
        let mut acc = 0.0;
        for (t, &(s, win)) in kernel.iter().enumerate() {
            // pad[i + t] where pad = zeros(100) ++ y ++ zeros(100)
            let j = i as i64 + t as i64 - HB_HALF_WIDTH as i64;
            let v = if j >= 0 && (j as usize) < m {
                y[j as usize]
            } else {
                0.0
            };
            acc += v * s * win;
        }
        out[i] = acc;
    }
    out
}

/// Bisect a magnitude threshold so NUM_TAPS of the first `taps` coefficients
/// survive; zero the rest.
fn sparsify(c: &mut [f64], taps: usize) {
    let a: Vec<f64> = c[..taps].iter().map(|v| v.abs()).collect();
    let (mut lo, mut hi) = (0.0f64, 1.0f64);
    let mut th;
    let mut cut;
    loop {
        th = (hi + lo) * 0.5;
        cut = th;
        let cnt = a.iter().filter(|&&v| v > th).count();
        if cnt == NUM_TAPS {
            break;
        }
        if cnt > NUM_TAPS {
            lo = th;
        } else {
            hi = th;
        }
        if hi - lo < 1e-15 {
            cut = hi;
            break;
        }
        if th < 1e-15 {
            break;
        }
    }
    for (v, &m) in c[..taps].iter_mut().zip(&a) {
        if m <= cut {
            *v = 0.0;
        }
    }
}

struct Tail {
    d0: usize,
    delay: [i64; TAIL_SEGMENTS],
    source: [i64; TAIL_SEGMENTS],
    gain: [f64; TAIL_SEGMENTS],
}

/// Late zero crossings and 24 delayed, scaled copies of the FIR's end section
/// fitted to the IR beyond the FIR.
fn tail_model(c: &[f64], taps: usize, m: usize) -> Tail {
    let last = c[..taps].iter().rposition(|&v| v != 0.0).unwrap_or(0) as i64;
    let at = |x: i64| c[x as usize];
    let rising = |x: i64| x >= 0 && at(x + 2) >= 0.0 && at(x) < 0.0;
    let falling = |x: i64| x >= 0 && at(x + 2) <= 0.0 && at(x) > 0.0;
    let last_crossing = |pred: &dyn Fn(i64) -> bool| -> i64 {
        for cnt in 0..TAIL_SEARCH {
            let x = last - 2 - cnt;
            if x < 0 {
                break;
            }
            if pred(x) {
                return x + 2;
            }
        }
        0
    };
    let a = last_crossing(&rising);
    let b = last_crossing(&falling);
    let use_rising = a > b;
    let pred = |x: i64| if use_rising { rising(x) } else { falling(x) };
    let d0 = if use_rising { a } else { b };

    let mut delay = [0i64; TAIL_SEGMENTS];
    let mut source = [0i64; TAIL_SEGMENTS];
    let mut gain = [0f64; TAIL_SEGMENTS];
    delay[0] = d0;
    for r in 0..TAIL_SEGMENTS {
        let base = last - TAIL_SEARCH + TAIL_STEP * r as i64;
        let width = TAIL_SEARCH - TAIL_STEP * r as i64;
        let Some(found) = (0..width).map(|k| base + k).find(|&x| pred(x - 2)) else {
            continue;
        };
        source[r] = found;
        if r + 1 < TAIL_SEGMENTS {
            delay[r + 1] = d0 + delay[r] - found;
            if delay[r + 1] > m as i64 {
                delay[r + 1] = m as i64;
            }
        }
        let (mut num, mut den) = (0.0, 0.0);
        for k in 0..(d0 - found).max(0) {
            den += at(found + k).abs();
            let idx = delay[r] + k;
            if idx < m as i64 {
                num += at(idx).abs();
            }
        }
        gain[r] = if den == 0.0 { 0.0 } else { num / den };
    }
    Tail {
        d0: d0.max(0) as usize,
        delay,
        source,
        gain,
    }
}

/// Convert mono samples at `rate` (already trimmed) into the slot values.
pub fn convert_samples(x: &[f64], rate: u32, lead_trim: u32) -> Vec<f32> {
    let y = resample_to_96k(x, rate);
    let m = y.len();
    let y = normalise(&y);
    let mut c = halfband(&y);
    let taps = m.min(MAX_TAPS);
    sparsify(&mut c, taps);
    let t = tail_model(&c, taps, m);

    let mut values = Vec::with_capacity(NUM_TAPS);
    let mut positions = Vec::with_capacity(NUM_TAPS);
    let ps = POS_SCALE as f32;
    for (i, &v) in c.iter().enumerate().take(taps.min(t.d0)) {
        if v != 0.0 {
            values.push(v as f32);
            positions.push(ps * i as f32);
        }
    }
    values.resize(NUM_TAPS, 0.0);
    positions.resize(NUM_TAPS, (POS_SCALE * MAX_TAPS as f64) as f32);
    let mut out = Vec::with_capacity(NUM_COEFS);
    out.extend_from_slice(&values[..NUM_TAPS]);
    out.extend_from_slice(&positions[..NUM_TAPS]);
    out.extend(
        t.delay
            .iter()
            .map(|&d| if d > 2 { (d - 2) as f32 } else { 1.0 }),
    );
    out.extend(
        t.source
            .iter()
            .map(|&s| ((s as f32) as f64 * POS_SCALE) as f32),
    );
    out.extend(t.gain.iter().map(|&g| g as f32));
    out.push(lead_trim as f32);
    out.push(1.0);
    debug_assert_eq!(out.len(), NUM_COEFS);
    out
}

/// WAV file bytes → the slot's 3354 values. `channel` 0 = left, 1 = right
/// (ignored for mono files).
pub fn convert_wav(data: &[u8], channel: usize) -> Result<Vec<f32>, WavError> {
    let wav = read_wav(data)?;
    let (x, trim) = trim_and_select(&wav, channel);
    if x.is_empty() || x.iter().all(|&v| v == 0.0) {
        return Err(WavError::Silent);
    }
    Ok(convert_samples(&x, wav.rate, trim))
}

/// Each value as its 32-bit pattern in 8 nibbles, high nibble first.
pub fn to_nibbles(values: &[f32]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|v| {
            let w = v.to_bits();
            (0..8).rev().map(move |i| ((w >> (4 * i)) & 0xF) as u8)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav16(rate: u32, ch: u16, frames: &[i16]) -> Vec<u8> {
        let data: Vec<u8> = frames.iter().flat_map(|s| s.to_le_bytes()).collect();
        let mut v = Vec::new();
        v.extend(b"RIFF");
        v.extend((36 + data.len() as u32).to_le_bytes());
        v.extend(b"WAVEfmt ");
        v.extend(16u32.to_le_bytes());
        v.extend(1u16.to_le_bytes());
        v.extend(ch.to_le_bytes());
        v.extend(rate.to_le_bytes());
        v.extend((rate * 2 * ch as u32).to_le_bytes());
        v.extend((2 * ch).to_le_bytes());
        v.extend(16u16.to_le_bytes());
        v.extend(b"data");
        v.extend((data.len() as u32).to_le_bytes());
        v.extend(data);
        v
    }

    #[test]
    fn rejects_non_wav_and_silence() {
        assert_eq!(convert_wav(b"hello", 0).unwrap_err(), WavError::NotWav);
        let silent = wav16(96000, 1, &[0; 100]);
        assert_eq!(convert_wav(&silent, 0).unwrap_err(), WavError::Silent);
    }

    #[test]
    fn layout_of_a_decaying_impulse() {
        let frames: Vec<i16> = (0..9600)
            .map(|i| {
                let t = i as f64;
                ((t * 0.37).sin() * (-t / 900.0).exp() * 20000.0) as i16
            })
            .collect();
        let v = convert_wav(&wav16(96000, 1, &frames), 0).unwrap();
        assert_eq!(v.len(), NUM_COEFS);
        assert_eq!(v[NUM_COEFS - 1], 1.0);
        assert_eq!(v[NUM_COEFS - 2], 1.0); // the first sample, sin(0) = 0, is trimmed
        // Tap positions are even multiples of 2^-14 (odd taps are zero).
        for p in &v[NUM_TAPS..2 * NUM_TAPS] {
            let i = (p * 16384.0).round() as i64;
            assert!(i % 2 == 0 && (0..=4080).contains(&i));
        }
    }

    #[test]
    fn nibbles_high_first() {
        assert_eq!(to_nibbles(&[1.0]), vec![3, 15, 8, 0, 0, 0, 0, 0]);
    }
}
