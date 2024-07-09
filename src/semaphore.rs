use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    count: Mutex<i32>, //临界资源的数目，相当于(同时访问该资源的最大线程数)。Mutex是互斥锁，用于保护临界资源的访问
    condvar: Condvar,  //条件变量，用于线程间的通信
}

impl Semaphore {
    pub fn new(count: i32) -> Self {
        Self {
            count: Mutex::new(count),
            condvar: Condvar::new(),
        }
    }

    pub fn wait(&self) {
        let mut _count = self.count.lock().unwrap();
        while *_count <= 0 {
            _count = self.condvar.wait(_count).unwrap(); // 阻塞，等待条件变量的通知，同时释放锁，遵循“让权等待”原则。得到通知后，重新获取锁，继续执行
        }
        *_count -= 1;
    }

    pub fn signal(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        if *count > 0 {
            self.condvar.notify_all(); // 通知所有等待的线程
        }
    }

    pub fn pseudowait(&self) {
        let mut count = self.count.lock().unwrap();
        while *count <= 0 {
            count = self.condvar.wait(count).unwrap(); // 阻塞，等待条件变量的通知，同时释放锁，遵循“让权等待”原则。得到通知后，重新获取锁，继续执行
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_count() {
        // 测试不同临界资源数目的情况
        for count in 1..10 {
            test_semaphore(count);
        }
    }

    fn test_semaphore(count: i32) {
        use rand::distributions::{Distribution, Uniform};

        let between = Uniform::from(10..20); // 随机等待时间
        let mut rng = rand::thread_rng();

        let semaphore = Arc::new(Semaphore::new(count));
        let mut handles = vec![];

        for i in 0..10 {
            let time = between.sample(&mut rng);
            let semaphore = Arc::clone(&semaphore);
            handles.push(thread::spawn(move || {
                semaphore.wait();
                println!("Thread {} is running", i);
                thread::sleep(std::time::Duration::from_millis(time));
                println!("Thread {} is done", i);
                semaphore.signal();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("All threads are done\n");
    }
}
