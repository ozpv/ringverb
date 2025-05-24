use std::iter::repeat_n;

pub struct DelayParams {
    /// mix 0 to 100
    pub mix: u8,
    /// Delay in milliseconds
    pub delay: f32,
    /// feedback in percent
    pub feedback: f32,
    /// the number of all-pass filters applied to the signal
    pub stages: u32,
    /// the width of the resulting stereo signal
    pub width: f32,
    /// high-pass filter after the reverb
    pub high_pass: f32,
    /// low-pass filter after the reverb
    pub low_pass: f32,
}

fn all_pass_filter(signal: impl AsRef<[i32]>, buffer_len: usize, feedback: f32) -> Vec<i32> {
    let signal = signal.as_ref();

    // normalize the feedback parameter
    let feedback = feedback / 100.0;

    let mut delay_line = vec![0.0; buffer_len];
    let mut res = vec![0; signal.len()];

    for (i, sample) in signal.iter().enumerate() {
        let sample = *sample as f32;
        let delayed = delay_line[i % buffer_len];
        res[i] = (-feedback * sample + delayed) as i32;
        delay_line[i % buffer_len] = sample + feedback * res[i] as f32;
    }

    res
}

pub fn diffusion_delay(
    sample_rate: u32,
    signal: impl AsRef<[i32]>,
    params: &DelayParams,
) -> Vec<i32> {
    // calculated based on delay length
    let buffer_length = (params.delay * sample_rate as f32 / 1_000.0) as i32;

    // allow for trails
    let mut res = signal.as_ref().to_vec();
    res.extend(repeat_n(0, (5 * sample_rate) as usize));

    for i in 0..params.stages {
        res = all_pass_filter(
            res,
            (buffer_length / (i as i32 + 1)) as usize,
            params.feedback,
        );
    }

    res
}
