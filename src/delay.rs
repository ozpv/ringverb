use std::iter::repeat_n;
use std::ops::Range;

pub struct DelayParams {
    /// 0 to 100
    pub mix: u8,
    /// Delay in milliseconds
    pub delay: f32,
    pub feedback: f32,
    pub width: f32,
    pub diffusion: u32,
    pub diffusion_size: f32,
    pub high_pass: f32,
    pub low_pass: f32,
}

fn all_pass_filter(signal: impl AsRef<[i32]>, buffer_len: usize, feedback: f32) -> Vec<i32> {
    let signal = signal.as_ref();

    // normalize the feedback parameter
    let feedback = feedback / 100.0;

    let mut delay_line = vec![0.0; buffer_len];
    let mut res = vec![0; signal.len()];

    for (i, _) in signal.iter().enumerate() {
        let delayed = delay_line[i % buffer_len];
        res[i] = (-feedback * signal[i] as f32 + delayed) as i32;
        delay_line[i % buffer_len] = signal[i] as f32 + feedback * res[i] as f32;
    }

	res
}

#[inline]
fn rand_range(range: Range<f32>) -> f32 {
    range.start + (range.end - range.start) * ((getrandom::u32().unwrap() % 100) as f32 / 100.0)
}

pub fn schroeder_all_pass(
    // the sample rate of the signal
    sample_rate: u32,
    // the signal to apply the effect to
    signal: impl AsRef<[i32]>,
    // the number of stages of the all-pass filter
    stages: u32,
    // the base delay for each stage
    base_delay: f32,
    // the feedback
    feedback: f32,
) -> Vec<i32> {
    // fill the delay stages with random values
    let mut delay_stages = vec![0.0; stages as usize];
    for stage in &mut delay_stages {
        *stage = rand_range(0.9 * base_delay..1.5 * base_delay);
    }

    // clone signal and push 5 seconds of zeros for reverb trails
    let mut res = Vec::from(signal.as_ref());
    res.extend(repeat_n(0, (5 * sample_rate) as usize));

    for delay in delay_stages {
        let delay_samples = (delay * sample_rate as f32 / 1_000.0) as usize;
        res = all_pass_filter(res, delay_samples, feedback);
    }

    res
}

pub fn diffusion_delay(
    // the sample rate of the signal
    sample_rate: u32,
    // the signal to apply the effect to
    signal: impl AsRef<[i32]>,
    // parameters for the delay
    params: &DelayParams,
) -> Vec<i32> {
    // calculated based on delay length
    let buffer_length = (params.delay * sample_rate as f32 / 1_000.0) as i32;

    let mut res = Vec::from(signal.as_ref());
    res.extend(repeat_n(0, (5 * sample_rate) as usize));

    for i in 0..params.diffusion {
        res = all_pass_filter(
            res,
            (buffer_length / (2 + i as i32)) as usize,
            params.feedback,
        );
    }

    res
}
