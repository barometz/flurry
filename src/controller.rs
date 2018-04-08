//! Handles events and modifies the game state

use piston::input::{GenericEvent, Key, MouseButton};
use model;
use graphics::math;

#[derive(Debug, Default)]
pub struct Controller {
    pub game: model::Model,
    mouse_position: math::Vec2d,
    screen_dimensions: math::Vec2d,
    speed: f64,
}

impl Controller {
    pub fn new(game: model::Model) -> Controller {
        Controller {
            game,
            screen_dimensions: [200.0, 200.0],
            speed: 1.0,
            ..Default::default()
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        e.update(|update| self.game.progress(update.dt * self.speed));
        e.mouse_cursor(|x, y| {
            // Only store non-zero position; 0,0 happens when the window gains focus
            if x != 0.0 || y != 0.0 {
                self.mouse_position = [x, y]
            }
        });
        e.press(|press| {
            use piston::input::Button;
            match press {
                Button::Mouse(button) => self.handle_mouse_event(button),
                Button::Keyboard(key) => self.handle_key_event(key),
                _ => (),
            }
        });

        e.render(|r| self.screen_dimensions = [f64::from(r.draw_width), f64::from(r.draw_height)]);
    }

    pub fn get_screen_center(&self) -> math::Vec2d {
        math::mul_scalar(self.screen_dimensions, 0.5)
    }

    fn handle_mouse_event(&mut self, button: MouseButton) {
        match button {
            MouseButton::Left => {
                let repositioned = math::sub(self.mouse_position, self.get_screen_center());
                self.game.set_center_mass(repositioned[0], repositioned[1]);
            }
            MouseButton::Right => {
                let repositioned = math::sub(self.mouse_position, self.get_screen_center());
                self.game.add_flier(repositioned[0], repositioned[1], 50.0);
            }
            _ => (),
        }
    }

    fn handle_key_event(&mut self, key: Key) {
        match key {
            Key::NumPadPlus | Key::Plus => self.speed *= 1.25,
            Key::NumPadMinus | Key::Minus => self.speed *= 0.8,
            _ => (),
        }
    }
}
