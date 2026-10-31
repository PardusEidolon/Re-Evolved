/*
TEXT_TAGS.C

The game's text drawn at the display's resolution from its own font tags
(text_hires.h, the interface source/rasterizer/rasterizer_text.c draws
through). The Linux port's text_hires.c stands a TrueType font in for each
tag; game.wasm keeps the tags' own letters and only draws them sharper.

A character is drawn first from its tag's 8-bit coverage, at the display's
pixels per unit of the 480 lines (times the text's scale), into an 8-bit
atlas the game binds as a placeholder bitmap (xbox_textures.c). The
coverage, filtered bilinearly, is read as a field whose level set is the
letter's edge: each pixel's distance from that edge is the field's value
less the level, over its gradient, and the pixel is inked by how much of it
is inside. The level is chosen for each glyph so that it keeps the tag's
ink, so strokes keep their weight however thin. A glyph's quad is its tag
bitmap's (a texel wider each side), so the game's layout is untouched.

A change of the display's scale, of the map's fonts, or a full atlas
starts the atlas over.
*/

#include "cseries.h"
#include "tag_files/tag_groups.h"
#include "text/font_group.h"
#include "opence_render.h"
#include "text_hires.h"

#include <math.h>
#include <stdlib.h>
#include <string.h>

/* (each font tag the menus and the HUD draw, at each of its oversamples) */
#define MAXIMUM_FONTS 16
#define ATLAS_SIZE 2048
/* clear texels around each glyph, so that filtering never reaches the next */
#define ATLAS_PADDING 2
#define GLYPH_CACHE_SIZE 4096
/* the tag texels a glyph's field reaches past its bitmap, each side */
#define FIELD_MARGIN 1
/* the largest glyph drawn, in the atlas's pixels each way */
#define MAXIMUM_GLYPH_PIXELS 512
/* the halvings that find a glyph's level */
#define LEVEL_STEPS 12

/* a font tag's character, as the tag lays it out (rasterizer_text.c's) */
struct font_character
{
	word character;
	short character_width;
	short bitmap_width;
	short bitmap_height;
	short bitmap_origin_x;
	short bitmap_origin_y;
	short hardware_character_index;
	short pad;
	long pixels_offset;
};

/* the Metal device's commands (port/linux/src/xgpu.h, which game.wasm's
headers cannot include beside the game's) */
unsigned int *xgpu_metal_command(unsigned int op, unsigned int words);
void xgpu_metal_flush(void);
unsigned int xgpu_metal_new_id(unsigned int kind);
void xgpu_metal_destroy(unsigned int kind, unsigned int id);
int config_boolean(const char *name);
unsigned long config_changes(void);

static struct
{
	char tag[64];
	float oversample;
	/* its tag, as last found (another map's font starts the atlas over) */
	long tag_index;
	struct font_header *header;
} fonts[MAXIMUM_FONTS];
static long font_count;

static struct
{
	long font; /* -1: free */
	unsigned long code;
	short x, y, width, height;
	/* the glyph's quad, in units from the pen on the baseline */
	float left, top, right, bottom;
	float advance;
} glyphs[GLYPH_CACHE_SIZE];

static unsigned char *atlas;
static float atlas_scale; /* the pixels per unit its glyphs have */
static int pack_x, pack_y, pack_row_height;
static int dirty_top = ATLAS_SIZE, dirty_bottom;
static unsigned long placeholder_data, placeholder_width, placeholder_height;
static unsigned int atlas_texture;
static void (*reset_hook)(void);
/* a glyph's field as it is drawn: its value and its gradient's length */
static float field_value[MAXIMUM_GLYPH_PIXELS * MAXIMUM_GLYPH_PIXELS];
static float field_slope[MAXIMUM_GLYPH_PIXELS * MAXIMUM_GLYPH_PIXELS];

static int names_differ(const char *a, const char *b)
{
	while (*a && (*a | 0x20) == (*b | 0x20))
	{
		a++;
		b++;
	}
	return (*a | 0x20) != (*b | 0x20);
}

int text_hires_enabled(void)
{
	static int enabled;
	static unsigned long read_at = (unsigned long)-1;

	if (read_at != config_changes())
	{
		read_at = config_changes();
		enabled = config_boolean("display.high_res_text");
	}
	return enabled;
}

void text_hires_set_reset_hook(void (*hook)(void))
{
	reset_hook = hook;
}

