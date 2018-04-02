// Handles events and modifies the game state

use piston::input::GenericEvent;
use model;

pub struct Controller {
    pub game: super::Model,
}

impl Controller {
    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}