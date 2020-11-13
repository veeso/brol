/**
 * @author Christian Visintin
 * @brief simple ctrl c handler
 */

#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include <signal.h> // Required


unsigned int sigterm_raised = 0;

void handle_sigterm(int s) {
  printf("\nRaised Signal %d\n", s);
  sigterm_raised = 1;
}

int main(int argc, char** argv) {

  //SIGTERM handler
  struct sigaction sigterm_hnd;
  sigterm_hnd.sa_handler = handle_sigterm;
  sigemptyset(&sigterm_hnd.sa_mask);
  sigterm_hnd.sa_flags = 0;
  // Handle both SIGTERM and SIGINT
  sigaction(SIGTERM, &sigterm_hnd, NULL);
  sigaction(SIGINT, &sigterm_hnd, NULL);

  printf("Press CTRL+C to stop the execution\n");
  while (!sigterm_raised) {
    usleep(1000);
  }

  return 0;
}
