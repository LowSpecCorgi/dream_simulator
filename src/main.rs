use rand::prelude::*;
use std::process;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::{thread, time};
use threadpool::ThreadPool;
use simple_logger::SimpleLogger;
use log;
use text_io::read;

const BARTER_ATTEMPTS: i32 = 262;
const ENDER_PEARL_CHANCE: f32 = 0.0473;
const PEARLS_NEEDED: i32 = 41;
const BLAZE_KILLS: i32 = 305;
const BLAZE_ROD_CHANCE: f32=0.5;
const BLAZE_RODS_NEEDED: i32=211;

fn main() {

    SimpleLogger::new().init().unwrap();

    let tries = Arc::new(AtomicU64::new(0));
    let max_pearls = Arc::new(AtomicU64::new(0));
    let max_rods = Arc::new(AtomicU64::new(0));
    let pause_simulation = Arc::new(AtomicBool::new(false));

    println!(r" 
    Please enter the amount of threads you want to use, the more threads the more lag inducing, but faster, for reference I have a rtx2060 (GPU) and a Ryzen 5 2600 (cpu),
    with 16gb ram, and using 50 threads for this did lag my pc considerably, so just use that as a baseline:");

    let n_jobs = read!();

    println!(r"Press S + Enter to pause the Simulation. Starting in 5s");
    
    thread::sleep(time::Duration::from_secs(5));

    let thread_pool = ThreadPool::new(n_jobs);

    for _ in 0..n_jobs {
        let tries_clone = Arc::clone(&tries);
        let max_pearls_clone = Arc::clone(&max_pearls);
        let max_rods_clone = Arc::clone(&max_rods);
	let pause_clone = Arc::clone(&pause_simulation);

        thread_pool.execute(move || loop {
            
            let mut thread_rng = rand::thread_rng();
            let successful: (i32,i32) = run_attempts(
	        tries_clone.load(Ordering::Relaxed) as u64,
                BARTER_ATTEMPTS,
                ENDER_PEARL_CHANCE,
                PEARLS_NEEDED,
                BLAZE_KILLS,
                BLAZE_ROD_CHANCE,
                BLAZE_RODS_NEEDED,
                &mut thread_rng,
            );
	    let max_pearls_temp = max_pearls_clone.load(Ordering::Relaxed) as i32 ;
	    let max_rods_temp = max_rods_clone.load(Ordering::Relaxed) as i32;
            if successful.0 >= max_pearls_temp && successful.0 + successful.1 >= max_pearls_temp + max_rods_temp
	       {
                max_pearls_clone.store(successful.0 as u64, Ordering::SeqCst);
                max_rods_clone.store(successful.1 as u64, Ordering::SeqCst);
              }

            if tries_clone.load(Ordering::Relaxed) % 10000000 == 0 {
                log::info!(
                    "[{:?}] [{} Iterations] | Successful this iteration: {} {} | Didn't achieve Dream level luck :( | Max Combined Pearls: {} Rods: {} ",
                    thread::current().id(),
                    english_numbers::convert_all_fmt(
                        tries_clone.load(Ordering::Relaxed) as i64
                    ),
                    successful.0,
                    successful.1,
                    max_pearls_clone.load(Ordering::SeqCst),
                    max_rods_clone.load(Ordering::SeqCst)
                );
            }
	    if pause_clone.load(Ordering::Relaxed){
	      log::info!("[{:?}] [{} Iterations] | Thread paused",
	        thread::current().id(),
		tries_clone.load(Ordering::Relaxed) as i64);
              while pause_clone.load(Ordering::Relaxed){
	        thread::sleep(time::Duration::from_secs(1));
	      };
	      log::info!("[{:?}] [{} Iterations] | Thread resumed",
	        thread::current().id(),
		tries_clone.load(Ordering::Relaxed) as i64);
            };
           tries_clone.fetch_add(1, Ordering::SeqCst);
        });
    }
    loop{
      let input: String = read!();
      let code = input.as_str();
      
      pause_simulation.store (code == "s" || code == "S",Ordering::Relaxed);
      if code == "s" || code == "S" {
        thread::sleep(time::Duration::from_secs(1));
	log::info!("Press any Key + Enter to continue!");
      }
    }
    
    let barrier = Arc::new(Barrier::new(n_jobs + 1));
    
    barrier.wait();
    
}

fn run_attempts(
    tries: u64;
    barter_attempts: i32,
    ender_pearl_chance: f32,
    pearls_needed: i32,
    blaze_kills: i32,
    blaze_rod_chance: f32,
    rods_needed: i32,
    thread_rng: &mut ThreadRng,
) -> (i32,i32) {
    let mut successful: (i32,i32) = (0,0);
    for _n in 0..barter_attempts {
        let result: f32 = thread_rng.gen();
        if result <= ender_pearl_chance {
            successful.0 += 1;
        }
    }
    for _n in 0..blaze_kills {
        let result: f32 = thread_rng.gen();
        if result <= blaze_rod_chance {
            successful.1 += 1;
        }
    }
    if successful.0 >= pearls_needed {
        if successful.1 >= rods_needed {
            println!("~ Dream's luck was replicated after {} iterations! ~ ({}, {})", tries, successful.0, successful.1);
            process::exit(0);
        }
    }


    successful
}
