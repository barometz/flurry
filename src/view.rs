// Takes a controller and presents it on the screen

use graphics::types::Color;
use graphics::character::CharacterCache;
use graphics::{Context, Graphics};

pub struct View {

}

impl View {
    pub fn draw<G: Graphics>(
        &self,
        game: &super::Model,
        c: &Context,
        g: &mut G
    )
    {

    }
}
