// The state of the art

use uom::si::f64::*;
use uom::si::length::meter;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;

pub mod math {
    pub use graphics::math::*;
    use uom::si::Quantity;
    use uom::si::f64::*;
    use uom::si::length::meter;

    pub fn normalized(v: Vec2d<Length>) -> Vec2d<f64> {
        if (v[0].value == 0.0) && (v[1].value == 0.0) {
            let result: Vec2d<f64> = [0.0, 0.0];
            return result;
        }

        let v: Vec2d = [v[0].value, v[1].value];

        let length = v[0] * v[0] + v[1] * v[1];
        let length = length.sqrt();
        let result: Vec2d<f64> = [v[0] / length, v[1] / length];
        result
    }
}

#[derive(Default)]
pub struct Model {
    pub position: math::Vec2d<Length>,
    pub target: math::Vec2d<Length>,
    pub linear_velocity: Velocity,
    pub rotational_velocity: f64,
    pub rotation: f64,
}

impl Model {
    pub fn new() -> Model {
        Model {
            rotational_velocity: 0.0,
            linear_velocity: Velocity::new::<meter_per_second>(60.0),
            ..Default::default()
        }
    }

    pub fn set_center_mass(&mut self, x: f64, y: f64) {
        self.target = [Length::new::<meter>(x), Length::new::<meter>(y)];
    }

    pub fn progress(&mut self, dt: f64) {
        let dt = Time::new::<second>(dt);

        let direction = math::sub(self.target, self.position);
        let direction = math::normalized(direction);
        let dp: Length = self.linear_velocity * dt;
        let dp = [dp * direction[0], dp * direction[1]];

        self.position = math::add(self.position, dp);
        self.rotation += self.rotational_velocity * dt.value;
    }
}
