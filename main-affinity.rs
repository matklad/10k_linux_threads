use std::{thread, time::Duration};

fn main() {
    let arg = std::env::args().nth(1);
    let pin_to_core = arg.as_deref() == Some("--pin-to-core");

    let core_ids = core_affinity::get_core_ids().unwrap();

    let threads: Vec<_> = (0..10_000u32)
        .zip(core_ids.iter().cycle().copied())
        .map(|(i, core_id)| {
            thread::spawn(move || {
                if pin_to_core {
                    core_affinity::set_for_current(core_id);
                }
                let bad_hash = i.wrapping_mul(2654435761) % 200_000;
                thread::sleep(Duration::from_micros(bad_hash as u64));
                for _ in 0..1000 {
                    thread::sleep(Duration::from_millis(10));
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap()
    }
}
