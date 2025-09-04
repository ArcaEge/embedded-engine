use super::{EngineInteractionLayer, Sound};

/// Plays sounds
pub struct SoundPlayer {
    sound: Sound,
    current_index: u32,
    micros_past_current_index: u64,
    last_timestamp: Option<u64>,
    finished: bool,
    pub repeat: bool,
}

impl SoundPlayer {
    pub fn new(sound: Sound) -> Self {
        Self {
            sound,
            current_index: 0,
            micros_past_current_index: 0,
            last_timestamp: None,
            finished: false,
            repeat: false,
        }
    }

    pub fn reset(&mut self, engine: &mut EngineInteractionLayer) {
        self.micros_past_current_index = 0;
        self.current_index = 0;
        self.last_timestamp = None;
        self.finished = false;

        engine.set_sound_state(false);
    }

    /// Run this when pausing instead of just ceasing to call play_tick. Calls engine.set_sound_state(false) automatically
    pub fn pause(&mut self, engine: &mut EngineInteractionLayer) {
        self.last_timestamp = None;

        engine.set_sound_state(false);
    }

    /// Tick, but for sound player. Updates the PWM state and returns true if finished
    pub fn play_tick(&mut self, engine: &mut EngineInteractionLayer) -> bool {
        if self.current_index >= self.sound.len() as u32 {
            self.reset(engine);

            if !self.repeat {
                self.finished = true;
            }

            return true;
        }

        if self.finished {
            return true;
        }

        let micros = engine.micros();

        if let Some(last_timestamp) = self.last_timestamp {
            // Using i128 to prevent overflow if micros has reached the upper half of u64
            // who knows maybe the program will be run a few billion years
            let mut timestamp_difference = micros as i128 - last_timestamp as i128;

            let mut should_reset = true;

            // Loop to find the correct sound
            for sound_no in self.current_index..(self.sound.len() as u32) {
                let mut length = self.sound[sound_no as usize].length_us;

                if sound_no == self.current_index {
                    // Not sure when this if statement would be necessary but it prevents an underflow if something weird happens
                    if self.micros_past_current_index < length {
                        length -= self.micros_past_current_index;
                    } else {
                        length = 0;
                    }
                }

                timestamp_difference -= length as i128;

                // This is the tone to play
                if timestamp_difference < 0 {
                    if sound_no == self.current_index {
                        self.micros_past_current_index +=
                            (timestamp_difference + length as i128) as u64;
                    } else {
                        // Don't do a += if switching to a new tone, overwrite it entirely otherwise it just skips through the ones after
                        // the first note, which is not good
                        self.micros_past_current_index =
                            (timestamp_difference + length as i128) as u64;
                    }
                    self.current_index = sound_no;
                    engine.set_sound_freq(self.sound[sound_no as usize].freq);

                    should_reset = false;
                    break;
                }
            }

            // Reached the end of the sound
            if should_reset {
                self.reset(engine);

                if !self.repeat {
                    self.finished = true;
                }
                return true;
            }
        } else {
            engine.set_sound_freq(self.sound[self.current_index as usize].freq);
        }

        self.last_timestamp = Some(micros);
        false
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }
}
