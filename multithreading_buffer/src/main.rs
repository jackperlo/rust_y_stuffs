mod producer;
mod consumer;
mod sensor_data;

use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use ctrlc;
use crate::consumer::start_consumer;
use crate::producer::start_producer;

fn main() {
    /*Shared Data management*/
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    let producer_data = shared_data.clone(); //returns a MutexGuard<T>.
    let consumer_data = shared_data.clone();

    /*CTRL-C handler management*/
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let producer_running = running.clone();
    let consumer_running = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    /*Threads Start*/
    let producer_thread = thread::spawn(move || {start_producer(producer_running, &producer_data);});
    let consumer_thread = thread::spawn(move || {start_consumer(consumer_running, &consumer_data);});

    /*Waiting for Thread End*/
    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
    println!("Producer Thread terminated correctly!");
    println!("Consumer Thread terminated correctly!");
}
