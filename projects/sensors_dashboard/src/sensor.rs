use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn start_sensor(name: String, shared_data: Arc<Mutex<HashMap<String, f64>>>) {
    let mut rng = rand::rng();

    loop {
        let value = match name.as_str() {
            "Temperature" => rng.random_range(20.0..30.0),
            "Humidity" => rng.random_range(30.0..70.0),
            "Pressure" => rng.random_range(1000.0..1020.0),
            _ => rng.random_range(0.0..100.0)
        };

        {
            let mut data_lock = shared_data.lock().unwrap();
            data_lock.insert(name.clone(), value);
        }

        thread::sleep(Duration::from_secs(1));
    }
}