/* the atlas emptied: every glyph to be drawn again */
static void atlas_reset(float scale)
{
	long index;

	/* (text batched from the glyphs there drawn first) */
	if (reset_hook && atlas)
		reset_hook();
	if (!atlas)
		atlas = malloc(ATLAS_SIZE * ATLAS_SIZE);
	if (atlas)
		memset(atlas, 0, ATLAS_SIZE * ATLAS_SIZE);
	for (index = 0; index < GLYPH_CACHE_SIZE; index++)
		glyphs[index].font = -1;
	atlas_scale = scale;
	pack_x = pack_y = pack_row_height = 0;
	dirty_top = 0;
	dirty_bottom = ATLAS_SIZE;
}

long text_hires_font(char const *tag_name, float cap_height, short const *letter_widths, float oversample)
{
	long font;
	long tag_index;
	struct font_header *header;

	/* (the tag's own letters are drawn, so its measures need no fitting) */
	(void)cap_height;
	(void)letter_widths;
	if (!text_hires_enabled() || !tag_name)
		return -1;
	if (oversample < 1.0f)
		oversample = 1.0f;
	tag_index = tag_loaded(FONT_GROUP_TAG, tag_name);
	if (tag_index == NONE)
		return -1;
	header = font_definition_get(tag_index);
	if (!header)
		return -1;
	for (font = 0; font < font_count; font++)
	{
		if (!names_differ(fonts[font].tag, tag_name) && fonts[font].oversample == oversample)
			break;
	}
	if (font == font_count)
	{
		if (font_count >= MAXIMUM_FONTS)
			return -1;
		font_count++;
		strncpy(fonts[font].tag, tag_name, sizeof(fonts[font].tag) - 1);
		fonts[font].tag[sizeof(fonts[font].tag) - 1] = 0;
		fonts[font].oversample = oversample;
	}
	else if ((fonts[font].header != header || fonts[font].tag_index != tag_index) && atlas)
	{
		/* (another map's tag of the same name: its glyphs may differ) */
		atlas_reset(atlas_scale);
	}
	fonts[font].tag_index = tag_index;
	fonts[font].header = header;
	return font;
}

/* the font's character, its pixels within the tag's */
static struct font_character *font_character_get(long font, unsigned long code, byte const **pixels)
{
	struct font_header *header;
	struct font_character *character;
	long size;

	if (font < 0 || font >= font_count || code > 0xFFFF)
		return NULL;
	header = fonts[font].header;
	character = header ? font_get_character_by_ascii_code(header, (word)code) : NULL;
	if (!character || character->bitmap_width < 0 || character->bitmap_height < 0)
		return NULL;
	size = (long)character->bitmap_width * character->bitmap_height;
	if (character->pixels_offset < 0 || !header->pixels.address ||
		character->pixels_offset > header->pixels.size - size)
	{
		return NULL;
	}
	*pixels = (byte const *)header->pixels.address + character->pixels_offset;
	return character;
}

int text_hires_covers(long font, unsigned long code)
{
	byte const *pixels;

	/* (control characters are never drawn) */
	return code < 32 || code == ' ' || font_character_get(font, code, &pixels) != NULL;
}

/* the tag's coverage at a texel, 0 to 1 (none past its bitmap) */
static float coverage_at(byte const *pixels, int width, int height, int x, int y)
{
	return x < 0 || y < 0 || x >= width || y >= height ? 0.0f : pixels[y * width + x] * (1.0f / 255.0f);
}

/* how much of a pixel is inside the edge at level, its field's value and
slope given, at scale pixels a texel */
static float pixel_ink(float value, float slope, float level, float scale)
{
	float ink;

	if (slope < 1.0e-4f)
		return value >= level ? 1.0f : 0.0f;
	ink = 0.5f + (value - level) / slope * scale;
	return ink < 0.0f ? 0.0f : ink > 1.0f ? 1.0f : ink;
}

