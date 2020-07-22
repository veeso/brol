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

size_t roundToMultiple(const size_t toRound, const size_t multiple) {
  if (multiple == 0)
    return toRound;

  const size_t remainder = toRound % multiple;
  if (remainder == 0)
    return toRound;

  return toRound + multiple - remainder;
}

uint8_t* buffer_to_bmp(const uint8_t* buffer, const size_t bufferSize, const size_t width, const size_t height, size_t* bmp_size) {
  //Get sizes
  const size_t bitsPerPixel = 24;
  const size_t nextMultipleOf4 = roundToMultiple(width * (bitsPerPixel / 8), 4);
  const size_t paddingSize = nextMultipleOf4 - width * (size_t) (bitsPerPixel / 8);
  const size_t totalRowSize = (width * (size_t) (bitsPerPixel / 8)) + paddingSize;
  const size_t realRowSize = (width * (size_t) (bitsPerPixel / 8));
  const size_t dataSize = totalRowSize * height;
  const size_t fileSize = 54 + dataSize;
  //Reserve bmo buffer
  uint8_t* bmp_buffer = (uint8_t*) malloc(sizeof(uint8_t) * (fileSize));
  if (bmp_buffer == NULL) {
    *bmp_size = 0;
    return NULL;
  }
  *bmp_size = fileSize * sizeof(uint8_t);
  //Set bmp 24
  //Write 54 Bytes header
  //'BM'
  bmp_buffer[0] = 0x42;
  bmp_buffer[1] = 0x4D;
  //Write filesize; we need to found it first
  bmp_buffer[2] = fileSize & 255;
  bmp_buffer[3] = (fileSize >> 8) & 255;
  bmp_buffer[4] = (fileSize >> 16) & 255;
  bmp_buffer[5] = (fileSize >> 24) & 255;
  //Reserved
  bmp_buffer[6] = 0;  
  bmp_buffer[7] = 0;  
  bmp_buffer[8] = 0;
  bmp_buffer[9] = 0;
  //Data offset
  const size_t dataOffset = 54;
  bmp_buffer[10] = dataOffset & 255;
  bmp_buffer[11] = (dataOffset >> 8) & 255;
  bmp_buffer[12] = (dataOffset >> 16) & 255;
  bmp_buffer[13] = (dataOffset >> 24) & 255;
  //DibSize
  bmp_buffer[14] = 40;  
  bmp_buffer[15] = 0;  
  bmp_buffer[16] = 0;  
  bmp_buffer[17] = 0;
  //Width
  bmp_buffer[18] = width & 255;
  bmp_buffer[19] = (width >> 8) & 255;  
  bmp_buffer[20] = (width >> 16) & 255;
  bmp_buffer[21] = (width >> 24) & 255;
  //Height
  bmp_buffer[22] = height & 255;
  bmp_buffer[23] = (height >> 8) & 255;  
  bmp_buffer[24] = (height >> 16) & 255;
  bmp_buffer[25] = (height >> 24) & 255;
  //Color planes
  bmp_buffer[26] = 1;
  bmp_buffer[27] = 0;
  //Bits per pixel
  bmp_buffer[28] = bitsPerPixel & 255;
  bmp_buffer[29] = (bitsPerPixel >> 8 ) & 255;
  //biRGB
  bmp_buffer[30] = 0;  
  bmp_buffer[31] = 0;  
  bmp_buffer[32] = 0;  
  bmp_buffer[33] = 0;
  //Data size
  bmp_buffer[34] = (dataSize & 255);
  bmp_buffer[35] = (dataSize >> 8) & 255;  
  bmp_buffer[36] = (dataSize >> 16) & 255;
  bmp_buffer[37] = (dataSize >> 24) & 255;
  //Print size Width
  bmp_buffer[38] = 0;
  bmp_buffer[39] = 0; 
  bmp_buffer[40] = 0;
  bmp_buffer[41] = 0;
  //Print size Height
  bmp_buffer[42] = 0;
  bmp_buffer[43] = 0;  
  bmp_buffer[44] = 0;
  bmp_buffer[45] = 0;
  //Palette
  bmp_buffer[46] = 0;  
  bmp_buffer[47] = 0;  
  bmp_buffer[48] = 0;  
  bmp_buffer[49] = 0;
  //Important colors
  bmp_buffer[50] = 0;  
  bmp_buffer[51] = 0;  
  bmp_buffer[52] = 0;  
  bmp_buffer[53] = 0;
  //Store pixels
  int rowPositionCounter = 0;
  int px_index = (int) (bufferSize - (width * 3)); //Last row, left
  size_t buffRowPositionCounter = 0;
  for (size_t dataPtr = dataOffset; dataPtr < fileSize - 1 && px_index >= 0;) {
    bmp_buffer[dataPtr++] = buffer[px_index + 2]; //B
    bmp_buffer[dataPtr++] = buffer[px_index + 1]; //G
    bmp_buffer[dataPtr++] = buffer[px_index]; //R
    px_index += 3;
    buffRowPositionCounter++;
    if (buffRowPositionCounter == width) {
      px_index -= (width * 2 * 3);
      buffRowPositionCounter = 0;
    }
    rowPositionCounter += 3;
    if (rowPositionCounter >= realRowSize) {
      rowPositionCounter = 0;
      for (size_t i = 0; i < paddingSize; i++) {
        bmp_buffer[dataPtr++] = 0;
      }
    }
  }
  return bmp_buffer;
}


