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

#include <easyfb/framebuffer.h>

#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

#ifndef MAX
#define MAX(a, b) (((a) > (b)) ? (a) : (b))
#endif
#ifndef MIN
#define MIN(a, b) (((a) < (b)) ? (a) : (b))
#endif

//Private functions
FB_Error color_init(FramebufferColor** color);
//FB_Error color_cleanup(FramebufferColor* color);
FB_Error color_write(Framebuffer* fb, const size_t x, const size_t y, const FramebufferColor* color);
FB_Error color_read(Framebuffer* fb, const size_t x, const size_t y, FramebufferColor** color);

uint8_t get_pow2(const uint8_t power);


/**
 * @brief initialize a Framebuffer structure
 * @param Framebuffer** fb
 * @param unsigned rbswap
 * @return FB_Error
 */

FB_Error framebuffer_init(Framebuffer** fb, const unsigned int rbswap) {
  *fb = (Framebuffer*) malloc(sizeof(Framebuffer));
  if (*fb == NULL) {
    return FRAMEBUFFER_BAD_ALLOC;
  }
  (*fb)->rbswap = rbswap;
  (*fb)->fd = -1;
  (*fb)->width = 0;
  (*fb)->height = 0;
  (*fb)->x_offset = 0;
  (*fb)->y_offset = 0;
  (*fb)->alpha = 0;
  (*fb)->area = NULL;
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief clean up Framebuffer structure
 * @param Framebuffer*
 * @return FB_Error
 */

FB_Error framebuffer_cleanup(Framebuffer* fb) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  //If framebuffer is open, close it first
  if (framebuffer_isopen(fb) != FRAMEBUFFER_ERROR_IS_CLOSED) {
    FB_Error err;
    if ((err = framebuffer_close(fb)) != FRAMEBUFFER_ERROR_SUCCESS) {
      return err;
    }
  }
  free(fb);
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief set the   used by the framebuffer
 * @param Framebuffer*
 * @param size_t
 * @return FB_error
 */

FB_Error framebuffer_set_width(Framebuffer* fb, const size_t width) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  fb->width = width;
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief set the   used by the framebuffer
 * @param Framebuffer*
 * @param size_t
 * @return FB_error
 */

FB_Error framebuffer_set_height(Framebuffer* fb, const size_t height) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  fb->height = height;
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief set the   used by the framebuffer
 * @param Framebuffer*
 * @param size_t
 * @return FB_error
 */

FB_Error framebuffer_set_xoffset(Framebuffer* fb, const size_t x_offset) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  fb->x_offset = x_offset;
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief set the   used by the framebuffer
 * @param Framebuffer*
 * @param size_t
 * @return FB_error
 */

FB_Error framebuffer_set_yoffset(Framebuffer* fb, const size_t y_offset) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  fb->y_offset = y_offset;
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief
 * @param Framebuffer*
 * @param char+ device
 * @return FB_Error
 */

FB_Error framebuffer_open(Framebuffer* fb, const char* dev) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }

  //Open framebuffer
  if ((fb->fd = open(dev, O_RDWR)) > 2)  {
    if (ioctl(fb->fd, FBIOGET_VSCREENINFO, &fb->vinfo) >= 0) {
      fb->size = (size_t) (fb->vinfo.xres_virtual * fb->vinfo.yres_virtual * (fb->vinfo.bits_per_pixel / 8));
      fb->depth = fb->vinfo.bits_per_pixel;
      fb->area = (uint8_t*) mmap(0, fb->size, PROT_READ | PROT_WRITE, MAP_SHARED, fb->fd, 0);
      if (fb->width == 0) {
        fb->width = fb->vinfo.xres;
      }
      if (fb->height == 0) {
        fb->height = fb->vinfo.yres;
      }
      fb->alpha = (fb->vinfo.transp.length > 0) ? 1 : 0;
    } else {
      framebuffer_close(fb);
      return FRAMEBUFFER_ERROR_OPEN;
    }
  } else {
    return FRAMEBUFFER_ERROR_OPEN;
  }
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief
 * @param Framebuffer*
 * @return FB_Error
 */

