use std::f32::consts::PI;

pub enum Waveform {
    /// Sinusoidal LFO wave form will smoothly oscillate between 0-3 octaves above PARAMS.frequency
    Sinusoidal,
    /// Square LFO wave form instantaneously jumps between an unaffected carrier signal and 3 octaves above PARAMS.frequency
    Square,
}

pub struct RingModParams {
    /// LFO section
    /// 0 to 10, this is normalized and controls a percentage of a 3 octave jump
    pub amount: f32,
    /// the waveform for the carrier LFO modulation
    pub lfo_waveform: Waveform,
    /// 0.1Hz to 25Hz the rate of the LFO modulation on the carrier signal
    pub rate: f32,

    /// Modulator section
    /// 0 to 100, mix with the original sampled signal
    pub mix: u8,
    /// 0.6Hz to 80Hz (LO setting), 30Hz to 4kHz (HI setting) for the carrier signal
    pub frequency: f32,
}

pub fn ring_mod(
    // the sample rate of the signal
    sample_rate: u32,
    // the total number of samples
    sample_length: usize,
    // the signal to apply the effect to
    signal: impl AsRef<[i32]>,
    // parameters for the ring mod
    params: &RingModParams,
) -> Vec<i32> {
    let mut res = vec![];

    // normalized mix and amount parameter
    let mix = f32::from(params.mix) / 100.0;
    let amount = params.amount / 10.0;

    // the signal
    let mut signal_iter = signal.as_ref().iter();

    let mut lfo_phase = 0.0;
    let mut carrier_phase = 0.0;

    let lfo_increment = 2.0 * PI * params.rate / sample_rate as f32;

    for _ in 0..sample_length {
        lfo_phase = (lfo_phase + lfo_increment).rem_euclid(2.0 * PI);

        let lfo = match params.lfo_waveform {
            Waveform::Sinusoidal => lfo_phase.sin(),
            Waveform::Square => {
                if lfo_phase.sin() >= 0.0 {
                    1.0
                } else {
                    0.0
                }
            }
        };

        // the carrier signal that's applied to the sampled one
        let carrier_increment =
            2.0 * PI * (params.frequency + lfo * (params.frequency * 3.0 * amount))
                / sample_rate as f32;

        carrier_phase = (carrier_phase + carrier_increment).rem_euclid(2.0 * PI);

        let carrier = carrier_phase.sin();

        if let Some(sample) = signal_iter.next() {
            let sample = *sample as f32;

            // accounted for the mix parameter
            // see https://en.wikipedia.org/wiki/Ring_modulation#Simplified_operation
            let out_sample = (sample * (1.0 - mix)) + (sample * carrier * mix);

            res.push(out_sample as i32);
        } else {
            println!("Signal processing may be incomplete");
            break;
        }
    }

    res
}
