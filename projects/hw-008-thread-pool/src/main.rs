use hw_008_thread_pool::SizedThreadPool;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    fn task() {
        let duration = rand::thread_rng().gen_range(100..3000);
        thread::sleep(Duration::from_millis(duration));
    }

    let pool = SizedThreadPool::new(10).unwrap();

    for _ in 0..100 {
        pool.add_task(&task).unwrap();
    }
}
