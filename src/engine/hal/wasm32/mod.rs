use super::super::{DISPLAY_HEIGHT, DISPLAY_WIDTH, FrameBuffer, InputsState};
use core::f32::consts::E;
use gloo::events::EventListener;
use gloo::utils::document;
use gloo_timers::future::sleep;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::JsCast;
use web_sys::window;

pub struct HAL {
    canvas_context: web_sys::CanvasRenderingContext2d,
    scaling_factor: u32,
    pub inputs: Rc<RefCell<InputsState>>,
}

impl HAL {
    pub fn new() -> Self {
        let canvas = document()
            .get_element_by_id("game-canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let scaling_factor = Self::calculate_scaling_factor();

        canvas.set_height(DISPLAY_HEIGHT as u32 * scaling_factor);
        canvas.set_width(DISPLAY_WIDTH as u32 * scaling_factor);

        let canvas_context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let inputs = InputsState {
            ..Default::default()
        };

        let mut s = Self {
            canvas_context,
            scaling_factor,
            inputs: Rc::new(RefCell::new(inputs)),
        };

        s.setup_inputs();

        s
    }

    fn setup_inputs(&mut self) {
        // Callback function, runs when
        let keyboard_event_callback =
            |event: &web_sys::Event, inputs: &Rc<RefCell<InputsState>>, is_keydown: bool| {
                let keyboard_event: web_sys::KeyboardEvent = event.clone().dyn_into().unwrap();
                match keyboard_event.code().as_str() {
                    "KeyW" => inputs.borrow_mut().up = is_keydown,
                    "KeyS" => inputs.borrow_mut().down = is_keydown,
                    "KeyA" => inputs.borrow_mut().left = is_keydown,
                    "KeyD" => inputs.borrow_mut().right = is_keydown,

                    "ArrowUp" => inputs.borrow_mut().up = is_keydown,
                    "ArrowDown" => inputs.borrow_mut().down = is_keydown,
                    "ArrowLeft" => inputs.borrow_mut().left = is_keydown,
                    "ArrowRight" => inputs.borrow_mut().right = is_keydown,

                    "Space" => inputs.borrow_mut().jump = is_keydown,

                    _ => {}
                }
            };

        {
            let inputs = self.inputs.clone();
            EventListener::new(&window().unwrap(), "keydown", move |event| {
                keyboard_event_callback(event, &inputs, true);
            })
            .forget();
        }

        {
            let inputs = self.inputs.clone();
            EventListener::new(&window().unwrap(), "keyup", move |event| {
                keyboard_event_callback(event, &inputs, false);
            })
            .forget();
        }
    }

    // Calculate integer scaling factor (for responsiveness so that it isn't absolutely massive on small screens)
    fn calculate_scaling_factor() -> u32 {
        let window_height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;
        let window_width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;

        let x_border = 40u32; // Just an estimate, I can't be asked to figure out how to do this properly
        let y_border = 200u32; // Same thing here, I'm sure there's a better way to do this

        let window_height = core::cmp::max(window_height - y_border, 0); // Clamp at 0 to prevent edge cases if your screen is 30px wide or something
        let window_width = core::cmp::max(window_width - x_border, 0);

        let y_scaling_factor: u32 = window_height / (DISPLAY_HEIGHT as u32);
        let x_scaling_factor: u32 = window_width / (DISPLAY_WIDTH as u32);

        core::cmp::max(core::cmp::min(x_scaling_factor, y_scaling_factor), 1)
    }

    // Returns a microsecond timestamp
    pub fn micros(&self) -> u64 {
        // It's pretty janky but good enough, I don't see any other API for this
        (window().unwrap().performance().unwrap().now() * 1000.0f64) as u64
    }

    // Delay for a number of milliseconds
    pub async fn delay_ms(self: &mut Self, ms: u32) {
        sleep(Duration::from_millis(ms as u64)).await;
    }

    // Delay for a number of microseconds (converts to milliseconds, doesn't actually delay for microseconds)
    pub async fn delay_us(self: &mut Self, us: u32) {
        sleep(Duration::from_millis(
            ((us as f32) / 1000.0f32).round() as u64
        ))
        .await;
    }

    pub async fn delay_until_us(&mut self, until: u64) {
        let current_timestamp = self.micros();
        self.delay_us((until - current_timestamp) as u32).await;
    }

    pub fn display_buffer(&mut self, framebuffer: &FrameBuffer) {
        // Here's a hack that'll probably save one or two CPU cycles:
        let states = ["#09021d", "#dddeff"];

        for x in 0..DISPLAY_WIDTH as usize {
            for y in 0..DISPLAY_HEIGHT as usize {
                let state = framebuffer.get_pixel_state(x, y);

                // Here's the other half of the hack:
                self.canvas_context
                    .set_fill_style_str(states[state as usize]);

                self.canvas_context.fill_rect(
                    x as f64 * self.scaling_factor as f64,
                    y as f64 * self.scaling_factor as f64,
                    self.scaling_factor as f64,
                    self.scaling_factor as f64,
                );
            }
        }
    }
}
