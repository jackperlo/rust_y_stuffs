use std::thread::sleep;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use crate::sensor_data::SensorData;

pub fn start_consumer(running: Arc<AtomicBool>, shared_data: &Arc<Mutex<Vec<Vec<u8>>>>){
    let mut sensor_data: Vec<SensorData>;
    let interval = Duration::from_secs(10);
    let mut next_time = Instant::now() + interval;

    while running.load(Ordering::SeqCst) {
        //println!("I'm the consumer; helo!");
        sensor_data = read_from_shared_data(shared_data);
        println!("Consumer>> {sensor_data:?}");
        sleep(next_time - Instant::now());
        next_time += interval;
    }
}

fn read_from_shared_data(shared_data: &Arc<Mutex<Vec<Vec<u8>>>>) -> Vec<SensorData>{
    let read_data_lock = shared_data.lock().unwrap();
    let mut collected_data: Vec<SensorData> = vec![];

    let mut index=0;
    while index < read_data_lock.len() {
        let n_bytes=0;
        collected_data.push(SensorData::from_bytes(&read_data_lock[index][n_bytes..n_bytes+(4+40+4)]));
        index+=1;
    }

    collected_data
}