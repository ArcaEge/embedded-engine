mod display;
mod hal;

#[cfg(target_arch = "arm")]
use defmt::debug;

#[cfg(target_arch = "wasm32")]
use log::debug;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[allow(unused)]
pub use display::{DISPLAY_HEIGHT, DISPLAY_PAGE_COUNT, DISPLAY_WIDTH, FrameBuffer};
use hal::HAL;

pub trait GameTrait {
    // Runs on Engine::new()
    fn new() -> Self;

    // Runs once when start() is called
    fn init(&mut self, engine: &mut EngineInteractionLayer);

    // Runs on every tick
    fn tick(&mut self, tick_count: u64, engine: &mut EngineInteractionLayer);
}

// Engine interaction layer (the functions the game can call and the objects it can access to interact with the engine)
#[allow(dead_code)]
pub struct EngineInteractionLayer<'a> {
    hal: &'a HAL,
    pub framebuffer: &'a mut FrameBuffer,
    pub inputs: &'a Inputs,
}

impl<'a> EngineInteractionLayer<'a> {
    pub fn set_pixel_state(&mut self, x: usize, y: usize, state: bool) {
        self.framebuffer.set_pixel_state(x, y, state);
    }

    pub fn set_pixel_state_check_bounds(&mut self, x: i32, y: i32, state: bool) -> Result<(), ()> {
        if x >= DISPLAY_WIDTH as i32 || x < 0 || y >= DISPLAY_HEIGHT as i32 || y < 0 {
            return Err(());
        }

        self.framebuffer
            .set_pixel_state(x as usize, y as usize, state);
        Ok(())
    }

    pub fn get_pixel_state(&self, x: usize, y: usize) {
        self.framebuffer.get_pixel_state(x, y);
    }
}

// Game engine, responsible for initialisation, ticks, rendering, input processing
pub struct Engine<T: GameTrait> {
    game: T,
    hal: HAL,
    framebuffer: FrameBuffer,
    inputs: Inputs,
}

impl<T: GameTrait> Engine<T> {
    pub fn new() -> Self {
        let hal = HAL::new();
        Self {
            game: T::new(),
            hal: hal,
            framebuffer: FrameBuffer::new(),
            inputs: Inputs {
                ..Default::default()
            },
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

    fn process_inputs(&mut self, current_tick: u64) {
        let inputs_state = self.hal.inputs.borrow().clone();

        // This is probably the worst possible way of going about this but oh well
        Self::process_input(&mut self.inputs.up, inputs_state.up, current_tick);
        Self::process_input(&mut self.inputs.down, inputs_state.down, current_tick);
        Self::process_input(&mut self.inputs.left, inputs_state.left, current_tick);
        Self::process_input(&mut self.inputs.right, inputs_state.right, current_tick);
        Self::process_input(&mut self.inputs.jump, inputs_state.jump, current_tick);
    }

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

#[derive(Clone)]
pub struct Input {
    pub state: bool,
    pub pressed_tick: Option<u64>,
    pub released_tick: Option<u64>,
}

#[derive(Clone)]
pub struct InputsState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    jump: bool,
}

#[derive(Clone)]
pub struct Inputs {
    pub up: Input,
    pub down: Input,
    pub left: Input,
    pub right: Input,
    pub jump: Input,
}

impl Default for Inputs {
    fn default() -> Self {
        let empty_input = Input {
            state: false,
            pressed_tick: None,
            released_tick: None,
        };

        Self {
            up: empty_input.clone(),
            down: empty_input.clone(),
            left: empty_input.clone(),
            right: empty_input.clone(),
            jump: empty_input.clone(),
        }
    }
}

impl Default for InputsState {
    fn default() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            jump: false,
        }
    }
}
