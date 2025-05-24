pub enum FilterType {
    Lowpass,
    Highpass,
}

pub struct Filter {
    sample_rate: u32,
    cutoff: f32,
    quality_factor: f32,
    filter_type: FilterType,
}

impl Filter {
    fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            cutoff: 5_000.0,
            quality_factor: 0.8,
            filter_type: FilterType::Highpass,
        }
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    fn sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    fn get_cutoff(&self) -> f32 {
        self.cutoff
    }

    fn set_cutoff(&mut self, cutoff: f32) {
        self.cutoff = cutoff;
    }

    fn cutoff(mut self, cutoff: f32) -> Self {
        self.cutoff = cutoff;
        self
    }

    fn get_quality_factor(&self) -> f32 {
        self.quality_factor
    }

    fn set_quality_factor(&mut self, quality_factor: f32) {
        self.quality_factor = quality_factor;
    }

    fn quality_factor(mut self, quality_factor: f32) -> Self {
        self.quality_factor = quality_factor;
        self
    }

    fn get_filter_type(&self) -> &FilterType {
        &self.filter_type
    }

    fn set_filter_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
    }

    fn filter_type(mut self, filter_type: FilterType) -> Self {
        self.filter_type = filter_type;
        self
    }

    /// processes an entire signal through this filter
    fn process_signal(signal: impl IntoIterator<Item = i32>) -> Vec<i32> {
        todo!()
    }
}
