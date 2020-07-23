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

#ifndef TUNTAP_H
#define TUNTAP_H

#include <inttypes.h>
#include <stdlib.h>

int tun_init(char** ifname, size_t* ifname_len);
int tap_init(char** ifname, size_t* ifname_len);

int tuntap_delete(const int fd, const char* ifname);

int tuntap_read(const int fd, uint8_t** buffer , size_t* bufsize, const int timeout);
int tuntap_write(const int fd, const uint8_t* buffer, size_t bufsize, const int timeout);

#endif //TUNTAP_H
