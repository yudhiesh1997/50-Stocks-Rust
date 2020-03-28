extern crate rand; 
extern crate threadpool;

use rand::Rng;
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;
use std::sync::mpsc;
use std::format;

// Create a program which simulates 50 stocks being updated. 
// Each stock is monitored (incremented) by a thread, updating a listener thread of any change.
// The listener thread receives the update and prints the value of the stock before waiting for the next update.

fn main() {
    // Generate 50 random stock names with random values 
    
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const STOCK_LENGTH: usize = 3;
    let mut rng = rand::thread_rng();
    let n_worker = 4;
    let n_job = 50;
    let pool1 = ThreadPool::new(n_worker);
    let (r_s, r_r) = mpsc::channel();
    let r_clone = r_s.clone();

    for _ in 0..50 {
        pool1.execute(move|| {
            let stock_name: String = (0..STOCK_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
        let mut stock_price = rng.gen_range(1.0,50.0);
        let stock = 
        r_clone.send(stock_name).unwrap();
        r_s.send(stock_price.to_string()).unwrap()        
        })
    }

    //Create a threapool 
    //Increment each stock with a random value 
    //Sleep for a random period 
    let n_workers = 4;
    let n_jobs = 50;
    let pool = ThreadPool::new(n_workers);
    

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    for _ in 0..n_jobs{
        pool.execute(move|| {
            loop {
                let stock_variables = r_r.recv().unwrap();
                let mut sleeptime = rng.gen_range(1,25);
                let mut random_stock_fluctuation = rng.gen_range(0.1,0.5);
                sleeptime = sleeptime * 100;
                stock_price = stock_price + random_stock_fluctuation;
                let stock = format!("{} {}", stock_name, stock_price);
                tx1.send(stock).expect("Failed to send data");
                thread::sleep(Duration::from_millis(sleeptime));
            }
        });
    }
    loop {
        let stock = rx.recv().unwrap();
        println!("{} : {:.2}" ,stock_name, stock_price);
    }





}




// let n_workers = 4;
// let n_jobs = 8;
// let pool = ThreadPool::new(n_workers);

// let (tx,rx) = channel();

// for _ in 0..n_jobs {
//     let tx = tx.clone();
//     pool.execute(move|| {
//         tx.send(1).expect("channel will be there waiting for the pool");

//     });
// }
// println!("{}", rx.recv().unwrap());
// //xassert_eq!(rx.iter().take(n_jobs).fold(0, |a, b| a + b), 8);
