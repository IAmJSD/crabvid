use clap::Parser;

mod constants;
mod pallette_gen;
mod screenshotting;
mod post_processing;
mod ui;

fn capture_flow_gif(framerate: u16, x: u32, y: u32, w: u32, h: u32) {
    // Start the pallette generation worker in a new thread.
    let pallette_gen_thread = std::thread::spawn(move || {
        pallette_gen::pallette_generation_worker(framerate);
    });

    // Wait for the screenshotting worker to be done.
    let images = screenshotting::screenshotting_worker(framerate, x, y, w, h, true);

    // Wait for the pallette generation worker to be done.
    pallette_gen_thread.join().unwrap();

    // Handle post processing.
    post_processing::do_post_processing(images, w, h, framerate, true);

    // Exit with code 0.
    std::process::exit(0);
}

fn capture_flow_mp4(framerate: u16, x: u32, y: u32, w: u32, h: u32) {
    // Wait for the screenshotting worker to be done.
    let images = screenshotting::screenshotting_worker(framerate, x, y, w, h, false);

    // Handle post processing.
    post_processing::do_post_processing(images, w, h, framerate, false);

    // Exit with code 0.
    std::process::exit(0);
}

fn main() {
    // TODO: Parse the CLI arguments!
    const FRAMERATE: u16 = 30;
    const X: u32 = 0;
    const Y: u32 = 0;
    const W: u32 = 100;
    const H: u32 = 100;
    const GIF: bool = true;

    // Turn the above into CLI arguments.
    

    // Start the capture flow in a new thread.
    let capture_flow_thread = std::thread::spawn(move || {
        if GIF {
            capture_flow_gif(FRAMERATE, X, Y, W, H);
        } else {
            capture_flow_mp4(FRAMERATE, X, Y, W, H);
        }
    });

    // Start the UI thread. This might block forever, but that's fine. The post capture flow will still run.
    ui::render_ui();

    // If we are still here, the UI thread did not block forever. Go ahead and join the capture flow thread.
    capture_flow_thread.join().unwrap();
}
