use std::{thread, time::Duration};

fn main() {
    let mut threads = Vec::new();
    for i in 0..10_000u32 {
        let t = thread::spawn(move || {
            let bad_hash = i.wrapping_mul(2654435761) % 200_000;
            thread::sleep(Duration::from_micros(bad_hash as u64));
            for _ in 0..1000 {
                thread::sleep(Duration::from_millis(10));
            }
        });
        threads.push(t);
    }

    for t in threads {
        t.join().unwrap()
    }
}
