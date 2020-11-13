/**
 * @author Christian Visintin
 * @brief print progress bar in C
 */

#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>

#include <unistd.h>

/**
 * @brief print progress bar to stdout
 * @param it - current iteration (<= max)
 * @param max - max iteration (e.g. 100)
 * @param prefix
 * @param suffix
 * @param fill_char
 * @param empty_char
 */

void print_progress_bar(size_t it, size_t max, const char* prefix, const char* suffix, const char fill_char, const char empty_char) {
  // The old good maths; it : x = max: 100
  const float percentage = (((float) it * 100) / (float) max);
  // Fill bar
  const size_t progress_bar_len = 101;
  char progress_bar[101];
  for (int i = 0; i < 100; i++) {
    if (i <= (int) percentage) {
      progress_bar[i] = fill_char; // Block
    } else {
      progress_bar[i] = empty_char; // Empty
    }
  }
  progress_bar[100] = 0x00; // Null terminate
  const size_t max_progress_bar_len = 512;
  char progress_bar_fmt[512];
  // Fmt
  snprintf(progress_bar_fmt, max_progress_bar_len, "%s [%s] %.2f%% %s", (prefix ? prefix : ""), progress_bar, percentage, (suffix ? suffix : ""));
  printf("\r%s", progress_bar_fmt);
  if (it >= max) { // New line if ended
    printf("\n");
  }
}

int main(int argc, char** argv) {

  const size_t max = 4096;
  for (size_t i = 0; i <= max; i++) {
    print_progress_bar(i, max, "Loading...", "Please, wait", '#', ' ');
    usleep(500); // 0.5ms
  }
  return 0;
}
