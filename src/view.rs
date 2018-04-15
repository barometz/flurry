//! Takes a controller and presents it on the screen

use graphics::{Context, Graphics};
use graphics::math;
use graphics::types::{Color, Polygon};
use graphics;

use piston::input::RenderArgs;

use controller::Controller;

#[derive(Default)]
pub struct View {
    dimensions: math::Vec2d,
}

impl View {
    pub fn new() -> View {
        Default::default()
    }

    pub fn update(&mut self, e: &RenderArgs) {
        self.dimensions = [f64::from(e.draw_width), f64::from(e.draw_height)];
    }

    pub fn draw<G: Graphics>(&self, controller: &Controller, c: &Context, g: &mut G) {
        use graphics::Transformed;

        let poly: Polygon = &[[-7.5, -5.0], [-7.5, 5.0], [7.5, 0.0]];
        const RED: Color = [1.0, 0.0, 0.0, 1.0];

        for flier in &controller.game.fliers {
            let transform = self.get_world_transform(controller, c)
                .trans(flier.position[0].value, flier.position[1].value)
                .append_transform(flier.rotation);
            graphics::polygon(RED, poly, transform, g);
        }
    }

    fn get_world_transform(&self, controller: &Controller, ctx: &Context) -> math::Matrix2d {
        use graphics::Transformed;

        let screen_center = controller.get_screen_center();
        ctx.transform.trans(screen_center[0], screen_center[1])
    }
}
