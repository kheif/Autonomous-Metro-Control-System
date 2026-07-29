use crate::train::Train;

mod train;

fn main() {
    let mut train = train::Train::new(
        0f64,
        5000f64,
        0f64,
        5000f64,
        train::TrainState::Accelerating {
            acceleration: 30f64,
            target_speed: 180f64,
        },
    );

    run_simulation(&mut train, 5, 1f64);
}

fn run_simulation(train: &mut Train, steps: u32, dt: f64) {
    for i in 0..steps {
        train.advance(dt);
        println!(
            "{}s: Train speed:{} - Position: {}",
            i,
            train.get_speed(),
            train.get_pos()
        );
    }
}
