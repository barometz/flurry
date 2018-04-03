// Handles events and modifies the game state

use piston::input::GenericEvent;
use model;
use vecmath;

pub struct Controller {
    pub game: super::Model,
    mouse_position: vecmath::Vector2<f64>,
    screen_dimensions: vecmath::Vector2<f64>,
}

impl Controller {
    pub fn new(game: super::Model) -> Controller {
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
                let repositioned = vecmath::vec2_sub(self.mouse_position, self.get_screen_center());
                self.game.set_center_mass(repositioned[0], repositioned[1])
            }
        });
        e.render(|r| self.screen_dimensions = [r.draw_width as f64, r.draw_height as f64]);
    }

    pub fn get_screen_center(&self) -> vecmath::Vector2<f64> {
        vecmath::vec2_scale(self.screen_dimensions, 0.5)
    }
}
