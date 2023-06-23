use crate::constants;

#[cfg(target_os = "macos")]
mod ui_darwin_bindings;

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
