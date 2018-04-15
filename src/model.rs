//! The state of the art

use std::f64;
use flier::*;

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