FB_Error framebuffer_close(Framebuffer* fb) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  if (framebuffer_isopen(fb) != FRAMEBUFFER_ERROR_SUCCESS) {
    return FRAMEBUFFER_ERROR_IS_CLOSED;
  }
  //Try to close fb
  if (close(fb->fd) == -1) {
    return FRAMEBUFFER_ERROR_CLOSE;
  }
  //Unmap area
  if (fb->area != NULL) {
    munmap(fb->area, fb->size);
  }
  //Reset parameters
  fb->fd = -1;
  fb->width = 0;
  fb->height = 0;
  fb->x_offset = 0;
  fb->y_offset = 0;
  fb->area = NULL;
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief
 * @param Framebuffer*
 * @return FB_Error (Success if open, FRAMEBUFFER_ERROR_IS_CLOSED if closed)
 */

FB_Error framebuffer_isopen(Framebuffer* fb) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  return fb->fd == -1 ? FRAMEBUFFER_ERROR_IS_CLOSED : FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief
 * @param Framebuffer*
 * @param size_t x
 * @param size_t y
 * @param FramebufferColor* color NOTE: color size must match bytes per pixel
 * @return FB_Error
 */

FB_Error framebuffer_write(Framebuffer* fb, const size_t x, const size_t y, FramebufferColor* color) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  if (framebuffer_isopen(fb) != FRAMEBUFFER_ERROR_SUCCESS) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  if (color == NULL) {
    return FRAMEBUFFER_ERROR_INVALID_COLOR;
  }
  return color_write(fb, x, y, color);
}

/**
 * @brief
 * @param Framebuffer*
 * @param size_t x
 * @param size_t y
 * @param size_t w
 * @param size_t h
 * @param void* img
 * @param FramebufferColor* (*read_img)(void* img, const size_t x, const size_t y) NOTE: function to read pixel data at. Color must be instantiated by the calling function
 * @return FB_Error
 */

