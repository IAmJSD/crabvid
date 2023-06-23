#ifndef _SCREENVIDCAP_COORDINATES
#define _SCREENVIDCAP_COORDINATES

typedef struct {
    int x;
    int y;
} ui_coord;

ui_coord screenvidcap_get_best_ui_coordinates(
    unsigned int ui_width, unsigned int ui_height,
    unsigned int capture_x, unsigned int capture_y,
    unsigned int capture_width, unsigned int capture_height,
    unsigned int screen_width, unsigned int screen_height,
    int screen_top, int screen_left
) {
    // Calculate the X by trying to get somewhat in the middle of the capture.
    int m = (int)(capture_width / 2) - (int)(ui_width / 2); // I don't get why this maths works. What the fuck is going on here?
    int x = (int)(capture_x) + m - 1;

    // Check if we should put it above the capture.
    int y = (int)(capture_y) - (int)(ui_height) - (int)(ui_height / 4);
    if (screen_top > y) {
        // In this event, this goes above the screen. We should try and put it below the capture.
        y = (int)(capture_y) + (int)(capture_height) + (int)(ui_height / 4);
        if (screen_top + screen_height < y)
            // This is too far down. We should just accept it will be in the capture and go back to the top.
            // Due to this, we should leave more of a gap.
            y = (int)(capture_y) + (int)(ui_height);
            // TODO: Handle shifting to the right or left.
    }

    // Return the result.
    return (ui_coord) {
        .x = x,
        .y = y
    };
}

#endif // _SCREENVIDCAP_COORDINATES
