use std::{sync::{Arc, Mutex, atomic::Ordering}, thread, time, collections::VecDeque, num::NonZeroU32};
use screenshots::Screen;
use crate::constants;
use fast_image_resize::{Image, PixelType, Resizer, ResizeAlg};

fn do_capture(
    screens: &Vec<Screen>, x: i32, y: i32, w: u32, h: u32,
    resizer: &mut Resizer,
) -> Option<Vec<u8>> {
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
            let image = screen.capture_area(
                x, y, w, h,
            );

            // Check if this is a error.
            if image.is_err() {
                // Return none.
                return None;
            }
            let image = image.unwrap();

            // Resize the image if the scale factor isn't 1.
            let mut image_pixels = image.owned_rgba;
            if info.scale_factor != 1.0 {
                // Get the 2 images in the type required by the library we are using.
                let scaled_width = ((w as f32) * info.scale_factor).floor() as u32;
                let scaled_height = ((h as f32) * info.scale_factor).floor() as u32;
                let different_sized_image = Image::from_vec_u8(
                    NonZeroU32::new(scaled_width).unwrap(),
                    NonZeroU32::new(scaled_height).unwrap(),
                    image_pixels, PixelType::U8x4,
                ).unwrap();
                let mut image = Image::new(
                    NonZeroU32::new(w).unwrap(),
                    NonZeroU32::new(h).unwrap(),
                    PixelType::U8x4,
                );
                let d_view = different_sized_image.view();
                let mut r_view = image.view_mut();

                // Make a resizer.
                resizer.resize(&d_view, &mut r_view).unwrap();

                // Set the image pixels to this.
                image_pixels = image.into_vec();
            }

            // Return the image.
            return Some(image_pixels);
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
    let mut resizer = Resizer::new(ResizeAlg::Nearest);
    while !constants::SHOULD_DIE.load(Ordering::Relaxed) {
        // Check if we have been signalled to pause.
        if constants::PAUSED.load(Ordering::Relaxed) {
            // Sleep for 1 frame.
            thread::sleep(time::Duration::from_millis(sleep_time as u64));
            continue
        }

        // Try to capture with the existing screens.
        let mut image = do_capture(&screens, x, y, w, h, &mut resizer);

        // Handle first time capture failures if for example the display was
        // unplugged or blipped.
        if image.is_none() {
            // Re-get the screens.
            screens = Screen::all().unwrap();

            // Re-try to capture.
            image = do_capture(&screens, x, y, w, h, &mut resizer);

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

        // Check if it is possible with the number of pixels for it to be hxw.
        if image.len() != image_size_bytes {
            // This is a error. Panic.
            panic!(
                "Image size is not correct. This means the capture failed in terrible ways.
Expected: {} bytes, got: {} bytes.",
                 image_size_bytes, image.len()
            );
        }

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
