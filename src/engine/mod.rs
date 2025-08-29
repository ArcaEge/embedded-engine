mod display;
mod hal;
mod interaction_layer;

#[cfg(target_arch = "arm")]
use defmt::debug;

#[cfg(target_arch = "wasm32")]
use log::debug;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[allow(unused)]
pub use display::{DISPLAY_HEIGHT, DISPLAY_PAGE_COUNT, DISPLAY_WIDTH, FrameBuffer};

pub use interaction_layer::EngineInteractionLayer;

use hal::HAL;
use iter_variants::IterVariants;
use variant_count::VariantCount;

pub trait GameTrait {
    /// Runs on Engine::new()
    fn new() -> Self;

    /// Runs once when start() is called
    fn init(&mut self, engine: &mut EngineInteractionLayer);

    /// Runs on every tick
    fn tick(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer);
}

/// Game engine, responsible for initialisation, ticks, rendering, input processing
pub struct Engine<T: GameTrait> {
    game: T,
    hal: HAL,
    framebuffer: FrameBuffer,
    inputs: [Input; Inputs::VARIANT_COUNT],
}

impl<T: GameTrait> Engine<T> {
    pub fn new() -> Self {
        let hal = HAL::new();
        Self {
            game: T::new(),
            hal: hal,
            framebuffer: FrameBuffer::new(),
            inputs: [Input::new(); Inputs::VARIANT_COUNT],
        }
    }

    #[cfg(target_arch = "arm")]
    pub fn start(mut self, tick_rate_hz: f32) -> !
    where
        Self: 'static,
    {
        debug!("Engine started!");
        let mut interaction_layer = EngineInteractionLayer {
            hal: &mut self.hal,
            framebuffer: &mut self.framebuffer,
            inputs: &self.inputs,
        };

        let tick_period: u32 = 1_000_000 / tick_rate_hz as u32;

        // Initialise the game
        self.game.init(&mut interaction_layer);

        let mut tick_count: u64 = 0;

        loop {
            let next_tick_time = self.hal.micros() + tick_period as u64;

            self.main_loop_contents(tick_count);

            // Increment tick counter
            tick_count += 1;

            self.hal.delay_until_us(next_tick_time);
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn start(mut self, tick_rate_hz: f32)
    where
        Self: 'static,
    {
        debug!("Engine started!");
        let mut interaction_layer = EngineInteractionLayer {
            hal: &mut self.hal,
            framebuffer: &mut self.framebuffer,
            inputs: &self.inputs,
        };

        let tick_period: u32 = 1_000_000 / tick_rate_hz as u32;

        // Initialise the game
        self.game.init(&mut interaction_layer);

        let mut tick_count: u64 = 0;

        spawn_local(async move {
            loop {
                let next_tick_time = self.hal.micros() + tick_period as u64;

                self.main_loop_contents(tick_count);

                // Increment tick counter
                tick_count += 1;

                self.hal.delay_until_us(next_tick_time).await;
            }
        });
    }

    fn main_loop_contents(&mut self, tick_count: u64) {
        // Clear framebuffer
        self.framebuffer.clear();

        self.process_inputs(tick_count);

        let mut interaction_layer = EngineInteractionLayer {
            hal: &mut self.hal,
            framebuffer: &mut self.framebuffer,
            inputs: &self.inputs,
        };

        self.game.tick(tick_count, &mut interaction_layer);

        // Write to display
        self.framebuffer.show(&mut self.hal);
    }

    /// Process all inputs
    fn process_inputs(&mut self, current_tick: u64) {
        self.hal.update_inputs();

        let inputs_state = self.hal.inputs.borrow().clone();

        Inputs::iter_variants(|input| {
            Self::process_input(
                &mut self.inputs[input as usize],
                inputs_state[input as usize],
                current_tick,
            );
        });
    }

    /// Process a single input
    fn process_input(input: &mut Input, new_state: bool, current_tick: u64) {
        if input.state != new_state {
            input.state = new_state;

            if new_state {
                input.pressed_tick = Some(current_tick);
            } else {
                input.released_tick = Some(current_tick);
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Input {
    pub state: bool,
    pub pressed_tick: Option<u64>,
    pub released_tick: Option<u64>,
}

/// A single input
impl Input {
    pub fn new() -> Self {
        Self {
            state: false,
            pressed_tick: None,
            released_tick: None,
        }
    }
}

/// A less stupid way of doing inputs (I think?), helps avoid code duplication
#[repr(usize)]
#[derive(VariantCount, IterVariants, Clone, Copy)]
pub enum Inputs {
    Up,
    Down,
    Left,
    Right,
    Jump,
}
