use std::{sync::Arc, io::Write};

fn encode_mp4(images: Vec<Arc<Vec<u8>>>, w: u32, h: u32, framerate: u16) -> Vec<u8> {
    // TODO
    vec![]
}

fn encode_gif(
    images: Vec<Arc<Vec<u8>>>, w: u32, h: u32, framerate: u16,
    color_map: Vec<u32>,
) -> Vec<u8> {
    // TODO
    vec![]
}

pub fn do_post_processing(
    images: Vec<Arc<Vec<u8>>>, w: u32, h: u32, framerate: u16,
    color_map: Option<Vec<u32>>,
) {
    // Encode the video.
    let video = if let Some(color_map) = color_map {
        encode_gif(images, w, h, framerate, color_map)
    } else {
        encode_mp4(images, w, h, framerate)
    };

    // Pipe the video to stdout.
    std::io::stdout().write_all(&video).unwrap();
}
