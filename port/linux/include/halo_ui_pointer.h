/*
HALO_UI_POINTER.H

The mouse in the menus of the desktop builds, and the touchscreen in those
of the Android build. While a menu is up, the mouse is released and its
pointer shows (port/linux/src/sdl_platform.c), or taps and drags go to the
menus (port/linux/src/touch_input.c); each frame the menus
(source/interface/ui_widget.c) ask where the pointer is, in their own 640x480
coordinates (port/linux/src/d3d8_gl.c undoes the letterbox, the scale and the
widescreen centering), and what it did since.
*/

#ifndef HALO_UI_POINTER_H
#define HALO_UI_POINTER_H

struct halo_ui_pointer
{
	short x, y;
	/* where the latest left click was */
	short click_x, click_y;
	unsigned char moved;
	unsigned char left_clicks;
	unsigned char right_clicks;
	/* whole wheel notches, away from the user positive */
	signed char wheel_steps;
	/* nonzero when the pointer is the touchscreen, not a mouse: a drag and a
	wheel notch are the same steps, but the menus treat them differently */
	unsigned char touch;
	/* fingers down since the last read and where the latest went down; only
	the touchscreen reports them (debug.touch_targets shows them) */
	unsigned char downs;
	short down_x, down_y;
};

/* gives the menus the pointer; frees the mouse for the menus while
menus_active and captures it for aiming when not (touchscreen: taps and
drags go to the menus while menus_active); returns nonzero while menus are
active and the pointer could be read */
int halo_ui_pointer_update(int menus_active, struct halo_ui_pointer *pointer);

/* the item a plugin that draws the menus anew (the macOS app's Halo UI)
drew under the pointer: its widget_instance's address (0 when no item is
drawn under it), where across and down the drawn item the pointer is (0 to
1), and the kind of target it stands for (enum ui_mouse_target_kind in
ui_widget.c, or -1 for whichever its widget has) */
struct halo_ui_redrawn
{
	unsigned int widget;
	float across, down;
	short kind;
	short unused;
};

/* the item drawn under the pointer (which 0) or under its latest click
(which 1); 0 where no plugin draws the menus anew, and the menus' own
layout is what is drawn */
int halo_ui_pointer_redrawn(int which, struct halo_ui_redrawn *redrawn);

/* a picture of the game's own (a frame of a widget's bitmap, from the map)
that a plugin that draws the menus anew placed in its drawing: where, in the
menus' centered 640 (x0, y0, x1, y1, on a screen screen_width by 480), and
which frame of the bitmap's first sequence */
struct halo_ui_placed
{
	short bounds[4];
	short frame;
	short unused;
};

/* the index'th picture placed for the widget_instance at widget; 0 when
there is none, or no plugin draws the menus anew */
int halo_ui_picture_placed(unsigned int widget, int index, int screen_width, struct halo_ui_placed *placed);

/* nonzero while a plugin that draws the loading screen anew holds the game
on its map load's loop (source/game/game.c's game_precache_new_map), its
screen not yet done: the first map loaded from the menus sweeps for a few
seconds however fast it loads; 0 where no plugin does */
int halo_ui_loading_held(void);

/* the texels, from the top left of a bitmap bitmap_width by bitmap_height,
that a placed picture shows in its hole hole_width by hole_height: the
picture's own picture_width by picture_height (a stock level's is its
widget's bounds, at the top left of a larger bitmap), no more than the
bitmap has, cropped from the right or the bottom to the hole's shape so
that it fills the hole unstretched; texels[0] across, texels[1] down */
static inline void halo_ui_placed_texels(
	short picture_width, short picture_height,
	short bitmap_width, short bitmap_height,
	short hole_width, short hole_height,
	short texels[2])
{
	long width = picture_width < bitmap_width ? picture_width : bitmap_width;
	long height = picture_height < bitmap_height ? picture_height : bitmap_height;

	if (width > 0 && height > 0 && hole_width > 0 && hole_height > 0)
	{
		if (width * hole_height > height * hole_width)
			width = height * hole_width / hole_height;
		else
			height = width * hole_height / hole_width;
	}
	texels[0] = (short)width;
	texels[1] = (short)height;
}

/* the open scoreboard's pointer (game_engine.c), offered (a network game's)
or not: 1 while a right click has freed it, with where it is and what it
did since the last call, in the screen's coordinates (the game's drawing,
not the menus' centered 640); 0 while it is not; -1 where there is none
(Android) */
int halo_scoreboard_pointer_update(int offered, struct halo_ui_pointer *pointer);

#endif
