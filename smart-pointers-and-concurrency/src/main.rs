use std::thread;
use std::sync::mpsc;
use std::time::{Instant};
use rand::Rng;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};


fn main() {
    let n = 100000000;
    let generate = false;
    let multi = true;


    // single_thread_solve(&n);

    if generate {
      let res = generate_data(n);
      match res {
        Ok(()) => println!("file successfully generated"),
        Err(e) => println!("{}", e),
      }
    }

    // one thread -- 45 seconds
    if !multi {
      let now = Instant::now();
      let res = single_thread_solve_with_file(&0, &n);
      match res {
        Ok(sum) => {
          println!("sum: {}", sum);
        },
        Err(e) => println!("Error: {}", e),
      }
      println!("{} sqrts calculated in {} seconds", n, now.elapsed().as_secs());
    }


    // multiple threads -- 51 seconds
    if multi {
      let num_threads = 8;
      let (tx, rx) = mpsc::channel::<f64>();
      let now = Instant::now();
      let lines_per_thread = n/num_threads;
      for t in 0..num_threads {
        let cloned_tx = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
          let res = single_thread_solve_with_file(&(lines_per_thread*t), &(lines_per_thread*(t+1)));
          match res {
            Ok(sum) => {
              println!("Thread {} done", t);
              let _ = cloned_tx.send(sum);
            },
            Err(e) => println!("Error on thread {}: {}", t, e),
          }
        });
      }

      let mut sum = 0.0;
      let mut completed_threads = 0;
      for received in rx {
        sum += received;
        println!("new sum: {}", sum);
        completed_threads += 1;
        if completed_threads == num_threads {
          break;
        }
      }
      println!("{} sqrts calculated in {} seconds", n, now.elapsed().as_secs());
    }
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

fn single_thread_solve_with_file(start_line: &i32, end_line: &i32) -> std::io::Result<f64> {
  let file = File::open("/Users/logan/Desktop/school/NextGen/smart-pointers-and-concurrency/smart-pointers-and-concurrency/data/data.txt")?;
  let buffer = io::BufReader::new(file);
  let mut start_reading = false;
  let mut sum = 0.0;
  for (index, line) in buffer.lines().enumerate() {
    if index == *end_line as usize {
      break;
    }
    if index == *start_line as usize {
      start_reading = true;
    }
    if start_reading {
      let f: f64 = line?.parse().unwrap();
      sum += f;
    }
  }
  Ok(sum)
}


fn generate_data(n: i32) -> std::io::Result<()> {
  let mut rng = rand::thread_rng();
  let v: Vec<f64> = (0..n).map(|_| rng.gen_range(9000.0..100000.0)).collect();
  let mut file = File::create("/Users/logan/Desktop/school/NextGen/smart-pointers-and-concurrency/smart-pointers-and-concurrency/data/data.txt")?;
  for i in v {
    writeln!(&mut file, "{}", i)?;
  }
  Ok(())
}
