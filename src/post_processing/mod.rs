use std::{sync::Arc, collections::VecDeque};

mod gif;
mod mp4;

pub fn do_post_processing(
    images: VecDeque<Arc<Vec<u8>>>, w: u32, h: u32, framerate: u16,
    color_map: Option<Vec<u32>>,
) {
    // Encode the video.
    if let Some(color_map) = color_map {
        gif::encode_gif(images, w, h, framerate, color_map);
    } else {
        mp4::encode_mp4(images, w, h, framerate);
    }
}
