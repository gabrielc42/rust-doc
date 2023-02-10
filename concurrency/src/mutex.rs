use std::{
    //rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

pub fn mutex() {
    // Mutex<T> is a smart pointer
    // Mutex<T> provides interior mutability
    // similar to RefCell<T> in Smart Pointers (ch 15)
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
