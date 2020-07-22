/**
 *   EasyFB
 *   Developed by Christian Visintin
 * 
 * MIT License
 * Copyright (c) 2020 Christian Visintin
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
**/

#ifndef FRAMEBUFFER_H
#define FRAMEBUFFER_H

#include "fbcolors.h"

#include <inttypes.h>
#include <stdlib.h>

#include <linux/fb.h>

typedef struct Framebuffer {
  int fd;
  struct fb_var_screeninfo vinfo;
  uint8_t* area;
  size_t size;
  unsigned int width;
  unsigned int height;
  unsigned int depth;
  unsigned int x_offset;
  unsigned int y_offset;
  unsigned int alpha;
  unsigned int rbswap;
} Framebuffer;

typedef enum FB_Error {
  FRAMEBUFFER_ERROR_SUCCESS,
  FRAMEBUFFER_ERROR_UNINITIALIZED,
  FRAMEBUFFER_ERROR_OPEN,
  FRAMEBUFFER_ERROR_CLOSE,
  FRAMEBUFFER_ERROR_IS_CLOSED,
  FRAMEBUFFER_ERROR_IO,
  FRAMEBUFFER_ERROR_OUT_OF_BOUNDS,
  FRAMEBUFFER_ERROR_INVALID_COLOR,
  FRAMEBUFFER_BAD_ALLOC
} FB_Error;

//Constructor/Destructor
FB_Error framebuffer_init(Framebuffer** fb, const unsigned int rbswap);
FB_Error framebuffer_cleanup(Framebuffer* fb);

//Config
FB_Error framebuffer_set_width(Framebuffer* fb, const size_t width);
FB_Error framebuffer_set_height(Framebuffer* fb, const size_t height);
FB_Error framebuffer_set_xoffset(Framebuffer* fb, const size_t x_offset);
FB_Error framebuffer_set_yoffset(Framebuffer* fb, const size_t y_offset);

//Open/Close
FB_Error framebuffer_open(Framebuffer* fb, const char* dev);
FB_Error framebuffer_close(Framebuffer* fb);
FB_Error framebuffer_isopen(Framebuffer* fb);

//I/O
FB_Error framebuffer_write(Framebuffer* fb, const size_t x, const size_t y, FramebufferColor* color);
FB_Error framebuffer_write_area(Framebuffer* fb, const size_t x, const size_t y, const size_t w, const size_t h, const void* img, FB_Error (*read_img)(FramebufferColor** color, const void* img, const size_t x, const size_t y));
FB_Error framebuffer_read(Framebuffer* fb, const size_t x, const size_t y, FramebufferColor** px);
FB_Error framebuffer_clear(Framebuffer* fb);

//Errors
const char* framebuffer_get_error_desc(const FB_Error error);

#endif //FRAMEBUFFER_H
