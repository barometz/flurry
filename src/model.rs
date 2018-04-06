// The state of the art

use std::f64;
use uom::si::f64::*;
use uom::si::length::meter;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
use uom::si::frequency::hertz;

type RotationalVelocity = Frequency;
#[allow(non_camel_case_types)]
type per_second = hertz;

pub mod math {
    pub use graphics::math::*;
    use std::f64;
    use std::ops::Mul;
    use uom::si::f64::*;
    use vecmath;

    pub fn normalized(v: Vec2d<Length>) -> Vec2d<f64> {
        let v: Vec2d = [v[0].value, v[1].value];
        if v[0] == 0.0 && v[1] == 0.0 {
            v
        } else {
            vecmath::vec2_normalized(v)
        }
    }

    pub fn normalized_rad(r: f64) -> f64 {
        let circle = 2.0 * f64::consts::PI;
        let mut r = r % circle;
        if r < 0.0 {
            r += circle;
        }

        r
    }

    pub fn mul_scalar<T, U, V>(t: Vec2d<T>, u: U) -> Vec2d<V>
    where
        T: Copy + Mul<U, Output = V>,
        U: Copy,
    {
        [t[0] * u, t[1] * u]
    }
}

#[derive(Default, Debug)]
pub struct Flier {
    pub position: math::Vec2d<Length>,
    pub rotation: math::Matrix2d,
    pub target: math::Vec2d<Length>,

    top_speed: Velocity,
    top_rotational_speed: RotationalVelocity,
}

impl Flier {
    pub fn new(top_speed: Velocity, top_rotational_speed: RotationalVelocity) -> Flier {
        Flier {
            top_speed,
            top_rotational_speed,
            ..Default::default()
        }
    }

    pub fn progress(&mut self, dt: Time) {

        let to_target = math::sub(self.target, self.position);

        let direction = math::normalized(to_target);
        let travel_distance: Length = self.top_speed * dt;
        let actual_distance = to_target[0].hypot(to_target[1]);

        let distance =
            if actual_distance < travel_distance {
                actual_distance
            } else {
                travel_distance
            };

        let dp = math::mul_scalar(direction, distance);


        self.position = math::add(self.position, dp);
        self.rotation = math::orient(dp[0].value, dp[1].value);
    }
}

#[derive(Default, Debug)]
pub struct Model {
    pub flier: Flier,
}

impl Model {
    pub fn new() -> Model {
        Model {
            flier: Flier::new(
                Velocity::new::<meter_per_second>(60.0),
                RotationalVelocity::new::<per_second>(0.0),
            ),
        }
    }

    pub fn set_center_mass(&mut self, x: f64, y: f64) {
        self.flier.target = [Length::new::<meter>(x), Length::new::<meter>(y)];
    }

    pub fn progress(&mut self, dt: f64) {
        self.flier.progress(Time::new::<second>(dt));
    }
}
