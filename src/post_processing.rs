use std::{sync::Arc, collections::VecDeque, io::Write};

fn encode_mp4(images: VecDeque<Arc<Vec<u8>>>, w: u32, h: u32, framerate: u16) {
    // TODO
}

fn encode_gif(
    mut images: VecDeque<Arc<Vec<u8>>>, w: u32, h: u32, framerate: u16,
    color_map: Vec<u32>,
) {
    // Defines the sleep time.
    let sleep_time = 1000 / framerate;

    // Make the color map a u8 slice.
    let mut color_map_u8 = vec![0; color_map.len() * 3];
    for (i, color) in color_map.iter().enumerate() {
        color_map_u8[i * 3] = (color >> 16) as u8;
        color_map_u8[i * 3 + 1] = (color >> 8) as u8;
        color_map_u8[i * 3 + 2] = *color as u8;
    }

    // Create the gif encoder.
    let mut stdout = std::io::stdout();
    let mut encoder = gif::Encoder::new(
        &stdout,
        w as u16, h as u16,
        &color_map_u8,
    ).unwrap();

    // Encode each image.
    let mut popped_image = images.pop_front();
    while !popped_image.is_none() {
        // Become the owner of the image.
        let mut image = Arc::try_unwrap(popped_image.unwrap()).unwrap();

        // Create the frame.
        let mut frame = gif::Frame::from_rgba(
            w as u16, h as u16,
            image.as_mut_slice(),
        );
        frame.delay = sleep_time / 10;

        // Write the frame.
        encoder.write_frame(&frame).unwrap();

        // Get the next image.
        popped_image = images.pop_front();
    }

    // Drop the encoder.
    drop(encoder);

    // Flush stdout.
    stdout.flush().unwrap();
}

pub fn do_post_processing(
    images: VecDeque<Arc<Vec<u8>>>, w: u32, h: u32, framerate: u16,
    color_map: Option<Vec<u32>>,
) {
    // Encode the video.
    if let Some(color_map) = color_map {
        encode_gif(images, w, h, framerate, color_map);
    } else {
        encode_mp4(images, w, h, framerate);
    }
}
