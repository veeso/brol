/**
 *   ____          _         _   _
 *  / ___|   ___  | |_    __| | (_)  _ __
 * | |  _   / _ \ | __|  / _` | | | | '__|
 * | |_| | |  __/ | |_  | (_| | | | | |
 *  \____|  \___|  \__|  \__,_| |_| |_|
 * 
 * Written by Christian Visintin
 * Required C++ version: c++11
 */

#include <dirent.h>
#include <errno.h>
#include <string>
#include <vector>

/**
 * @function getDir
 * @description returns files in a directory
 * @param const std::string& directory to search in
 * @param std::vector<std::string>& files found
 * @param const std::string& match to find files
 * @returns int: amount of files found
 */

int getDir(const std::string& dir, std::vector<std::string>& files, const std::string& match) {
  DIR* dp;
  struct dirent* dirp;
  if ((dp = opendir(dir.c_str())) == NULL) {
    return errno;
  }

  files.clear();
  int entryCount = 0;

  while ((dirp = readdir(dp)) != NULL) {
    if (match == "*") {
      files.push_back(std::string(dirp->d_name));
      entryCount++;
    } else {
      //Check if file name contains match
      if (std::string(dirp->d_name).find(match) != std::string::npos) {
        files.push_back(std::string(dirp->d_name));
        entryCount++;
      }
    }
  }
  closedir(dp);
  return entryCount;
}
