// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    /*Arc：允许多个线程共享同一个 JobStatus 实例。
    Mutex：提供互斥锁，确保只有一个线程可以同时访问或修改 JobStatus。 */
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
             // 使用 Mutex 锁定共享数据并更新
             /*lock()：获取 Mutex 的锁。如果另一个线程已经持有锁，当前线程会阻塞直到锁可用。
             unwrap()：解包 lock() 返回的 Result，如果锁定失败（例如死锁），程序会 panic。
             status.jobs_completed += 1：安全地更新共享数据。 */
            let mut status2 = status_shared.lock().unwrap();
            status2.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
