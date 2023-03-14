
use std::{sync::Arc, cell::RefCell};

use parking_lot::ReentrantMutex;

//use parking_lot::RwLock;
//use parking_lot::deadlock;


// use std::sync::RwLock;


struct LockedData {
    data: u64
}


fn main() {

    
    let mut data = LockedData { data: 0 };
    //let mut data_2 = LockedData { data: 0 };

    let lock = Arc::new( ReentrantMutex::new(RefCell::new(data)));
    // let lock2 = Arc::new(RwLock::new(data_2));

    let lock_t1 = lock.clone();
    // let lock2_t1 = lock2.clone();
    std::thread::spawn(move || {
        {
            println!("lock 1");
            let  lock_guard = lock_t1.lock();
            let mut locked = lock_guard.borrow_mut();
            std::thread::sleep(std::time::Duration::from_secs(1));
            locked.data += 1;
            
            std::mem::drop(locked);
            std::mem::drop(lock_guard);
            let lock_guard_2 = lock_t1.lock();
            
            println!("lock 2");
            let mut locked = lock_guard_2.borrow_mut();
            std::thread::sleep(std::time::Duration::from_secs(1));
            locked.data += 1;
            println!("done");
        }
    });

    while true {
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        print!(".");
        // let deadlocks = deadlock::check_deadlock();
        
        // for deadlock in deadlocks {
        //     println!("Deadlock detected: Threads:");
        //     for thread in deadlock {
        //         println!(" - Thread Id {}, -", thread.thread_id());
        //         let backtrace = thread.backtrace();
                
        //         for frame in backtrace.frames() {
        //             for symbol in frame.symbols().iter() {

        //                 let filename = symbol.filename().map_or("unknown", |p | p.to_str().unwrap_or("unknown"));
        //                 let lineNumber = symbol.lineno().unwrap_or_default();

        //                 println!("{}:{}", filename, lineNumber);
        //             }
        //         }
        //     }
        // }

        println!("data {}", lock.lock().borrow().data);
        
    }

}
