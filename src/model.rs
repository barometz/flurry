//! The state of the art

use std::f64;
use uom::si::f64::*;
use uom::si::length::meter;
use uom::si::time::second;
use uom::si::velocity::meter_per_second;
use uom::si::frequency::hertz;

use math;

/// Rotational velocity is described in rad/s, or s<sup>-1</sup>
pub type RotationalVelocity = Frequency;
#[allow(non_camel_case_types)]
/// Type alias for the unit of rotational velocity
pub type per_second = hertz;

#[derive(Default, Debug)]
/// A thing that flies.
pub struct Flier {
    // Current position of the flier in world space
    pub position: math::Vec2d<Length>,
    /// Current rotation in world space. Stored as Matrix2d only because that's easier to math on
    /// than a number of radians.
    pub rotation: math::Matrix2d,
    /// The "target" the flier will travel towards.
    pub target: math::Vec2d<Length>,

    top_speed: Velocity,
    top_rotational_speed: RotationalVelocity,
}

impl Flier {
    pub fn new(top_speed: Velocity, top_rotational_speed: RotationalVelocity) -> Flier {
        Flier {
            top_speed,
            top_rotational_speed,
            rotation: math::orient(0.0, 1.0),
            ..Default::default()
        }
    }

    /// Progress the position and rotation of the flier by the given amount of time.
    pub fn progress(&mut self, dt: Time) {
        let to_target = math::sub(self.target, self.position);

        let direction = math::normalized(to_target);
        let travel_distance: Length = self.top_speed * dt;
        let actual_distance = to_target[0].hypot(to_target[1]);

        let distance = if actual_distance < travel_distance {
            actual_distance
        } else {
            travel_distance
        };

        let dp = math::mul_scalar(direction, distance);

        self.position = math::add(self.position, dp);
        if !(dp[0].value == 0.0 && dp[1].value == 0.0) {
            self.rotation = math::orient(dp[0].value, dp[1].value);
        }
    }
}

#[derive(Default, Debug)]
pub struct Model {
    pub fliers: Vec<Flier>,
}

impl Model {
    pub fn new() -> Model {
        let s = vec![100.0, 50.0, 80.0];
        let r = vec![0.0, 0.0, 0.0];
        let mut fliers: Vec<Flier> = Vec::new();

        for (s, r) in s.into_iter().zip(r) {
            fliers.push(Flier::new(
                Velocity::new::<meter_per_second>(s),
                RotationalVelocity::new::<per_second>(r),
            ));
        }

        Model { fliers }
    }

    pub fn add_flier(&mut self, x: f64, y: f64, top_speed: f64) {
        let mut flier = Flier::new(
            Velocity::new::<meter_per_second>(top_speed),
            RotationalVelocity::new::<per_second>(0.0),
        );
        flier.position = [Length::new::<meter>(x), Length::new::<meter>(y)];
        flier.target = flier.position;
        self.fliers.push(flier);
    }

    pub fn set_center_mass(&mut self, x: f64, y: f64) {
        for flier in &mut self.fliers {
            flier.target = [Length::new::<meter>(x), Length::new::<meter>(y)];
        }
    }

    pub fn progress(&mut self, dt: f64) {
        for flier in &mut self.fliers {
            flier.progress(Time::new::<second>(dt));
        }
    }
}
