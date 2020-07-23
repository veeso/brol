/**
 * Written by Christian Visintin
*/

#include <stdlib.h>
#include <stdio.h>

#include <sys/time.h>
#include <unistd.h>

int main(int argc, char** argv) {

  struct timeval t0, tdelta, tdiff;

  gettimeofday(&t0, NULL);
  usleep(50000); //50ms
  gettimeofday(&tdelta, NULL);
  timersub(&tdelta, &t0, &tdiff);

  printf("Time 0: %ld.%06ld\n", t0.tv_sec, t0.tv_usec);
  printf("Time Delta: %ld.%06ld\n", tdelta.tv_sec, tdelta.tv_usec);
  printf("Time diff: %ld.%06ld\n", tdiff.tv_sec, tdiff.tv_usec);

  return 0;
}
