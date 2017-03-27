extern crate qutex;
extern crate futures;

use std::thread;
use futures::Future;
use qutex::Qutex;

fn main() {
    let thread_count = 20;
    let mut threads = Vec::with_capacity(thread_count);        
    let start_val = 0;
    let qutex = Qutex::new(start_val);

    for _ in 0..thread_count {
        let future_val = qutex.clone().request_lock();

        let future_add = future_val.and_then(|mut val| {
            *val += 1;
            Ok(())
        });

        threads.push(thread::spawn(|| {
            future_add.wait().unwrap();
        }));    
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let val = qutex.request_lock().wait().unwrap();
    assert_eq!(*val, start_val + thread_count);
    println!("Qutex final value: {}", *val);
}