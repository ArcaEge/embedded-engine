mod display;
mod hal;

#[cfg(target_arch = "arm")]
use defmt::debug;

#[cfg(target_arch = "wasm32")]
use log::debug;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

use display::{Buffer, DISPLAY_HEIGHT, DISPLAY_PAGE_COUNT, DISPLAY_WIDTH, FrameBuffer};
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
pub struct EngineInteractionLayer<'a> {
    hal: &'a HAL,
    pub framebuffer: &'a mut FrameBuffer,
}

impl<'a> EngineInteractionLayer<'a> {
    pub fn set_pixel_state(&mut self, x: usize, y: usize, state: bool) {
        self.framebuffer.set_pixel_state(x, y, state);
    }

    pub fn get_pixel_state(&self, x: usize, y: usize) {
        self.framebuffer.get_pixel_state(x, y);
    }
}

// Game engine, responsible for initialisation, ticks, rendering
pub struct Engine<T: GameTrait> {
    game: T,
    hal: HAL,
    framebuffer: FrameBuffer,
}

impl<T: GameTrait> Engine<T> {
    pub fn new() -> Self {
        let hal = HAL::new();
        Self {
            game: T::new(),
            hal: hal,
            framebuffer: FrameBuffer::new(),
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

        let mut interaction_layer = EngineInteractionLayer {
            hal: &mut self.hal,
            framebuffer: &mut self.framebuffer,
        };

        self.game.tick(tick_count, &mut interaction_layer);

        // Write to display
        self.framebuffer.show(&mut self.hal);
    }
}
