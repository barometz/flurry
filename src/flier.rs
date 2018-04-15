//! A thing that flies.

use math;
pub use uom::si::f64::{Length, Frequency, Time, Velocity};
pub use uom::si::length::meter;
pub use uom::si::time::second;
pub use uom::si::velocity::meter_per_second;
pub use uom::si::frequency::hertz;

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