/**
 *  _____                      __  _____
 * |_   _|  _   _   _ __      / / |_   _|   __ _   _ __
 *   | |   | | | | | '_ \    / /    | |    / _` | | '_ \
 *   | |   | |_| | | | | |  / /     | |   | (_| | | |_) |
 *   |_|    \__,_| |_| |_| /_/      |_|    \__,_| | .__/
 *                                                |_|
 * 
 * Written by Christian Visintin
*/

#include "tun_tap.h"

#ifndef __USE_MISC //Required for net/if on some operating systems
#define __USE_MISC
#endif

#include <net/if.h>
#include <fcntl.h>
#include <poll.h>
#include <string.h>
#include <sys/errno.h>
#include <sys/ioctl.h>
#include <unistd.h>

#ifdef __gnu_linux__
#include <linux/if_tun.h>
#endif // GNU Linux

/**
 * @brief initialize a new TUN interface
 * @param ifname
 * @param ifname_length
 * @return fd
 */

int tun_init(char** ifname, size_t* ifname_len) {
  //Create interface
  const char* file = "/dev/net/tun";
  struct ifreq ifr;
  int fd;
  memset(&ifr, 0, sizeof(ifr));

  ifr.ifr_flags = IFF_TUN | IFF_NO_PI;
  if ((fd = open(file, O_RDWR)) < 0) {
    return -1;
  }
  if (ioctl(fd, TUNSETIFF, (void*)&ifr) < 0) {
    return -1;
  }
  close(fd);
  *ifname_len = strlen(ifr.ifr_name) + 1;
  (*ifname) = (char*) malloc(sizeof(char) * (*ifname_len));
  if (*ifname == NULL) {
    return 1;
  }
  strcpy(*ifname, ifr.ifr_name);
  //Open interface
  int err;
  if ((fd = open("/dev/net/tun", O_RDWR)) < 0) {
    return fd;
  }
  memset(&ifr, 0, sizeof(ifr));
  ifr.ifr_flags = IFF_TUN | IFF_NO_PI;

  if ((err = ioctl(fd, TUNSETIFF, (void*)&ifr)) < 0) {
    close(fd);
    return err;
  }
  //Set NONBLOCK
  if(fcntl(fd, F_SETFL,O_NONBLOCK) < 0) {
    close(fd);
    return -1;
  }
  return fd;
}

/**
 * @brief initialize a new TAP interface
 * @param ifname
 * @param ifname_length
 * @return fd
 */

int tap_init(char** ifname, size_t* ifname_len) {
  const char* file = "/dev/net/tun";
  struct ifreq ifr;
  int fd;
  memset(&ifr, 0, sizeof(ifr));

  ifr.ifr_flags = IFF_TAP | IFF_NO_PI;

  if ((fd = open(file, O_RDWR)) < 0) {
    return -1;
  }
  if (ioctl(fd, TUNSETIFF, (void*)&ifr) < 0) {
    return -1;
  }
  close(fd);

  *ifname_len = strlen(ifr.ifr_name) + 1;
  (*ifname) = (char*) malloc(sizeof(char) * (*ifname_len));
  if (*ifname == NULL) {
    return 1;
  }
  strcpy(*ifname, ifr.ifr_name);

  //Open interface
  int err;

  if ((fd = open("/dev/net/tun", O_RDWR)) < 0) {
    return fd;
  }
  memset(&ifr, 0, sizeof(ifr));
  ifr.ifr_flags = IFF_TAP | IFF_NO_PI;

  if ((err = ioctl(fd, TUNSETIFF, (void*)&ifr)) < 0) {
    close(fd);
    return err;
  }
  //Set NONBLOCK
  if(fcntl(fd, F_SETFL,O_NONBLOCK) < 0) {
    close(fd);
    return -1;
  }

  return fd;
}


/**
 * @brief delete previously created TAP/TUN
 * @param fd
 * @param ifname
 * @return int
 */

int tuntap_delete(const int fd, const char* ifname) {

  //Close fd
  close(fd);

  //Delete fd
  const char* file = "/dev/net/tun";
  struct ifreq ifr;
  int fd2;
  memset(&ifr, 0, sizeof(ifr));
  strncpy(ifr.ifr_name, ifname, IFNAMSIZ - 1);
  ifr.ifr_flags = IFF_TAP | IFF_NO_PI;
  if ((fd2 = open(file, O_RDWR)) < 0) {
    return 1;
  }

  if (ioctl(fd, TUNSETPERSIST, 0) < 0) {
    return 1;
  }

  close(fd2);
  return 0;
}

