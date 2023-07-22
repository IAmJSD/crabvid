use windows::{
    Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::{
        UI::WindowsAndMessaging::{
            WNDCLASSW, RegisterClassW, CS_HREDRAW, CS_VREDRAW, LoadCursorW,
            IDC_ARROW, DefWindowProcA, WS_OVERLAPPEDWINDOW, WS_VISIBLE, CreateWindowExW, WINDOW_EX_STYLE,
            MSG, GetMessageA, DispatchMessageA, WM_PAINT,
        },
        Graphics::Gdi::{GetSysColorBrush, COLOR_3DFACE, BeginPaint, FillRect, EndPaint}, Foundation::{HWND, WPARAM, LPARAM, LRESULT},
    },
    w,
};
use crate::ui::ui_coordinates_bindings;
use screenshots::Screen;

struct Callbacks {
    timer_handle: HWND,
    pause_cb: extern fn(bool),
    stop_cb: extern fn(),
}

// There's not really a way to pass arguments to the window procedure, so we have to use a global.
static mut ARGS: Option<Callbacks> = None;

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            // Handle the paint event.
            WM_PAINT => {
                if window == ARGS.as_ref().unwrap().timer_handle {
                    // Handle checking if this is dark mode.
                    // TODO
                } else {
                    // Handle painting the window green. We always start off recording.
                    let mut brush = GetSysColorBrush(COLOR_3DFACE);
                    brush.0 = 0x0000FF00;
                    let mut paint_struct = std::mem::zeroed();
                    let hdc = BeginPaint(window, &mut paint_struct);
                    FillRect(hdc, &paint_struct.rcPaint, brush);
                    EndPaint(window, &paint_struct);                    
                }

                // Return 0 to indicate that the message was handled.
                LRESULT(0)
            },

            // Do the default behaviour for all other messages.
            _ => DefWindowProcA(window, message, wparam, lparam)
        }
    }
}

pub fn render_ui(x: i32, y: i32, w: u32, h: u32, pause_cb: extern fn(bool), stop_cb: extern fn()) {
    // Get the screen this is on.
    let screens = Screen::all().unwrap();
    let screen = screens.iter().find(|s| {
        let info = s.display_info;

        x >= info.x && x <= info.x + info.width as i32 &&
        y >= info.y && y <= info.y + info.height as i32
    }).unwrap();

    // Setup the various parts of the window.
    unsafe {
        // Create the window class.
        let instance = GetModuleHandleA(None).unwrap();
        let class_name = w!("ActiveRecordingClass");
        let class = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hbrBackground: GetSysColorBrush(COLOR_3DFACE),
            hInstance: instance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            lpszClassName: class_name,
            ..Default::default()
        };
        RegisterClassW(&class);

        // Get the best UI coordinates.
        let ui_coord = ui_coordinates_bindings::screenvidcap_get_best_ui_coordinates(
            300,
            100,
            x as u32,
            y as u32,
            w,
            h,

            screen.display_info.width, screen.display_info.height,
            screen.display_info.x, screen.display_info.y,
        );

        // Create the controller window.
        let controller_window = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            class_name,
            w!("Active Recording"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            ui_coord.x,
            ui_coord.y,
            300,
            100,
            None,
            None,
            instance,
            None,
        );

        // This is actually safe because this is called once and the UI is not drawn yet.
        ARGS = Some(Callbacks {
            pause_cb,
            stop_cb,
            timer_handle: controller_window,
        });
    }

    // Start the UI loop.
    let mut message = MSG::default();
    unsafe {
        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }
    }
}
