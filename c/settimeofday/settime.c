#include <stdio.h>
#include <stdlib.h>

#include <sys/time.h>
#include <time.h>

int main(int argc, char** argv) {
  if (argc < 2) {
    printf("Usage: %s <UNIX epoch time>\n", argv[0]);
    return 1;
  }

  const time_t epoch = atoi(argv[1]);
  struct timeval t_new;
  t_new.tv_sec = epoch;
  t_new.tv_usec = 0;

  struct timezone tz;
  gettimeofday(NULL, &tz);
  return settimeofday(&t_new, &tz);
}
