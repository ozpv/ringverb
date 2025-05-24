#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::struct_field_names)]

pub mod delay;
pub mod filter;
pub mod resample;
pub mod ring_mod;

use delay::{DelayParams, diffusion_delay};
use resample::downsample;
use ring_mod::{RingModParams, Waveform, ring_mod};

use hound::{WavReader, WavWriter};

const RING_MOD_PARAMS: RingModParams = RingModParams {
    mix: 50,
    frequency: 156.0,
    amount: 6.7,
    lfo_waveform: Waveform::Square,
    rate: 0.18,
};

const DELAY_PARAMS: DelayParams = DelayParams {
    mix: 100,
    delay: 519.0,
    feedback: 90.0,
    stages: 20,
    width: 100.0,
    high_pass: 10.0,
    low_pass: 2_580.0,
};

fn main() {
    let r = WavReader::open("guitar.wav").unwrap();
    let mut w = WavWriter::create("output.wav", r.spec().sample_rate(44_100)).unwrap();

    // total number of samples in the input file "guitar.wav"
    let len = r.len();
    let sample_rate = r.spec().sample_rate;

    let signal = r
        .into_samples()
        .map(|sample| sample.expect("Failed to open signal as an array"))
        .collect::<Vec<i32>>();

    let result = ring_mod(sample_rate, len as usize, signal, &RING_MOD_PARAMS);
    let result = diffusion_delay(sample_rate, result, &DELAY_PARAMS);
    //let result = downsample(sample_rate, result, 8_000);

    for sample in result {
        w.write_sample(sample).unwrap();
    }
}
