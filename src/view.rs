// Takes a controller and presents it on the screen

use graphics::{Context, Graphics};
use graphics::math;
use graphics::types;
use graphics;

use piston::input::RenderArgs;

pub struct View {
    dimensions: math::Vec2d,
}

impl View {
    pub fn new() -> View {
        View {
            dimensions: [0.0, 0.0],
        }
    }

    pub fn update(&mut self, e: &RenderArgs) {
        self.dimensions = [e.draw_width as f64, e.draw_height as f64];
    }

    pub fn draw<G: Graphics>(&self, controller: &super::Controller, c: &Context, g: &mut G) {
        use graphics::Transformed;

        let poly: types::Polygon = &[[-5.0, -7.5], [5.0, -7.5], [0.0, 7.5]];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let transform = self.get_world_transform(controller, c)
            .trans(controller.game.position[0], controller.game.position[1])
            .rot_rad(controller.game.rotation);
        graphics::polygon(RED, poly, transform, g);
    }

    fn get_world_transform(&self, controller: &super::Controller, ctx: &Context) -> math::Matrix2d {
        use graphics::Transformed;

        let screen_center = controller.get_screen_center();
        ctx.transform.trans(screen_center[0], screen_center[1])
    }
}
