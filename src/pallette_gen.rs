use std::{thread, time, sync::{atomic::Ordering, Arc, Mutex}, collections::HashMap};
use crate::constants;

pub fn pallette_generation_worker(
    framerate: u16, screenshot_stack: Arc<Mutex<constants::OptionalBoxedStack>>,
) -> Vec<u32> {
    // Get the sleep time for 2 frames.
    let sleep_time = 1000 / framerate * 2;

    // Defines a mapping of colors to their frequency.
    let mut color_map: HashMap<u32, u32> = HashMap::new();

    while !constants::SHOULD_DIE.load(Ordering::Relaxed) {
        // Sleep for 2 frames.
        thread::sleep(time::Duration::from_millis(sleep_time as u64));

        // Take the current queue as the owner and blank it.
        let mut stack_value = screenshot_stack.lock().unwrap().take();

        // Process each image in the stack.
        while !stack_value.is_none() {
            // Unwrap the item.
            let unwrapped = stack_value.unwrap();

            // Sanity check the length is divisible by 4.
            if unwrapped.item.len() % 4 != 0 {
                // Skip this image.
                stack_value = unwrapped.next;
                continue
            }

            // Iterate over the image.
            for i in (0..unwrapped.item.len()).step_by(4) {
                // Get the color.
                let color = (unwrapped.item[i] as u32) << 24 | (unwrapped.item[i + 1] as u32) << 16 | (unwrapped.item[i + 2] as u32) << 8 | (unwrapped.item[i + 3] as u32);

                // Check if the color is in the map.
                if color_map.contains_key(&color) {
                    // Increment the value.
                    let value = color_map.get_mut(&color).unwrap();
                    *value += 1;
                } else {
                    // Insert the value.
                    color_map.insert(color, 1);
                }
            }

            // Get the next value.
            stack_value = unwrapped.next;
        }
    }

    // Create the vector for the colors.
    let map_len = color_map.len();
    let mut vec_cap = map_len;
    if vec_cap > 256 {
        vec_cap = 256;
    }
    let mut vec: Vec<u32> = Vec::with_capacity(vec_cap);
    drop(vec_cap);

    // If the map length is <= 256, just plop the map into this.
    if map_len <= 256 {
        // Iterate over the map.
        for (key, _) in color_map {
            // Push the key into the vec.
            vec.push(key);
        }
    } else {
        // Get the map sorted by value from highest to lowest.
        let mut sorted_map: Vec<(&u32, &u32)> = color_map.iter().collect();
        sorted_map.sort_by(|a, b| b.1.cmp(a.1));

        // Iterate over the map.
        for (key, _) in sorted_map {
            // Push the key into the vec.
            vec.push(*key);

            // Check if we have enough colors.
            if vec.len() == 256 {
                break
            }
        }
    }

    // Return the vec.
    vec
}
