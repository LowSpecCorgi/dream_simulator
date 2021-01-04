use rand::prelude::*;
use std::process;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;
use threadpool::ThreadPool;
use simple_logger::SimpleLogger;
use log::{info, trace, warn};

const BARTER_ATTEMPTS: i32 = 262;
const ENDER_PEARL_CHANCE: f32 = 0.0473;
const PEARLS_NEEDED: i32 = 41;

fn main() {

    SimpleLogger::new().init().unwrap();

    let tries = Arc::new(AtomicU64::new(0));
    let max = Arc::new(AtomicU64::new(0));

    let n_jobs = 50;

    let thread_pool = ThreadPool::new(n_jobs);

    let barrier = Arc::new(Barrier::new(n_jobs + 1));

    for _ in 0..n_jobs {
        let tries_clone = Arc::clone(&tries);
        let max_clone = Arc::clone(&max);

        thread_pool.execute(move || loop {
            
            let mut thread_rng = rand::thread_rng();
            let successful: i32 = run_attempts(
                BARTER_ATTEMPTS,
                ENDER_PEARL_CHANCE,
                PEARLS_NEEDED,
                &mut thread_rng,
            );
            
            if successful > max_clone.load(Ordering::Relaxed) as i32 {
                max_clone.store(successful as u64, Ordering::SeqCst);
            }

            if tries_clone.load(Ordering::Relaxed) % 10000000 == 0 {
                log::info!(
                    "[{:?}] [{} Iterations] | Successful this iteration: {} | Didn't acvieve Dream level luck :( | Max: {} |",
                    thread::current().id(),
                    english_numbers::convert_all_fmt(
                        tries_clone.load(Ordering::Relaxed) as i64
                    ),
                    successful,
                    max_clone.load(Ordering::SeqCst)
                );
            }
            tries_clone.fetch_add(1, Ordering::SeqCst);
        });
    }
    let barrier = Arc::new(Barrier::new(n_jobs + 1));
    barrier.wait();
    
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
