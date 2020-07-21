/** 
 *  ____                  _   _   _     _   _
 * / ___|   _   _   ___  | | | | | |_  (_) | |  ___
 * \___ \  | | | | / __| | | | | | __| | | | | / __|
 *  ___) | | |_| | \__ \ | |_| | | |_  | | | | \__ \
 * |____/   \__, | |___/  \___/   \__| |_| |_| |___/
 *          |___/
 * 
 * Author: Christian Visintin
**/

#include "sysutils.hpp"


#include <system_error>

#include <sys/sysinfo.h>

namespace sysutils {

/**
 * @function getUptime
 * @description get uptime
 * @returns long
 */

long getUptime() {
  struct sysinfo info;
  sysinfo(&info);
  return info.uptime;
}

/**
 * @function getMemorySize
 * @description get RAM memory size
 * @returns unsigned long
 */

unsigned long getMemorySize() {
  struct sysinfo info;
  sysinfo(&info);
  return info.totalram;
}

/**
 * @function getMemoryFree
 * @description get the free RAM memory
 * @returns unsigned long long
 */

unsigned long getMemoryFree() {
  struct sysinfo info;
  sysinfo(&info);
  return info.freeram;
}

}