FB_Error framebuffer_write_area(Framebuffer* fb, const size_t x, const size_t y, const size_t w, const size_t h, const void* img, FB_Error (*read_img)(FramebufferColor** color, const void* img, const size_t x, const size_t y)) {
  if (fb == NULL) {
    return FRAMEBUFFER_ERROR_UNINITIALIZED;
  }
  if (framebuffer_isopen(fb) != FRAMEBUFFER_ERROR_SUCCESS) {
    return FRAMEBUFFER_ERROR_IS_CLOSED;
  }
  FramebufferColor* color = NULL;
  FB_Error err;
  //Write area iterating over coordinates
  for (size_t i = x; i < w; i++) {
    for (size_t j = y; i < y; j++) {
      //Read pixel data at i,j
      if ((err = read_img(&color, img, i, j)) != FRAMEBUFFER_ERROR_SUCCESS) {
        color_cleanup(color);
        return err;
      }
      //Write color
      if ((err = color_write(fb, i, j, color)) != FRAMEBUFFER_ERROR_SUCCESS) {
        color_cleanup(color);
        return err;
      }
      //Clean color
      color_cleanup(color);
      color = NULL;
    }
  }
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief Read a pixel from the framebuffer
 * @param Framebuffer*
 * @param
 * @return FB_Error
 */

FB_Error framebuffer_read(Framebuffer* fb, const size_t x, const size_t y, FramebufferColor** px) {
  FB_Error err;
  if ((err = framebuffer_isopen(fb)) != FRAMEBUFFER_ERROR_SUCCESS) {
    return err;
  }
  //Allocate px
  return color_read(fb, x, y, px);
}

/**
 * @brief clear frame buffer
 * @param Framebuffer*
 * @return FB_Error
 */

FB_Error framebuffer_clear(Framebuffer* fb) {
  FB_Error err;
  if ((err = framebuffer_isopen(fb)) != FRAMEBUFFER_ERROR_SUCCESS) {
    return err;
  }
  FramebufferColor* color = NULL;
  if ((err = color_init(&color)) != FRAMEBUFFER_ERROR_SUCCESS) {
    return err;
  }
  switch (fb->depth) {
    case 16: {
      FramebufferColorRGB16* ptr = (FramebufferColorRGB16*) malloc(sizeof(FramebufferColorRGB16));
      ptr->blue = 0x00;
      ptr->green = 0x00;
      ptr->red = 0x00;
      color->color_ptr = (void*) ptr;
      color->syntax = Color_RGB16;
      break;
    }
    case 24: {
      FramebufferColorRGB24* ptr = (FramebufferColorRGB24*) malloc(sizeof(FramebufferColorRGB24));
      ptr->blue = 0x00;
      ptr->green = 0x00;
      ptr->red = 0x00;
      color->color_ptr = (void*) ptr;
      color->syntax = Color_RGB24;
      break;
    }
    case 32: {
      FramebufferColorRGB32* ptr = (FramebufferColorRGB32*) malloc(sizeof(FramebufferColorRGB32));
      ptr->alpha = 0x00;
      ptr->blue = 0x00;
      ptr->green = 0x00;
      ptr->red = 0x00;
      color->color_ptr = (void*) ptr;
      color->syntax = Color_RGB32;
      break;
    }
  }

  for (size_t y = 0; y < fb->height; y++) {
    for (size_t x = 0; x < fb->width; x++) {
      if ((err = framebuffer_write(fb, x, y, color)) != FRAMEBUFFER_ERROR_SUCCESS) {
        return err;
      }
    }
  }
  color_cleanup(color);
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief Initialize a FramebufferColor
 * @param FramebufferColor**
 * @return FB_Error
 */

FB_Error color_init(FramebufferColor** color) {
  (*color) = (FramebufferColor*) malloc(sizeof(FramebufferColor));
  if ((*color) == NULL) {
    return FRAMEBUFFER_BAD_ALLOC;
  }
  (*color)->color_ptr = NULL;
  (*color)->syntax = Color_None;
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief Cleanup a Framebuffer Color
 * @param FramebufferColor**
 * @return FB_Error
 */

void color_cleanup(FramebufferColor* color) {
  if (color == NULL) {
    return;
  }
  if (color->color_ptr) {
    switch (color->syntax) {
      case Color_None: {
        free(color->color_ptr);
        break;
      }
      case Color_RGB24: {
        FramebufferColorRGB24* color_ptr = (FramebufferColorRGB24*) color->color_ptr;
        free(color_ptr);
        break;
      }
      case Color_RGB32: {
        FramebufferColorRGB32* color_ptr = (FramebufferColorRGB32*) color->color_ptr;
        free(color_ptr);
        break;
      }
    }
  }
  free(color);
}

/**
 * @brief 
 * @param Framebuffer*
 * @param size_t x
 * @param size_t y
 * @param FramebufferColor*
 * @return FB_Error
 */

FB_Error color_write(Framebuffer* fb, const size_t x, const size_t y, const FramebufferColor* color) {
  //Switch over color syntax
  const size_t index = ((fb->width * y) + x) * (fb->depth / 8);
  if (index >= fb->size) {
    return FRAMEBUFFER_ERROR_OUT_OF_BOUNDS;
  }
  switch (color->syntax) {
    case Color_None: {
      return FRAMEBUFFER_ERROR_INVALID_COLOR;
    }
    case Color_RGB16: {
      FramebufferColorRGB16* color_ptr = (FramebufferColorRGB16*) color->color_ptr;
      //printf("%d; %d; %d\n", fb->vinfo.red.offset, fb->vinfo.green.offset, fb->vinfo.blue.offset);
      uint16_t color_word = 0;
      //Sort positions
      uint8_t color_offsets[3];
      color_offsets[0] = MIN(fb->vinfo.red.offset, MIN(fb->vinfo.green.offset, fb->vinfo.blue.offset));
      color_offsets[2] = MAX(fb->vinfo.red.offset, MAX(fb->vinfo.green.offset, fb->vinfo.blue.offset));
      //Get middle value
      if (fb->vinfo.green.offset != color_offsets[0] && fb->vinfo.green.offset != color_offsets[2]) {
        color_offsets[1] = fb->vinfo.green.offset;
      } else if (fb->vinfo.blue.offset != color_offsets[0] && fb->vinfo.blue.offset != color_offsets[2]) {
        color_offsets[1] = fb->vinfo.blue.offset;
      } else {
        color_offsets[1] = fb->vinfo.red.offset;
      }
      //Assign color word
      color_word = ((color_offsets[2] == fb->vinfo.red.offset ? (color_ptr->red) : (color_offsets[2] == fb->vinfo.blue.offset ? (color_ptr->blue) : (color_ptr->green))) & (get_pow2(16 - color_offsets[2]) - 1)) << color_offsets[2];
      color_word += ((color_offsets[1] == fb->vinfo.red.offset ? (color_ptr->red) : (color_offsets[1] == fb->vinfo.blue.offset ? (color_ptr->blue) : (color_ptr->green))) & (get_pow2(color_offsets[2] - color_offsets[1]) - 1)) << color_offsets[1];
      color_word += ((color_offsets[0] == fb->vinfo.red.offset ? (color_ptr->red) : (color_offsets[0] == fb->vinfo.blue.offset ? (color_ptr->blue) : (color_ptr->green))) & (get_pow2(color_offsets[1] - color_offsets[0]) - 1)) << color_offsets[0];
      fb->area[index] = color_word & 0xFF;   //LSB
      fb->area[index + 1] = color_word >> 8; //LSB 
      return FRAMEBUFFER_ERROR_SUCCESS;
    }
    case Color_RGB24: {
      FramebufferColorRGB24* color_ptr = (FramebufferColorRGB24*) color->color_ptr;
      fb->area[index + (fb->vinfo.red.offset / 8)] = fb->rbswap ? color_ptr->blue : color_ptr->red;
      fb->area[index + (fb->vinfo.green.offset / 8)] = color_ptr->green;
      fb->area[index + (fb->vinfo.blue.offset / 8)] = fb->rbswap ? color_ptr->red : color_ptr->blue;
      return FRAMEBUFFER_ERROR_SUCCESS;
    }
    case Color_RGB32: {
      FramebufferColorRGB32* color_ptr = (FramebufferColorRGB32*) color->color_ptr;
      fb->area[index + (fb->vinfo.red.offset / 8)] = fb->rbswap ? color_ptr->blue : color_ptr->red;
      fb->area[index + (fb->vinfo.green.offset / 8)] = color_ptr->green;
      fb->area[index + (fb->vinfo.blue.offset / 8)] = fb->rbswap ? color_ptr->red : color_ptr->blue;
      if (fb->alpha) {
        fb->area[index + (fb->vinfo.transp.offset / 8)] = color_ptr->alpha;
      }
      return FRAMEBUFFER_ERROR_SUCCESS;
    }
    default: {
      return FRAMEBUFFER_ERROR_INVALID_COLOR;
    }
  }
}

/**
 * @brief 
 * @param Framebuffer*
 * @param size_t x
 * @param size_t y
 * @param FramebufferColor*
 * @return FB_Error
 */

FB_Error color_read(Framebuffer* fb, const size_t x, const size_t y, FramebufferColor** color) {
  FB_Error err;
  if ((err = color_init(color)) != FRAMEBUFFER_ERROR_SUCCESS) {
    return err;
  }
  const size_t index = ((fb->width * y) + x) * (fb->depth / 8);
  if (index >= fb->size) {
    return FRAMEBUFFER_ERROR_OUT_OF_BOUNDS;
  }
  //Create color
  if (fb->depth == 16) {
    FramebufferColorRGB16* color_wrapper = (FramebufferColorRGB16*) malloc(sizeof(FramebufferColorRGB16));
    if (color_wrapper == NULL) {
      color_cleanup(*color);
      return FRAMEBUFFER_BAD_ALLOC;
    }
    uint16_t color_word = (fb->area[index + 1] << 8) + (fb->area[index]);
    //Get offsets
    uint8_t color_offsets[3];
    //Sort positions
    color_offsets[0] = MIN(fb->vinfo.red.offset, MIN(fb->vinfo.green.offset, fb->vinfo.blue.offset));
    color_offsets[2] = MAX(fb->vinfo.red.offset, MAX(fb->vinfo.green.offset, fb->vinfo.blue.offset));
    //Get middle value
    if (fb->vinfo.green.offset != color_offsets[0] && fb->vinfo.green.offset != color_offsets[2]) {
      color_offsets[1] = fb->vinfo.green.offset;
    } else if (fb->vinfo.blue.offset != color_offsets[0] && fb->vinfo.blue.offset != color_offsets[2]) {
      color_offsets[1] = fb->vinfo.blue.offset;
    } else {
      color_offsets[1] = fb->vinfo.red.offset;
    }
    color_wrapper->red = (fb->vinfo.red.offset == color_offsets[0] ? (color_word & (get_pow2(color_offsets[0]) - 1)) : (fb->vinfo.red.offset == color_offsets[2] ? (color_word >> color_offsets[2]) : ((color_word >> color_offsets[1]) & get_pow2(color_offsets[2] - color_offsets[1]))));
    color_wrapper->green = (fb->vinfo.green.offset == color_offsets[0] ? (color_word & (get_pow2(color_offsets[0]) - 1)) : (fb->vinfo.green.offset == color_offsets[2] ? (color_word >> color_offsets[2]) : ((color_word >> color_offsets[1]) & get_pow2(color_offsets[2] - color_offsets[1]))));
    color_wrapper->blue = (fb->vinfo.blue.offset == color_offsets[0] ? (color_word & (get_pow2(color_offsets[0]) - 1)) : (fb->vinfo.blue.offset == color_offsets[2] ? (color_word >> color_offsets[2]) : ((color_word >> color_offsets[1]) & get_pow2(color_offsets[2] - color_offsets[1]))));
    (*color)->syntax = Color_RGB16;
    (*color)->color_ptr = (void*) color_wrapper;
  }
  else if (fb->depth == 24) {
    FramebufferColorRGB24* color_wrapper = (FramebufferColorRGB24*) malloc(sizeof(FramebufferColorRGB24));
    if (color_wrapper == NULL) {
      color_cleanup(*color);
      return FRAMEBUFFER_BAD_ALLOC;
    }
    color_wrapper->red = fb->area[fb->rbswap ? (index + (fb->vinfo.blue.offset / 8)) : (index + (fb->vinfo.red.offset / 8))];
    color_wrapper->green = fb->area[index + (fb->vinfo.green.offset / 8)];
    color_wrapper->blue = fb->area[fb->rbswap ? (index + (fb->vinfo.red.offset / 8)) : (index + (fb->vinfo.blue.offset / 8))];
    (*color)->syntax = Color_RGB24;
    (*color)->color_ptr = (void*) color_wrapper;
  } else if (fb->depth == 32) {
    FramebufferColorRGB32* color_wrapper = (FramebufferColorRGB32*) malloc(sizeof(FramebufferColorRGB32));
    if (color_wrapper == NULL) {
      color_cleanup(*color);
      return FRAMEBUFFER_BAD_ALLOC;
    }
    color_wrapper->red = fb->area[fb->rbswap ? (index + (fb->vinfo.blue.offset / 8)) : (index + (fb->vinfo.red.offset / 8))];
    color_wrapper->green = fb->area[index + (fb->vinfo.green.offset / 8)];
    color_wrapper->blue = fb->area[fb->rbswap ? (index + (fb->vinfo.red.offset / 8)) : (index + (fb->vinfo.blue.offset / 8))];
    if (fb->alpha) {
      color_wrapper->alpha = fb->area[index + (fb->vinfo.transp.offset / 8)];
    } else {
      color_wrapper->alpha = 255;
    }
    (*color)->syntax = Color_RGB32;
    (*color)->color_ptr = (void*) color_wrapper;
  }
  return FRAMEBUFFER_ERROR_SUCCESS;
}

/**
 * @brief get error description
 * @param FB_Error
 * @return char*
 */

const char* framebuffer_get_error_desc(const FB_Error error) {
  switch (error) {
    case FRAMEBUFFER_BAD_ALLOC:
      return "Could not allocate more memory in the heap";
    case FRAMEBUFFER_ERROR_CLOSE:
      return "Could not close frame buffer";
    case FRAMEBUFFER_ERROR_INVALID_COLOR:
      return "The color provided is not valid";
    case FRAMEBUFFER_ERROR_IO:
      return "I/O Error";
    case FRAMEBUFFER_ERROR_IS_CLOSED:
      return "Framebuffer is not closed";
    case FRAMEBUFFER_ERROR_OPEN:
      return "Could not open framebuffer";
    case FRAMEBUFFER_ERROR_OUT_OF_BOUNDS:
      return "Attempted to write out of framebuffer bounds";
    case FRAMEBUFFER_ERROR_SUCCESS:
      return "Not an error";
    case FRAMEBUFFER_ERROR_UNINITIALIZED:
      return "Tried to use framebuffer structure before initialization";
    default:
      return "Uknown error";
  }
}

uint8_t get_pow2(const uint8_t power) {
  uint8_t res = 2;
  if (power == 0) {
    return 1;
  }
  for (uint8_t i = 1; i < power; i++) {
    res = res * 2;
  }
  return res;
}
