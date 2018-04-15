#![deny(missing_docs)]

//! Entry point for flurry.

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

extern crate flurry;

use flurry::model::Model;
use flurry::view::View;
use flurry::controller::Controller;

use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;

fn main() {
    let opengl = opengl_graphics::OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("flurry", [640, 480])
        .opengl(opengl)
        .exit_on_esc(true)
        .samples(2)
        .srgb(false)
        .build()
        .unwrap();

    let model = Model::new();
    let mut controller = Controller::new(model);
    let view = View::new();

    let mut gl = opengl_graphics::GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        controller.event(&e);

        e.render(|r| {
            gl.draw(r.viewport(), |ctx, gfx| {
                graphics::clear([1.0; 4], gfx);
                view.draw(&controller, &ctx, gfx);
            })
        });
    }
}
