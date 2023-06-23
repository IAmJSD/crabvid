use std::{sync::{Arc, Mutex, atomic::Ordering}, thread, time, collections::VecDeque};
use screenshots::Screen;
use crate::constants;

fn do_capture(screens: &Vec<Screen>, x: i32, y: i32, w: u32, h: u32) -> Option<Vec<u8>> {
    // Capture wants w/h as i32.
    let w_i32 = w as i32;
    let h_i32 = h as i32;

    // Iterate over the screens.
    for screen in screens {
        // Check if the region is in the screen.
        let info = screen.display_info;
        let info_width = info.width as i32;
        let info_height = info.height as i32;
        if x >= info.x && y >= info.y && x + w_i32 <= info.x + info_width && y + h_i32 <= info.y + info_height {
            // Get the image.
            let image = screen.capture_area(x, y, w, h);

            // Check if this is a error.
            if image.is_err() {
                // Return none.
                return None;
            }
            let image = image.unwrap();

            // Return the image.
            return Some(image.owned_rgba);
        }
    }

    // Return none.
    None
}

pub fn screenshotting_worker(
    framerate: u16, x: i32, y: i32, w: u32, h: u32, gif: bool,
    screenshot_stack: Arc<Mutex<constants::OptionalBoxedStack>>,
) -> VecDeque<Arc<Vec<u8>>> {
    // Get the sleep time for a frame.
    let sleep_time = 1000 / framerate;

    // Defines a vec of vec u8's.
    let mut vecvec: VecDeque<Arc<Vec<u8>>> = VecDeque::new();

    // Get the screens.
    let mut screens = Screen::all().unwrap();

    let image_size_bytes = (w * h * 4) as usize;
    while !constants::SHOULD_DIE.load(Ordering::Relaxed) {
        // Check if we have been signalled to pause.
        if constants::PAUSED.load(Ordering::Relaxed) {
            // Sleep for 1 frame.
            thread::sleep(time::Duration::from_millis(sleep_time as u64));
            continue
        }

        // Try to capture with the existing screens.
        let mut image = do_capture(&screens, x, y, w, h);

        // Handle first time capture failures if for example the display was
        // unplugged or blipped.
        if image.is_none() {
            // Re-get the screens.
            screens = Screen::all().unwrap();

            // Re-try to capture.
            image = do_capture(&screens, x, y, w, h);

            // If we still have none, we just allocate a huge blank vector.
            // This is because it will be all black.
            if image.is_none() {
                // Allocate a vec of the correct size.
                let vec = vec![0; image_size_bytes];

                // Set the image to the vec.
                image = Some(vec);
            }
        }
        let image = Arc::new(image.unwrap());
        let image_clone = Arc::clone(&image);

        // Add to the vecvec.
        vecvec.push_back(image);

        // If this is a gif, we should add to the stack.
        if gif {
            // Lock the stack.
            let mut stack = screenshot_stack.lock().unwrap();

            // Create the new stack item.
            let new_stack_item = Box::new(constants::Stack {
                item: image_clone,
                next: stack.take(),
            });

            // Set the stack to the new stack item.
            *stack = Some(new_stack_item);

            // Drop the lock.
            drop(stack);
        }

        // Sleep for 1 frame.
        thread::sleep(time::Duration::from_millis(sleep_time as u64));
    }

    // Return the vector of vectors.
    vecvec
}