int main(int argc, char** argv) {

  if (argc < 3) {
    printf("Usage: %s <device> <outfile>\n", argv[0]);
    return 255;
  }
  int rc = 0;
  const char* device = argv[1];
  const char* outfile = argv[2];
  //Instantiate framebuffer
  Framebuffer* fb = NULL;
  FB_Error error;
  //RGB
  uint8_t* rgb_buffer = NULL;
  size_t rgb_buffer_size = 0;
  //BMP
  size_t bmp_size;
  uint8_t* out_bmp = NULL;
  //File
  FILE* out_file_ptr = NULL;
  //Init
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
  //Dump
  rgb_buffer_size = fb->width * fb->height * 3;
  rgb_buffer = (uint8_t*) malloc(sizeof(uint8_t) * rgb_buffer_size);
  if (rgb_buffer == NULL) {
    printf("Could not allocate RGB buffer\n");
    rc = 1;
    goto cleanup;
  }
  for (size_t y = 0; y < fb->height; y++) {
    for (size_t x = 0; x < fb->width; x++) {
      FramebufferColor* color = NULL;
      if ((error = framebuffer_read(fb, x, y, &color)) != FRAMEBUFFER_ERROR_SUCCESS) {
        color_cleanup(color);
        rc = 1;
        printf("Could not read from framebuffer at (%lu, %lu): %s\n", x, y, framebuffer_get_error_desc(error));
        goto cleanup;
      }
      //Push color to rgb buffer
      size_t index = ((fb->width * y) + x) * 3;
      switch (color->syntax) {
        case Color_RGB16: {
          FramebufferColorRGB16* color_ptr = (FramebufferColorRGB16*) color->color_ptr;
          rgb_buffer[index++] = color_ptr->red;
          rgb_buffer[index++] = color_ptr->green;
          rgb_buffer[index++] = color_ptr->blue;
          break;
        }
        case Color_RGB24: {
          FramebufferColorRGB24* color_ptr = (FramebufferColorRGB24*) color->color_ptr;
          rgb_buffer[index++] = color_ptr->red;
          rgb_buffer[index++] = color_ptr->green;
          rgb_buffer[index++] = color_ptr->blue;
          break;
        }
        case Color_RGB32: {
          FramebufferColorRGB32* color_ptr = (FramebufferColorRGB32*) color->color_ptr;
          rgb_buffer[index++] = color_ptr->red;
          rgb_buffer[index++] = color_ptr->green;
          rgb_buffer[index++] = color_ptr->blue;
          break;
        }
        default: {
          color_cleanup(color);
          rc = 1;
          printf("Unknown depth %u\n", fb->depth);
          goto cleanup;
        }
      }
      //Cleanup color
      color_cleanup(color);
    }
  }
  //Dump to BMP
  out_bmp = buffer_to_bmp(rgb_buffer, rgb_buffer_size, fb->width, fb->height, &bmp_size);
  if (out_bmp == NULL) {
    printf("Failed to encode framebuffer to BMP\n");
    rc = 2;
    goto cleanup;
  }
  //Dump to file
  out_file_ptr = fopen(outfile, "wb");
  if (out_file_ptr == NULL) {
    printf("Could not open out file %s\n", outfile);
    rc = 3;
    goto cleanup;
  }
  for (size_t i = 0; i < bmp_size; i++) {
    if (fwrite(&out_bmp[i], sizeof(uint8_t), 1, out_file_ptr) != 1) {
      printf("Failed to write byte %lu to file %s\n", i, outfile);
      rc = 3;
      goto cleanup;
    }
  }
  fclose(out_file_ptr);
  printf("Written %lu bytes to %s\n", bmp_size, outfile);
  
cleanup:
  if (rgb_buffer != NULL) {
    free(rgb_buffer);
    rgb_buffer = NULL;
  }
  if (out_bmp != NULL) {
    free(out_bmp);
    out_bmp = NULL;
  }
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
