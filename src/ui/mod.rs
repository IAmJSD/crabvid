use crate::constants;

// Used in Windows/Linux UI code.
mod ui_coordinates_bindings;

#[cfg(target_os = "macos")]
mod ui_darwin_bindings;

#[cfg(target_os = "windows")]
mod ui_windows;

extern fn pause_cb(paused: bool) {
    constants::PAUSED.store(paused, std::sync::atomic::Ordering::Relaxed);
}

extern fn stop_cb() {
    constants::SHOULD_DIE.store(true, std::sync::atomic::Ordering::Relaxed);
}

#[cfg(target_os = "macos")]
pub fn render_ui(x: i32, y: i32, w: u32, h: u32) {
    unsafe {
        ui_darwin_bindings::screenvidcap_draw_ui(ui_darwin_bindings::ui_args {
            x, y, w, h,
            pause_cb: Some(pause_cb),
            stop_cb: Some(stop_cb),
        });
    }
}

#[cfg(target_os = "windows")]
pub fn render_ui(x: i32, y: i32, w: u32, h: u32) {
    ui_windows::render_ui(x, y, w, h, pause_cb, stop_cb);
}
