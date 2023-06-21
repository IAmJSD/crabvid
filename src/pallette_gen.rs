use std::{thread, time, sync::atomic::Ordering};
use crate::constants;

pub fn pallette_generation_worker(framerate: u16) {
    // Get the sleep time for 2 frames.
    let sleep_time = 1000 / framerate * 2;

    while !constants::SHOULD_DIE.load(Ordering::Relaxed) {
        // Sleep for 2 frames.
        thread::sleep(time::Duration::from_millis(sleep_time as u64));

        // Take the current queue as the owner and blank it.
        // TODO

        // Process each image.
        // TODO
    }
}