/* the glyph of a tag's character drawn into the atlas at x and y, width by
height pixels at scale pixels a texel */
static void glyph_draw(byte const *pixels, int bitmap_width, int bitmap_height,
	int x, int y, int width, int height, float scale)
{
	float tag_ink = 0.0f;
	float low = 0.02f, high = 0.98f, level = 0.5f;
	int row, column, step;

	for (row = 0; row < bitmap_height; row++)
	{
		for (column = 0; column < bitmap_width; column++)
			tag_ink += pixels[row * bitmap_width + column] * (1.0f / 255.0f);
	}
	/* the field: the coverage filtered bilinearly, texel centres at halves */
	for (row = 0; row < height; row++)
	{
		for (column = 0; column < width; column++)
		{
			float u = (column + 0.5f) / scale - FIELD_MARGIN - 0.5f;
			float v = (row + 0.5f) / scale - FIELD_MARGIN - 0.5f;
			int x0 = (int)floorf(u), y0 = (int)floorf(v);
			float fx = u - x0, fy = v - y0;
			float c00 = coverage_at(pixels, bitmap_width, bitmap_height, x0, y0);
			float c10 = coverage_at(pixels, bitmap_width, bitmap_height, x0 + 1, y0);
			float c01 = coverage_at(pixels, bitmap_width, bitmap_height, x0, y0 + 1);
			float c11 = coverage_at(pixels, bitmap_width, bitmap_height, x0 + 1, y0 + 1);
			float top = c00 + (c10 - c00) * fx;
			float bottom = c01 + (c11 - c01) * fx;
			float dx = (1.0f - fy) * (c10 - c00) + fy * (c11 - c01);
			float dy = bottom - top;

			field_value[row * width + column] = top + (bottom - top) * fy;
			field_slope[row * width + column] = sqrtf(dx * dx + dy * dy);
		}
	}
	/* the level whose ink is the tag's (a lower level inks more) */
	for (step = 0; step < LEVEL_STEPS && tag_ink > 0.0f; step++)
	{
		float ink = 0.0f;
		long index;

		level = (low + high) * 0.5f;
		for (index = 0; index < (long)width * height; index++)
			ink += pixel_ink(field_value[index], field_slope[index], level, scale);
		if (ink / (scale * scale) > tag_ink)
			low = level;
		else
			high = level;
	}
	for (row = 0; row < height; row++)
	{
		unsigned char *line = atlas + (long)(y + row) * ATLAS_SIZE + x;

		for (column = 0; column < width; column++)
		{
			long index = (long)row * width + column;

			line[column] = (unsigned char)(pixel_ink(field_value[index], field_slope[index], level, scale) * 255.0f + 0.5f);
		}
	}
}

int text_hires_glyph(long font, unsigned long code, struct text_hires_glyph *glyph)
{
	float scale = halo_screen_pixel_scale();
	unsigned long slot;
	long probe;

	if (font < 0 || font >= font_count || !placeholder_data || scale <= 0.0f)
		return 0;
	if (!atlas || scale != atlas_scale)
		atlas_reset(scale);
	if (!atlas)
		return 0;
	slot = ((unsigned long)font * 2654435761UL ^ code * 40503UL) & (GLYPH_CACHE_SIZE - 1);
	for (probe = 0; probe < GLYPH_CACHE_SIZE; probe++, slot = (slot + 1) & (GLYPH_CACHE_SIZE - 1))
	{
		if (glyphs[slot].font == -1 || (glyphs[slot].font == font && glyphs[slot].code == code))
			break;
	}
	if (probe == GLYPH_CACHE_SIZE)
	{
		atlas_reset(scale);
		return text_hires_glyph(font, code, glyph);
	}
	if (glyphs[slot].font == -1)
	{
		/* drawn now */
		byte const *pixels = NULL;
		struct font_character *character = font_character_get(font, code, &pixels);
		float pixel_scale = scale * fonts[font].oversample;
		int width = 0, height = 0;

		if (character && character->bitmap_width > 0 && character->bitmap_height > 0)
		{
			width = (int)ceilf((character->bitmap_width + 2 * FIELD_MARGIN) * pixel_scale);
			height = (int)ceilf((character->bitmap_height + 2 * FIELD_MARGIN) * pixel_scale);
		}
		if (width > MAXIMUM_GLYPH_PIXELS || height > MAXIMUM_GLYPH_PIXELS ||
			width + 2 * ATLAS_PADDING > ATLAS_SIZE || height + 2 * ATLAS_PADDING > ATLAS_SIZE)
		{
			return 0;
		}
		if (width > 0 && height > 0)
		{
			if (pack_x + width + 2 * ATLAS_PADDING > ATLAS_SIZE)
			{
				pack_x = 0;
				pack_y += pack_row_height;
				pack_row_height = 0;
			}
			if (pack_y + height + 2 * ATLAS_PADDING > ATLAS_SIZE)
			{
				atlas_reset(scale);
				return text_hires_glyph(font, code, glyph);
			}
			glyph_draw(pixels, character->bitmap_width, character->bitmap_height,
				pack_x + ATLAS_PADDING, pack_y + ATLAS_PADDING, width, height, pixel_scale);
			if (pack_y < dirty_top)
				dirty_top = pack_y;
			if (pack_y + height + 2 * ATLAS_PADDING > dirty_bottom)
				dirty_bottom = pack_y + height + 2 * ATLAS_PADDING;
		}
		glyphs[slot].font = font;
		glyphs[slot].code = code;
		glyphs[slot].x = (short)(pack_x + ATLAS_PADDING);
		glyphs[slot].y = (short)(pack_y + ATLAS_PADDING);
		glyphs[slot].width = (short)width;
		glyphs[slot].height = (short)height;
		if (character)
		{
			/* (the tag's bitmap, a margin wider; the pen sits at its origin) */
			glyphs[slot].left = (float)(-character->bitmap_origin_x - FIELD_MARGIN);
			glyphs[slot].top = (float)(-character->bitmap_origin_y - FIELD_MARGIN);
			glyphs[slot].right = glyphs[slot].left + width / pixel_scale;
			glyphs[slot].bottom = glyphs[slot].top + height / pixel_scale;
			glyphs[slot].advance = (float)character->character_width;
		}
		if (width > 0 && height > 0)
		{
			pack_x += width + 2 * ATLAS_PADDING;
			if (height + 2 * ATLAS_PADDING > pack_row_height)
				pack_row_height = height + 2 * ATLAS_PADDING;
		}
	}
	if (!glyphs[slot].width || !glyphs[slot].height)
		return 0;
	glyph->left = glyphs[slot].left;
	glyph->top = glyphs[slot].top;
	glyph->right = glyphs[slot].right;
	glyph->bottom = glyphs[slot].bottom;
	glyph->advance = glyphs[slot].advance;
	glyph->u0 = (float)glyphs[slot].x * placeholder_width / ATLAS_SIZE;
	glyph->v0 = (float)glyphs[slot].y * placeholder_height / ATLAS_SIZE;
	glyph->u1 = (float)(glyphs[slot].x + glyphs[slot].width) * placeholder_width / ATLAS_SIZE;
	glyph->v1 = (float)(glyphs[slot].y + glyphs[slot].height) * placeholder_height / ATLAS_SIZE;
	return 1;
}

