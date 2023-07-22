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
);
