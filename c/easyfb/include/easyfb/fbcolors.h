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

#include <inttypes.h>

typedef enum FramebufferColorStd {
  Color_None,
  Color_RGB16,
  Color_RGB24,
  Color_RGB32
} FramebufferColorStd;

typedef struct FramebufferColor {
  FramebufferColorStd syntax;
  void* color_ptr;
} FramebufferColor;

typedef struct FramebufferColorRGB16 {
  uint8_t red;
  uint8_t green;
  uint8_t blue;
} FramebufferColorRGB16;

typedef struct FramebufferColorRGB24 {
  uint8_t red;
  uint8_t green;
  uint8_t blue;
} FramebufferColorRGB24;

typedef struct FramebufferColorRGB32 {
  uint8_t red;
  uint8_t green;
  uint8_t blue;
  uint8_t alpha;
} FramebufferColorRGB32;

void color_cleanup(FramebufferColor* color);
