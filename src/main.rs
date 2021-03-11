fn main() {
    let pin_to_core = std::env::args().nth(1).as_deref() == Some("--pin-to-core");

    let core_ids = core_affinity::get_core_ids().unwrap();
    let core_ids = core_ids.iter().cycle().copied();

    let threads: Vec<_> = (0..10_000)
        .zip(core_ids)
        .map(|(_idx, core_id)| {
            std::thread::spawn(move || {
                if pin_to_core {
                    core_affinity::set_for_current(core_id);
                }
                for _ in 0..1000 {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap()
    }
}
