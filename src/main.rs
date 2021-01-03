use rand::prelude::*;
use rustplotlib::Figure;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

const BARTER_ATTEMPTS: i32 = 262;
const ENDER_PEARL_CHANCE: f32 = 0.0473;
const PEARLS_NEEDED: i32 = 41;

fn main() {
    let tries = Arc::new(AtomicU64::new(0));
    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(12)
        .build()
        .unwrap();

    let tries_clone = Arc::clone(&tries);
    let _handle = thread_pool.spawn(move || loop {
        let mut thread_rng = rand::thread_rng();
        let successful: i32 = run_attempts(
            BARTER_ATTEMPTS,
            ENDER_PEARL_CHANCE,
            PEARLS_NEEDED,
            &mut thread_rng,
        );

        if Arc::clone(&tries_clone).load(Ordering::Relaxed) % 1000000 == 0 {
            println!(
                "[{:?}]({} Iterations) | Successful this iteration: {} | Didn't acvieve Dream level luck :(",
                thread::current().id(),
                english_numbers::convert_all_fmt(
                    Arc::clone(&tries_clone).load(Ordering::Relaxed) as i64
                ),
                successful
            );
        }
        Arc::clone(&tries_clone).fetch_add(1, Ordering::SeqCst);
    });

    let mut thread_rng = rand::thread_rng();
    loop {
        let successful: i32 = run_attempts(
            BARTER_ATTEMPTS,
            ENDER_PEARL_CHANCE,
            PEARLS_NEEDED,
            &mut thread_rng,
        );

        if Arc::clone(&tries).load(Ordering::Relaxed) % 1000000 == 0 {
            println!(
                "[{:?}]({}) | Successful this iteration: {} | Didn't acvieve Dream level luck :(",
                thread::current().id(),
                english_numbers::convert_all_fmt(Arc::clone(&tries).load(Ordering::Relaxed) as i64),
                successful
            );
        }
        Arc::clone(&tries).fetch_add(1, Ordering::SeqCst);
    }
}

fn run_attempts(
    barter_attempts: i32,
    ender_pearl_chance: f32,
    pearls_needed: i32,
    thread_rng: &mut ThreadRng,
) -> i32 {
    let mut successful: i32 = 0;
    for _n in 0..barter_attempts {
        let result: f32 = thread_rng.gen();
        if result <= ender_pearl_chance {
            successful += 1;
            if successful >= pearls_needed {
                println!("~ Dream's luck was replicated! ~ ({})", successful);
                process::exit(0);
            }
        }
    }
    successful
}
