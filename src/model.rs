// The state of the art

pub mod math {
    use vecmath;
    pub type Position = vecmath::Vector2<f64>;
}

pub struct Model {
    pub position: math::Position,
    pub rotation: f64,
}

impl Model {
    pub fn new() -> Model {
        Model {
            position: [0.0, 0.0],
            rotation: 0.0,
        }
    }

    pub fn set_center_mass(&mut self, x: f64, y: f64) {
        self.position = [x, y];
    }

    pub fn progress(&mut self, dt: f64) {
        self.rotation += 2.0 * dt;
    }
}
