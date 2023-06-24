#ifdef __APPLE__
#ifndef _SCREENVIDCAP_UI
#define _SCREENVIDCAP_UI
#include <Cocoa/Cocoa.h>
#include "./ui_coordinates.c"

typedef struct {
    int x;
    int y;
    unsigned int w;
    unsigned int h;
    void (*pause_cb)(bool);
    void (*stop_cb)();
} ui_args;

ui_args screenvidcap_darwin_args;

@interface KeyWindow : NSWindow
@end

@implementation KeyWindow

- (BOOL)canBecomeKeyWindow {
    return YES;
}

@end

int macosify_y(int y, NSRect display) {
    return display.size.height - y;
}

@interface UIApplicationDelegate : NSObject <NSApplicationDelegate>

@property (assign) NSWindow *window;
@property (nonatomic) u_int64_t millis;
@property (nonatomic, strong) NSMutableArray *windows;
@property (nonatomic, strong) NSButton *startButton;
@property (nonatomic, strong) NSButton *stopButton;
@property (nonatomic, strong) NSTextView *timeView;
@property (nonatomic, strong) NSTimer *timer;

- (void)tick:(NSTimer *)timer;
- (void)pause:(id)sender;
- (void)stop:(id)sender;

@end

bool paused = false;

@implementation UIApplicationDelegate

- (void)tick:(NSTimer *)timer {
    if (paused) return;
    self.millis += 10;
    char time_str[13];
    sprintf(time_str, "%02d:%02d:%02d.%03d", (int)(self.millis / 1000 / 60 / 60), (int)(self.millis / 1000 / 60 % 60), (int)(self.millis / 1000 % 60), (int)(self.millis % 1000));
    self.timeView.string = [NSString stringWithUTF8String:time_str];
}

