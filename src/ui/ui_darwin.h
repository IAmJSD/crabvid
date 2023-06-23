#include <stdbool.h>

typedef struct {
    int x;
    int y;
    unsigned int w;
    unsigned int h;
    void (*pause_cb)(bool paused);
    void (*stop_cb)();
} ui_args;

void screenvidcap_draw_ui(ui_args args);
