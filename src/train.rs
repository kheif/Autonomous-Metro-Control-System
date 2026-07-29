#[derive(Clone, Copy)]
pub enum TrainState {
    Stopped,
    Accelerating {
        acceleration: f64,
        target_speed: f64,
    },
    Cruising,
    Braking {
        brake_force: f64,
    },
    Emergency,
}

pub struct Train {
    speed: f64,
    max_brake_force: f64,
    pos: f64,
    weight: f64,
    state: TrainState,
}

impl Train {
    pub fn get_speed(&self) -> f64 {
        self.speed
    }

    pub fn get_pos(&self) -> f64 {
        self.pos
    }

    pub fn advance(&mut self, dt: f64) {
        match self.state {
            TrainState::Stopped => {}
            TrainState::Accelerating {
                acceleration,
                target_speed,
            } => {
                let calculated_speed = self.speed + acceleration * dt;
                let end_speed = if target_speed > calculated_speed {
                    calculated_speed
                } else {
                    self.state = TrainState::Cruising;
                    target_speed
                };
                self.pos += Self::traveled_distance(self.speed, end_speed, dt);
                self.speed = end_speed;
            }
            TrainState::Cruising => {
                self.pos += self.speed * dt;
            }
            TrainState::Braking { brake_force } => Self::apply_braking(self, brake_force, dt),
            TrainState::Emergency => Self::apply_braking(self, self.max_brake_force, dt),
        };
    }

    fn apply_braking(&mut self, brake_force: f64, dt: f64) {
        let end_speed = f64::max(self.speed - (brake_force / self.weight) * dt, 0f64);
        self.pos += Self::traveled_distance(self.speed, end_speed, dt);
        self.speed = end_speed;
        if end_speed == 0f64 {
            self.state = TrainState::Stopped;
        }
    }

    fn traveled_distance(first_speed: f64, end_speed: f64, dt: f64) -> f64 {
        (first_speed + end_speed) / 2f64 * dt
    }

    pub fn new(
        speed: f64,
        max_brake_force: f64,
        pos: f64,
        weight: f64,
        state: TrainState,
    ) -> Train {
        Train {
            speed,
            max_brake_force,
            pos,
            weight,
            state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speed_never_negative_while_braking() {
        let mut train = Train::new(
            10.0,
            50000.0,
            0.0,
            1000.0,
            TrainState::Braking {
                brake_force: 5000.0,
            },
        );

        for _ in 0..10 {
            train.advance(1.0);
            assert!(
                train.get_speed() >= 0.0,
                "speed went negative: {}",
                train.get_speed()
            );
        }
    }
}