- (void)applicationDidFinishLaunching:(NSNotification *)aNotification {
    self.windows = [NSMutableArray array];
    CGRect screenRect = [[NSScreen mainScreen] frame];

    // Create the start button.
    self.startButton = [[NSButton alloc] initWithFrame:NSMakeRect(120, 10, 80, 32)];
    [self.startButton setTitle:@"Pause"];
    [self.startButton setBezelStyle:NSBezelStyleRegularSquare];
    [self.startButton setAction:@selector(pause:)];
    [self.startButton setTarget:self];

    // Create the stop button.
    self.stopButton = [[NSButton alloc] initWithFrame:NSMakeRect(200, 10, 80, 32)];
    [self.stopButton setTitle:@"Stop"];
    [self.stopButton setBezelStyle:NSBezelStyleRegularSquare];
    [self.stopButton setAction:@selector(stop:)];
    [self.stopButton setTarget:self];

    // Create the time view.
    self.timeView = [[NSTextView alloc] initWithFrame:NSMakeRect(10, 15, 280, 20)];
    [self.timeView setEditable:NO];
    [self.timeView setSelectable:NO];
    [self.timeView setDrawsBackground:NO];
    [self.timeView setTextColor:[NSColor whiteColor]];
    [self.timeView setFont:[NSFont fontWithName:@"Helvetica" size:15]];
    [self.timeView setString:@"00:00:00.000"];

    // Create a window to hold the start and stop buttons
    ui_coord uiCoords = screenvidcap_get_best_ui_coordinates(
            300, 50, screenvidcap_darwin_args.x, screenvidcap_darwin_args.y, screenvidcap_darwin_args.w, screenvidcap_darwin_args.h,
            screenRect.size.width, screenRect.size.height, screenRect.origin.y,
            screenRect.origin.x);
    NSRect buttonWindowRect = NSMakeRect(uiCoords.x, macosify_y(uiCoords.y, screenRect), 300, 50);
    KeyWindow *buttonWindow = [[KeyWindow alloc] initWithContentRect:buttonWindowRect
                                                         styleMask:NSWindowStyleMaskTitled
                                                           backing:NSBackingStoreBuffered
                                                             defer:NO];
    [buttonWindow setLevel:NSFloatingWindowLevel];
    [buttonWindow setOpaque:YES];
    [buttonWindow setIgnoresMouseEvents:NO];
    [buttonWindow setCollectionBehavior:NSWindowCollectionBehaviorCanJoinAllSpaces | NSWindowCollectionBehaviorStationary | NSWindowCollectionBehaviorIgnoresCycle];
    [buttonWindow makeKeyAndOrderFront:nil];
    self.window = buttonWindow;
    [self.window.contentView addSubview:self.startButton];
    [self.window.contentView addSubview:self.stopButton];
    [self.window.contentView addSubview:self.timeView];

    // Create the windows for the screen capture overlay.
    int topY = macosify_y(screenvidcap_darwin_args.y, screenRect) - 1;
    int bottomY = macosify_y(screenvidcap_darwin_args.y + screenvidcap_darwin_args.h, screenRect) - 1;
    int height = topY - bottomY;
    for (int i = 0; i < 4; i++) {
        NSRect rect;
        if (i == 0) {
            // Top
            rect = NSMakeRect(
                    screenvidcap_darwin_args.x - 1,
                    topY, screenvidcap_darwin_args.w + 2, 1);
        } else if (i == 1) {
            // Bottom
            rect = NSMakeRect(
                    screenvidcap_darwin_args.x - 1,
                    bottomY, screenvidcap_darwin_args.w + 2, 1);
        } else if (i == 2) {
            // Left
            rect = NSMakeRect(screenvidcap_darwin_args.x - 1,
                              topY - height, 1, height);
        } else {
            // Right
            rect = NSMakeRect((screenvidcap_darwin_args.x + screenvidcap_darwin_args.w) + 1,
                              topY - height, 1, height);
        }
        KeyWindow *window = [[KeyWindow alloc] initWithContentRect:rect
                                                       styleMask:NSWindowStyleMaskBorderless
                                                         backing:NSBackingStoreBuffered
                                                           defer:NO];
        [window setBackgroundColor:[NSColor greenColor]];
        [window setLevel:NSFloatingWindowLevel];
        [window setOpaque:NO];
        [window setIgnoresMouseEvents:YES];
        [window setCollectionBehavior:NSWindowCollectionBehaviorCanJoinAllSpaces | NSWindowCollectionBehaviorStationary | NSWindowCollectionBehaviorIgnoresCycle];
        [self.windows addObject:window];
        [window makeKeyAndOrderFront:nil];
    }

    // Start a ticker every millisecond to update the timer.
    self.timer = [NSTimer scheduledTimerWithTimeInterval:0.01
                                     target:self
                                   selector:@selector(tick:)
                                   userInfo:nil
                                    repeats:YES];
}

- (void)pause:(id)sender {
    paused = !paused;
    if (paused) {
        [self.startButton setTitle:@"Start"];
    } else {
        [self.startButton setTitle:@"Pause"];
    }

    // Make the windows red.
    for (KeyWindow *window in self.windows) {
        if (paused) {
            [window setBackgroundColor:[NSColor redColor]];
        } else {
            [window setBackgroundColor:[NSColor greenColor]];
        }
    }

    screenvidcap_darwin_args.pause_cb(paused);
}

- (void)stop:(id)sender {
    // Get the application and delegate.
    NSApplication *application = [NSApplication sharedApplication];
    UIApplicationDelegate* delegate = [application delegate];

    // Close all running things.
    [delegate.timer invalidate];
    [delegate.window close];
    for (NSWindow* window in delegate.windows) [window close];

    // Call the callback.
    screenvidcap_darwin_args.stop_cb();
}

@end

// Draws the UI and keeps blocking/updating until the kill function is called.
// Maintains the state of the UI and if this is paused.
void screenvidcap_draw_ui(ui_args args) {
    screenvidcap_darwin_args = args;
    NSApplication *application = [NSApplication sharedApplication];
    UIApplicationDelegate *delegate = [[UIApplicationDelegate alloc] init];
    [application setDelegate:delegate];
    [application run];
}

#endif // _SCREENVIDCAP_UI
#endif // __APPLE__
