use log::error;
use pixels::Error;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod front;
mod pokemon;

use front::Front;

const WINDOW_WIDTH: u32 = 10;
const WINDOW_HEIGHT: u32 = 5;

fn main() -> Result<(), Error> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize {
            width: 800,
            height: 600,
        };
        WindowBuilder::new()
            .with_title("Pokemon automata")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut front = Front::new(&window)?;

    event_loop.run(move |event, _, control_flow| {
        //Draw frame
        if let Event::RedrawRequested(_) = event {
            if let Err(err) = front.draw() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = front.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            front.update();
            window.request_redraw();
        }
    });
}
