/** 
 *  _____   ____    _   _   _     _   _
 * |  ___| / ___|  | | | | | |_  (_) | |  ___
 * | |_    \___ \  | | | | | __| | | | | / __|
 * |  _|    ___) | | |_| | | |_  | | | | \__ \
 * |_|     |____/   \___/   \__| |_| |_| |___/
 *  
 * Author: Christian Visintin
**/

#include "fsutils.hpp"

#include <fstream>
#include <system_error>

#include <utime.h>

//If c++17, use filesystem, otherwise use the old stat method
#if __cplusplus >= 201703L
#include <filesystem>
#define WITH_CPP_FILESYSTEM 1
#else
#include <dirent.h>
#include <sys/stat.h>
#include <unistd.h>
#endif //C++17

namespace fsutils {

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

/**
 * @function fileExists
 * @description check if provided file exists
 * @param const std::string& file
 * @returns bool
 */

bool fileExists(const std::string& file) {
#ifdef WITH_CPP_FILESYSTEM
  return std::filesystem::exists(file);
#else
  struct stat buffer;
  return (stat (file.c_str(), &buffer) == 0);
#endif //WITH_CPP_FILESYSTEM
}

/**
 * @brief remove file
 * @param string file
 * @return bool
 */

bool removeFile(const std::string& filename) {
#ifdef WITH_CPP_FILESYSTEM
  return std::filesystem::remove(filename);
#else
  return unlink(filename.c_str()) == 0;
#endif //WITH_CPP_FILESYSTEM
}

/**
 * @brief create a symlink
 * @param string link name
 * @param string target path
 * @returns bool
 */

bool createSymlink(const std::string& link, const std::string& target) {
#ifdef WITH_CPP_FILESYSTEM
  return std::filesystem::create_symlink(target, link);
#else
  return symlink(target.c_str(), link.c_str()) == 0;
#endif //WITH_CPP_FILESYSTEM
}

/**
 * @brief touch file
 * @param string file
 * @returns bool
 */

bool touchFile(const std::string& file) {
  const time_t tNow = std::chrono::duration_cast<std::chrono::seconds>(std::chrono::system_clock::now().time_since_epoch()).count();
  if (!fsutils::fileExists(file)) {
    std::ofstream fileStream;
    fileStream.open(file);
    if (!fileStream.is_open()) {
      return false;
    }
    fileStream << "\n";
    fileStream.close();
  }
  struct stat runfile_stat;
  struct utimbuf new_times;
  stat(file.c_str(), &runfile_stat);
  new_times.actime = runfile_stat.st_atime;
  new_times.modtime = tNow;
  return utime(file.c_str(), &new_times) == 0;
}

/**
 * @function getLastFileChange
 * @description returns the epoch time of the last change on a file
 * @param const std::string& filename
 * @returns time_t
 * @throws SystemError
 */

time_t getLastFileChange(const std::string& filename) {
#ifdef WITH_CPP_FILESYSTEM
  return std::filesystem::last_write_time(filename);
#else
  struct stat result;
  if(stat(filename.c_str(), &result) == 0) {
    time_t lastChangeTime = result.st_mtime;
    return lastChangeTime;
  } else {
    const std::string errDesc = "No such file or directory: " + filename;
    throw std::system_error(std::make_error_code(std::errc::no_such_file_or_directory), errDesc.c_str());
  }
#endif //WITH_CPP_FILESYSTEM
}

} //fsutils

