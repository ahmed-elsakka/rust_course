mod sensor;

use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use std::{thread, vec};
use std::sync::{Arc, Mutex};

use sensor::start_sensor;

fn main() {
    let sensors = vec!["Tempertature", "Humidity", "Pressure"];
    let data = Arc::new(Mutex::new(HashMap::new()));

    for sensor in &sensors {
        let sensor_data =
         Arc::clone(&data);
        let sensor_name = sensor.to_string();

        thread::spawn(|| start_sensor(sensor_name,
         sensor_data));
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        let data_lock = data.lock().unwrap();

        println!("\n--- Sensor Data ---");
        for (key, val) in data_lock.iter() {
            println!("{}: {}", key, val);
        }
    }
}
