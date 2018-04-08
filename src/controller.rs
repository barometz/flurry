//! Handles events and modifies the game state

use piston::input::GenericEvent;
use model;
use graphics::math;

pub struct Controller {
    pub game: model::Model,
    mouse_position: math::Vec2d,
    screen_dimensions: math::Vec2d,
}

impl Controller {
    pub fn new(game: model::Model) -> Controller {
        Controller {
            game,
            mouse_position: [0.0, 0.0],
            screen_dimensions: [200.0, 200.0],
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, MouseButton};

        e.update(|update| self.game.progress(update.dt));
        e.mouse_cursor(|x, y| self.mouse_position = [x, y]);
        e.press(|press| {
            if let Button::Mouse(MouseButton::Left) = press {
                let repositioned = math::sub(self.mouse_position, self.get_screen_center());
                self.game.set_center_mass(repositioned[0], repositioned[1])
            }
        });
        e.render(|r| self.screen_dimensions = [f64::from(r.draw_width), f64::from(r.draw_height)]);
    }

    pub fn get_screen_center(&self) -> math::Vec2d {
        math::mul_scalar(self.screen_dimensions, 0.5)
    }
}
