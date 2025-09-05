use crate::engine::alloc::Vec;

use crate::engine::{Sound, SoundTone, tones::Tone};

pub fn soundtrack() -> Sound {
    Vec::from([
        SoundTone {
            freq: Tone::C4.to_freq(),
            length_us: 200_000,
        },
        SoundTone {
            freq: Tone::Rest.to_freq(),
            length_us: 200_000,
        },
        SoundTone {
            freq: Tone::E4.to_freq(),
            length_us: 200_000,
        },
        SoundTone {
            freq: Tone::F4.to_freq(),
            length_us: 200_000,
        },
        SoundTone {
            freq: Tone::G4.to_freq(),
            length_us: 200_000,
        },
        SoundTone {
            freq: Tone::A4.to_freq(),
            length_us: 200_000,
        },
        SoundTone {
            freq: Tone::B4.to_freq(),
            length_us: 200_000,
        },
        SoundTone {
            freq: Tone::C2.to_freq(),
            length_us: 200_000,
        },
    ])
}
