mod tower;
mod game;

use pixels::{Pixels, SurfaceTexture};

use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
};

use winit::event::{ElementState, MouseButton};
use crate::game::Game;
use crate::tower::{Position, Tower};

const INITIAL_WIDTH: u32 = 640;
const INITIAL_HEIGHT: u32 = 360;

fn main() {
    // Create the window event loop
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Tower Defense")
        .with_inner_size(LogicalSize::new(INITIAL_WIDTH, INITIAL_HEIGHT))
        .with_min_inner_size(LogicalSize::new(100, 100))
        .build(&event_loop)
        .unwrap();

    // Create the pixel buffer for the window
    let mut pixels = {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, surface_texture).unwrap()
    };

    // Animation timer
    let mut cursor_position = (0.0, 0.0);

    let mut game = Game::new();

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. }, ..
            } => {
                cursor_position = (position.x, position.y);
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { button, state, .. }, ..
            } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    let size = window.inner_size();

                    let x_percent = (cursor_position.0 / size.width as f64) * 100.0;
                    let y_percent = (cursor_position.1 / size.height as f64) * 100.0;
                    
                    let tower = Tower::new_archer(Position::new(x_percent as usize, y_percent as usize));
                    game.add_tower(tower);
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::RedrawRequested => {
                    let size = window.inner_size();

                    let frame = pixels.frame_mut();

                    game.render(frame, size.width, size.height);

                    //Render the frame to the window
                    if pixels.render().is_err() {
                        elwt.exit();
                        return;
                    }
                }
                WindowEvent::CloseRequested => elwt.exit(),

                WindowEvent::Resized(size) => {
                    // Resize the pixel buffer to match the new window size
                    pixels.resize_surface(size.width, size.height).unwrap();
                    pixels.resize_buffer(size.width, size.height).unwrap();
                }

                _ => {}
            },

            Event::AboutToWait => {
                // Request a redraw to animate
                window.request_redraw();
            }

            _ => {}
        }
    }).unwrap();
}
