extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;

mod model;
mod view;
mod controller;

pub use model::Model;
pub use view::View;
pub use controller::Controller;

use glutin_window::GlutinWindow;
use piston::window::{Window, WindowSettings};
use piston::event_loop::{Events, EventSettings};
use piston::input::RenderEvent;

fn main() {
    let opengl = opengl_graphics::OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
        "flurry",
        [200, 200]
    )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut controller = Controller {
        game: Model {}
    };

    let mut view = View {};

    let mut gl = opengl_graphics::GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        controller.event(&e);

        if let Some(e) = e.render_args() {
            gl.draw(e.viewport(), |ctx, gfx| {
                graphics::clear([1.0; 4], gfx);
                view.draw(&controller.game, &ctx, gfx);
            })
        }
    }
}