void text_hires_register_atlas(const unsigned long *texture, unsigned long width, unsigned long height)
{
	placeholder_data = texture ? texture[1] : 0;
	placeholder_width = width;
	placeholder_height = height;
}

unsigned int text_hires_atlas_texture(unsigned long data)
{
	if (!data || data != placeholder_data || !atlas)
		return 0;
	if (!atlas_texture)
	{
		unsigned int *fields;

		atlas_texture = xgpu_metal_new_id(OPENCE_RENDER_OBJECT_TEXTURE);
		fields = atlas_texture ? xgpu_metal_command(OPENCE_RENDER_OP_CREATE_TEXTURE,
			OPENCE_RENDER_CREATE_TEXTURE_WORDS) : NULL;
		if (!fields)
		{
			if (atlas_texture)
				xgpu_metal_destroy(OPENCE_RENDER_OBJECT_TEXTURE, atlas_texture);
			atlas_texture = 0;
			return 0;
		}
		fields[0] = atlas_texture;
		fields[1] = OPENCE_RENDER_TEXTURE_2D;
		fields[2] = OPENCE_RENDER_FORMAT_RGBA8;
		fields[3] = ATLAS_SIZE;
		fields[4] = ATLAS_SIZE;
		fields[5] = 1;
		fields[6] = 1;
		fields[7] = OPENCE_RENDER_USAGE_SAMPLED;
		fields[8] = 1;
		dirty_top = 0;
		dirty_bottom = ATLAS_SIZE;
	}
	if (dirty_top < dirty_bottom)
	{
		/* (white, the glyph's coverage its alpha, as the maps' fonts are) */
		long rows = dirty_bottom - dirty_top;
		unsigned char *texels = malloc((size_t)rows * ATLAS_SIZE * 4);

		if (texels)
		{
			long index;
			const unsigned char *coverage = atlas + (long)dirty_top * ATLAS_SIZE;
			unsigned int *fields;

			for (index = 0; index < rows * ATLAS_SIZE; index++)
			{
				texels[index * 4 + 0] = texels[index * 4 + 1] = texels[index * 4 + 2] = 255;
				texels[index * 4 + 3] = coverage[index];
			}
			/* (a blit after the draws batched before it, read from texels
			when the commands are flushed) */
			fields = xgpu_metal_command(OPENCE_RENDER_OP_UPLOAD_TEXTURE, OPENCE_RENDER_UPLOAD_TEXTURE_WORDS);
			if (fields)
			{
				memset(fields, 0, (OPENCE_RENDER_UPLOAD_TEXTURE_WORDS - 1) * sizeof(*fields));
				fields[0] = atlas_texture;
				fields[4] = (unsigned int)dirty_top;
				fields[6] = ATLAS_SIZE;
				fields[7] = (unsigned int)rows;
				fields[8] = 1;
				fields[9] = (unsigned int)(unsigned long)texels;
				fields[10] = ATLAS_SIZE * 4;
				fields[11] = (unsigned int)(rows * ATLAS_SIZE * 4);
				fields[12] = (unsigned int)(rows * ATLAS_SIZE * 4);
			}
			xgpu_metal_flush();
			free(texels);
			dirty_top = ATLAS_SIZE;
			dirty_bottom = 0;
		}
	}
	return atlas_texture;
}
