pub fn downsample(sample_rate: u32, signal: impl AsRef<[i32]>, new_sample_rate: u32) -> Vec<i32> {
    if sample_rate < new_sample_rate || new_sample_rate == 0 {
        return vec![];
    }

    // TODO: lowpass filter

    let signal = signal.as_ref();
    let resample_factor = (sample_rate / new_sample_rate) as usize;

    signal.iter().copied().step_by(resample_factor).collect()
}
