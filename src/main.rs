use std::{
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    let tick_rate = Duration::from_millis(600);
    let mut next_tick = Instant::now();

    loop {
        game_loop();

        next_tick += tick_rate;

        let now = Instant::now();
        if now < next_tick {
            sleep(next_tick - now);
        } else {
            eprintln!("tick lagging behind by {:?}", now - next_tick);
            next_tick = now;
        }
    }
}

fn game_loop() {
    println!("tick");
}
