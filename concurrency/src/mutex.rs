use std::sync::Mutex;

pub fn mutex() {
    // Mutex<T> is a smart pointer
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
