use std::sync::{Mutex, Arc};
use clap::Parser;

mod constants;
mod pallette_gen;
mod screenshotting;
mod post_processing;
mod ui;

fn capture_flow_gif(framerate: u16, x: i32, y: i32, w: u32, h: u32, screenshot_stack: Arc<Mutex<constants::OptionalBoxedStack>>) {
    // Start the pallette generation worker in a new thread.
    let s_ref = Arc::clone(&screenshot_stack);
    let pallette_gen_thread = std::thread::spawn(move || {
        pallette_gen::pallette_generation_worker(framerate, s_ref)
    });

    // Wait for the screenshotting worker to be done.
    let images = screenshotting::screenshotting_worker(framerate, x, y, w, h, true, screenshot_stack);

    // Print out 'ABOUT_TO_ENCODE' so that any software hooking on this can show the user.
    print!("ABOUT_TO_ENCODE");

    // Wait for the pallette generation worker to be done.
    let color_map = Some(pallette_gen_thread.join().unwrap());

    // Handle post processing.
    post_processing::do_post_processing(images, w, h, framerate, color_map);

    // Exit with code 0.
    std::process::exit(0);
}

fn capture_flow_mp4(framerate: u16, x: i32, y: i32, w: u32, h: u32, screenshot_stack: Arc<Mutex<constants::OptionalBoxedStack>>) {
    // Wait for the screenshotting worker to be done.
    let images = screenshotting::screenshotting_worker(framerate, x, y, w, h, false, screenshot_stack);

    // Print out 'ABOUT_TO_ENCODE' so that any software hooking on this can show the user.
    print!("ABOUT_TO_ENCODE");

    // Handle post processing.
    post_processing::do_post_processing(images, w, h, framerate, None);

    // Exit with code 0.
    std::process::exit(0);
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    framerate: u16,
    #[arg(short, long)]
    x: i32,
    #[arg(short, long)]
    y: i32,
    #[arg(long)]
    width: u32,
    #[arg(long)]
    height: u32,
    #[arg(short, long)]
    gif: bool,
}

fn main() {
    // Defines the mutex for the boxed stack that contains the screenshot stack.
    let screenshot_stack: Arc<Mutex<constants::OptionalBoxedStack>> = Arc::new(Mutex::new(None));

    // Parse the CLI arguments with clap.
    let args = Args::parse();

    // Start the capture flow in a new thread.
    let capture_flow_thread = std::thread::spawn(move || {
        if args.gif {
            capture_flow_gif(args.framerate, args.x, args.y, args.width, args.height, screenshot_stack);
        } else {
            capture_flow_mp4(args.framerate, args.x, args.y, args.width, args.height, screenshot_stack);
        }
    });

    // Make a thread to watch this and close the process on panic.
    let panic_detection_thread = std::thread::spawn(move || {
        if capture_flow_thread.join().is_err() {
            std::process::exit(1);
        }
    });

    // Start the UI thread. This might block forever, but that's fine. The post capture flow will still run.
    ui::render_ui(args.x, args.y, args.width, args.height);

    // If we are get here, the UI thread did not block forever. Go ahead and join the capture flow panic detection thread.
    panic_detection_thread.join().unwrap();
}
