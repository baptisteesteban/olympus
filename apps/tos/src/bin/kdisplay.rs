use pixels::{Pixels, SurfaceTexture};
use std::error::Error;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(WIDTH as u16, HEIGHT as u16);
        WindowBuilder::new()
            .with_title("KDisplay")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let size = window.inner_size();
        let texture = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(size.width, size.height, texture).unwrap()
    };
    pixels.frame_mut().iter_mut().for_each(|pixel| {
        *pixel = 255;
    });

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: _,
            } => elwt.exit(),
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::RedrawRequested,
            } => {
                pixels.render().unwrap();
            }
            _ => (),
        })
        .unwrap();

    Ok(())
}
