use std::thread;
use std::sync::mpsc;
use std::time::{Instant};
use rand::Rng;


fn main() {
    let n = 100000000;
    let multi = true;

    // one thread -- 99 seconds
    if !multi {
      let now = Instant::now();
      let inside = num_inside(&n);
      println!("{}", 4.0*(inside as f64)/(n as f64));
      println!("pi estimated with {} samples in {} seconds", n, now.elapsed().as_secs());
    }

    // multiple threads -- 20 seconds
    if multi {
      let num_threads = 8;
      let (tx, rx) = mpsc::channel::<i32>();
      let now = Instant::now();
      let samples_per_thread = n/num_threads;
      for t in 0..num_threads {
        let cloned_tx = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
          let inside = num_inside(&samples_per_thread);
          println!("Thread {} done", t);
          let _ = cloned_tx.send(inside);
        });
      }

      let mut total_inside = 0;
      let mut completed_threads = 0;
      for received in rx {
        total_inside += received;
        completed_threads += 1;
        if completed_threads == num_threads {
          break;
        }
      }
      println!("{}",4.0*(total_inside as f64)/(n as f64));
      println!("pi estimated with {} samples in {} seconds", n, now.elapsed().as_secs());
    }
}

fn num_inside(samples: &i32) -> i32 {
  let mut rng = rand::thread_rng();
  let mut inside = 0;
  for _ in 0..*samples {
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    if x*x+y*y <= 1.0 {
      inside += 1;
    }
  }
  inside
}

fn single_thread_solve(n: &i32) {
  let mut rng = rand::thread_rng();
  let now = Instant::now();
  let v: Vec<f64> = (0..*n).map(|_| rng.gen_range(9000.0..100000.0)).collect();
  let vec_create_time = now.elapsed().as_secs();
  let mut sum = 0.0;
  // create time 53 seconds with n=100m
  println!("created random vec in {} seconds", vec_create_time);
  
  for i in v {
    sum = sum + i.sqrt();
  }
  let solve_time = now.elapsed().as_secs()-vec_create_time;
  // solve time 0 second n=100m
  println!("{} sqrts calculated in {} seconds", n, solve_time);
}