/**
 * @brief read from TUN/TAP
 * @param buffer
 * @param bufsize
 * @param timeout
 * @return 0 if success
 */

int tuntap_read(const int fd, uint8_t** buffer , size_t* bufsize, const int timeout) {
  struct pollfd fds[1];
  fds[0].fd = fd;
  fds[0].events = POLLIN | POLLRDBAND | POLLHUP;
  int time_elapsed = 0;
  const int poll_time = 50;
  int ret;
  int rc = 0;
  //Poll FIFO
  while (time_elapsed < timeout) {
    ret = poll(fds, 1, poll_time);
    if (ret > 0) {
      // Fifo is available to be read
      if ((fds[0].revents & POLLIN) || (fds[0].revents & POLLRDBAND)) {
        //Read from FIFO
        uint8_t tmp[2048];
        const size_t bytes_read = read(fds[0].fd, tmp, 2048);
        if (bytes_read == -1) {
          if (errno == EAGAIN) { //No more data available
          //Break if no data is available (only if data is null, otherwise keep waiting)
            if (*buffer == NULL) {
              time_elapsed += poll_time; //Sum time only if no data was received (in order to prevent data cut)
              continue; //Keep waiting for data
            } else {
              break; //Exit
            }
          }
          rc = -1;
          break;
        }
        //Copy data to data
        //Track current data index
        const size_t curr_data_ptr = *bufsize;
        //Increment data size of bytes read
        *bufsize += bytes_read;
        *buffer = (uint8_t*) realloc(*buffer, sizeof(uint8_t) * *bufsize);
        if (*buffer == NULL) { //Bad alloc
          rc = -1;
          break;
        }
        //Copy new data to data buffer
        memcpy(*buffer + curr_data_ptr, buffer, bytes_read);
        //Keep iterating
      } else if (fds[0].revents & POLLERR) {
        //FIFO is in error state
        rc = -1;
        break;
      } else if (fds[0].revents & POLLHUP) {
        //Break if no data is available (only if data is null, otherwise keep waiting)
        if (*buffer == NULL) {
          time_elapsed += poll_time; //Sum time only if no data was received (in order to prevent data cut)
          continue; //Keep waiting for data
        } else {
          break; //Exit
        }
      }
    } else if (ret == 0) {
      //Break if no data is available (only if data is null, otherwise keep waiting)
      if (*buffer == NULL) {
        time_elapsed += poll_time; //Sum time only if no data was received (in order to prevent data cut)
        continue; //Keep waiting for data
      } else {
        break; //Exit
      }
    } else { //Ret == -1
      if (errno == EAGAIN) {
        //Break if no data is available (only if data is null, otherwise keep waiting)
        if (*buffer == NULL) {
          time_elapsed += poll_time; //Sum time only if no data was received (in order to prevent data cut)
          continue; //Keep waiting for data
        } else {
          break; //Exit
        }
      } else {
        //Set error state
        rc = -1;
      }
      break;
    }
  }
  return rc;
}

/**
 * @brief write to TUN/TAP
 * @param buffer
 * @param bufsize
 * @return 0 if success
 */

int tuntap_write(const int fd, const uint8_t* buffer, size_t bufsize, const int timeout) {
  struct pollfd fds[1];
  int ret;
  int rc = -1;
  size_t total_bytes_written = 0; //Must be == data_size to succeed
  //Open FIFO
  time_t elapsed_time = 0;
  fds[0].fd = fd;
  fds[0].events = POLLOUT;
  //Poll FIFO
  while (total_bytes_written < bufsize) {
    ret = poll(fds, 1, timeout);
    if (ret > 0) {
      // Fifo is available to be written
      if (fds[0].revents & POLLOUT) {
        //Write data to FIFO
        const size_t remaining_bytes = bufsize - total_bytes_written;
        //It's not obvious the data will be written in one shot, so just in case sum total_bytea_written to buffer index and write only remaining bytes
        const size_t bytes_written = write(fds[0].fd, buffer + total_bytes_written, remaining_bytes);
        //Then sum bytes written to total bytes written
        total_bytes_written += bytes_written;
      }
    } else {
      //Could not write or nobody was listening
      rc = -1;
    }
  }
  close(fds[0].fd);
  if (total_bytes_written == bufsize) {
    rc = 0;
  }
  return rc;
}
