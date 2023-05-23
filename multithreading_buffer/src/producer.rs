use std::thread::sleep;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use crate::sensor_data::SensorData;

pub fn start_producer(running: Arc<AtomicBool>, shared_data: &Arc<Mutex<Vec<Vec<u8>>>>){
    let mut sensor_data: SensorData = SensorData::new();
    let interval = Duration::from_secs(1);
    let mut next_time = Instant::now() + interval;
    let mut counter=0;

    while running.load(Ordering::SeqCst) {
        read_sensors(&mut sensor_data, counter);
        //println!("I'm the producer; helo!");
        write_on_shared_data(&mut sensor_data, shared_data);
        sleep(next_time - Instant::now());
        next_time += interval;
        counter+=1;
    }
}

fn read_sensors(sensor_data: &mut SensorData, seq: u32){
    sensor_data.set_seq(seq);
    sensor_data.set_values(&sensor_data.get_values().map(|value| value+0.5));
    sensor_data.set_timestamp(seq+1);
}

fn write_on_shared_data(sensor_data: &SensorData, shared_data: &Arc<Mutex<Vec<Vec<u8>>>>){
    let struct_as_byte: Vec<u8> = sensor_data.to_bytes();

    let mut write_data_lock = shared_data.lock().unwrap();
    write_data_lock.push(struct_as_byte);
    println!("Producer>> new record written on shared data!");
}