#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/mman.h>
#include <sys/ioctl.h>


#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <errno.h>
#include <inttypes.h>

#include <easyfb/framebuffer.h>

int main(int argc, char** argv) {

  if (argc < 5) {
    printf("Usage: %s <device> <red> <green> <blue>\n", argv[0]);
    return 255;
  }
  int rc = 0;
  const char* device = argv[1];
  const uint8_t red = atoi(argv[2]);
  const uint8_t green = atoi(argv[3]);
  const uint8_t blue = atoi(argv[4]);
  FramebufferColor color;
  FramebufferColorRGB24 color_ptr24;
  color_ptr24.blue = blue;
  color_ptr24.green = green;
  color_ptr24.red = red;
  FramebufferColorRGB32 color_ptr32;
  color_ptr32.blue = blue;
  color_ptr32.alpha = 255;
  color_ptr32.green = green;
  color_ptr32.red = red;
  FramebufferColorRGB16 color_ptr16;
  color_ptr16.blue = blue;
  color_ptr16.green = green;
  color_ptr16.red = red;
  //Instantiate framebuffer
  Framebuffer* fb = NULL;
  FB_Error error;
  if ((error = framebuffer_init(&fb, 0)) != FRAMEBUFFER_ERROR_SUCCESS) {
    rc = 1;
    printf("Could not initialize framebuffer: %s\n", framebuffer_get_error_desc(error));
    goto cleanup;
  }
  //Open framebuffer
  if ((error = framebuffer_open(fb, device)) != FRAMEBUFFER_ERROR_SUCCESS) {
    rc = 1;
    printf("Could not open framebuffer: %s\n", framebuffer_get_error_desc(error));
    goto cleanup;
  }
  //Assign color
  if (fb->depth == 24) {
    color.color_ptr = (void*) &color_ptr24;
    color.syntax = Color_RGB24;
  } else if (fb->depth == 32) {
    color.color_ptr = (void*) &color_ptr32;
    color.syntax = Color_RGB32;
    printf("Color depth 32\n");
  } else if (fb->depth == 16) {
    color.color_ptr = (void*) &color_ptr16;
    color.syntax = Color_RGB16;
  } else {
    printf("Unknown depth %u\n", fb->depth);
    goto cleanup;
  }
  //Write framebuffer
  for (size_t y = 0; y < fb->height; y++) {
    for (size_t x = 0; x < fb->width; x++) {
      if ((error = framebuffer_write(fb, x, y, &color)) != FRAMEBUFFER_ERROR_SUCCESS) {
        rc = 1;
        printf("Could not write pixel at (%lu, %lu): %s\n", x, y, framebuffer_get_error_desc(error));
        goto cleanup;
      }
    }
  }
  
cleanup:
  if (framebuffer_isopen(fb) == FRAMEBUFFER_ERROR_SUCCESS) {
    if ((error = framebuffer_close(fb)) != FRAMEBUFFER_ERROR_SUCCESS) {
      printf("Could not close framebuffer: %s\n", framebuffer_get_error_desc(error));
    }
  }
  if ((error = framebuffer_cleanup(fb)) != FRAMEBUFFER_ERROR_SUCCESS) {
    printf("Could not cleanup framebuffer: %s\n", framebuffer_get_error_desc(error));
  }
  return rc;
}
