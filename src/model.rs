// The state of the art

pub mod math {
    use vecmath;
    pub type Position = vecmath::Vector2<f64>;
}

#[derive(Default)]
pub struct Model {
    pub position: math::Position,
    pub rotation: f64,
}

impl Model {
    pub fn new() -> Model {
        Default::default()
    }

    pub fn set_center_mass(&mut self, x: f64, y: f64) {
        self.position = [x, y];
    }

    pub fn progress(&mut self, dt: f64) {
        self.rotation += 2.0 * dt;
    }
}
