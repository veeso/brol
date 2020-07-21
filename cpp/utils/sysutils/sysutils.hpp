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

#ifndef SYSUTILS_HPP
#define SYSUTILS_HPP

namespace sysutils {

long getUptime();
unsigned long getMemorySize();
unsigned long getMemoryFree();

} //namespace sysutils

#endif
