/** 
 *  _____   ____    _   _   _     _   _
 * |  ___| / ___|  | | | | | |_  (_) | |  ___
 * | |_    \___ \  | | | | | __| | | | | / __|
 * |  _|    ___) | | |_| | | |_  | | | | \__ \
 * |_|     |____/   \___/   \__| |_| |_| |___/
 *  
 * Author: Christian Visintin
**/

#ifndef FSUTILS_HPP
#define FSUTILS_HPP

#include <chrono>
#include <string>
#include <vector>

namespace fsutils {

int getDir(const std::string& dir, std::vector<std::string>& files, const std::string& match);
bool fileExists(const std::string& filename);
bool removeFile(const std::string& filename);
bool createSymlink(const std::string& link, const std::string& target);
bool touchFile(const std::string& file);
time_t getLastFileChange(const std::string& filename);

} //namespace fsutils

#endif
