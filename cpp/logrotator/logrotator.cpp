/**
 *  @author: Christian Visintin
**/

#include <logrotator.hpp>

#include <fstream>
#include <sstream>

#if __cplusplus >= 201703L
#include <filesystem>
#define WITH_CPP_FILESYSTEM 1
#else
#include <sys/stat.h>
#include <unistd.h>
#endif //C++17


namespace log {

/**
 * @brief LogRotator class constructor
 */

LogRotator::LogRotator() {
  this->running = false;
}

/**
 * @brief Log rotator class destructor
 */

LogRotator::~LogRotator() {
  //Stop thread
  this->stopLogThread();
  //Free files
  for (auto& file : files) {
    delete file;
  }
}

/**
 * @brief add a file to the files to rotate and monitor; if the file already exists in the list, false is returned
 * @param string log file path
 * @param size_t maximum rotations for this file
 * @param size_t maximum file size
 * @return bool
 */

bool LogRotator::monitorLogfile(const std::string& logfile, const size_t maxRotations, const size_t maxFilesize) {
  for (const auto& file : this->files) {
    if (logfile == file->getLogfile()) {
      return false;
    } 
  }
  //Instance new log file
  Logfile* newLogfile;
  try {
    newLogfile = new Logfile(logfile, maxRotations, maxFilesize);
  } catch (std::bad_alloc& ex) {
    return false;
  }
  //Lock thread
  this->threadMutex.lock();
  this->files.push_back(newLogfile);
  this->threadMutex.unlock();
  return true;
}

/**
 * @brief start log rotator thread
 * @return bool; return false if already running or it was not possible to start the thread
 */

bool LogRotator::startLogThread() {
  if (this->running) {
    return false;
  }
  //Set running to true and start the thread
  this->running = true;
  this->logThread = std::thread(&LogRotator::run, this);
  return true;
}

/**
 * @brief stop log thread
 * @return bool
 */

bool LogRotator::stopLogThread() {
  if (this->running && this->logThread.joinable()) {
    this->running = false;
    this->logThread.join();
    return true;
  } else {
    return false;
  }
}

/**
 * @brief thread loop
 */

void LogRotator::run() {
  //Keep running (indeed) until running is true
  constexpr size_t ms500 = 500000;
  constexpr size_t sec30 = 30000000;
  while (this->running) {
    //Iterate over files
    this->threadMutex.lock();
    for (const auto& file : this->files) {
      //Check current size
      size_t currentFilesize;
      const std::string& currentFile = file->getLogfile();
      try {
        currentFilesize = getFilesize(currentFile);
      } catch (std::system_error& ex) {
        continue;
      }
      if (currentFilesize >= file->getMaxFileSize()) {
        //@! Over size
        //Move all the previous existing file
        const size_t maxRotations = file->getMaxRotations();
        bool errorOccurred = false;
        for (size_t thisRotation = maxRotations; thisRotation > 1; thisRotation--) {
          //Move thisRotation - 1 to thisRotation
          std::stringstream originStream;
          originStream << currentFile << "." << thisRotation - 1;
          std::stringstream destStream;
          destStream << currentFile << "." << thisRotation;
          if (fileExists(originStream.str())) { //If file exists move it
            if (moveFile(originStream.str(), destStream.str())) {
            } else {
              //Break, preserve files...
              break;
            }
          }
        }
        //Once copied file, rotate the current file
        std::stringstream destStream;
        destStream << currentFile << ".1";
        if (splitFile(currentFile, file->getMaxFileSize(), destStream.str())) {
        } else {
          continue;
        }
      }
      //File processed
    }
    this->threadMutex.unlock();
    //Sleep for 30 seconds, but with interrupts
    for (size_t sleep_time = 0; sleep_time < sec30 && this->running; sleep_time += ms500) {
      usleep(ms500);
    }
  }
}

//Private Utilities

/**
 * @brief get file size in bytes
 * @param string file
 * @return size_t
 * @throw std::system_error
 */

size_t LogRotator::getFilesize(const std::string& file) {
#if __cplusplus >= 201703L
  return std::filesystem::file_size(file);
#else
  struct stat statbuf;
  if (stat(file.c_str(), &statbuf) == -1) {
    const std::string errDesc = "No such file or directory: " + file;
    throw std::system_error(std::make_error_code(std::errc::no_such_file_or_directory), errDesc.c_str());
  }
  return static_cast<size_t>(statbuf.st_size);
#endif
}

/**
 * @brief move the first file's {maxSize} bytes to rotatedFilePath (lines are preserved)
 * @param string origin file
 * @param size_t max size
 * @param string rotated file path (destination)
 * @return bool
 */

bool LogRotator::splitFile(const std::string& file, const size_t maxSize, const std::string& rotatedFilePath) {
  //Read file
  std::ifstream fileStream;
  fileStream.open(file);
  //Could not open file
  if (!fileStream.is_open()) {
    return false;
  }
  std::string fileContent = std::string((std::istreambuf_iterator<char>(fileStream)), std::istreambuf_iterator<char>());
  fileStream.close();
  //Get the first maxSize - position of first EOL before max size position
  size_t cutIndex = maxSize - 1;
  if (cutIndex >= fileContent.length()) {
    return false; //Does not require to be split???
  }
  while (cutIndex > 0) { //Prevent going out of range
    //Find First EOL
    if (fileContent.at(cutIndex) == '\n') {
      cutIndex++; //Restore EOL index
      break; //Break when EOL is found
    }
    cutIndex--; //Otherwise decrement cut index
  }
  const std::string contentToRotate = fileContent.substr(0, cutIndex);
  fileContent = fileContent.substr(cutIndex);
  //Write files
  //Rewrite source
  std::ofstream outStream;
  outStream.open(file, std::ofstream::out | std::ofstream::trunc);
  if (!outStream.is_open()) {
    return false;
  }
  outStream << fileContent;
  outStream.close();
  //Write destination
  outStream.open(rotatedFilePath, std::ofstream::out | std::ofstream::trunc);
  if (!outStream.is_open()) {
    return false;
  }
  outStream << contentToRotate;
  outStream.close();
  return true;
}

/**
 * @brief move a file into another
 * @param string source file
 * @param string destination file
 * @return bool
 */

bool LogRotator::moveFile(const std::string& src, const std::string& dest) {
#if __cplusplus >= 201703L
  try {
    std::filesystem::rename(src, dest);
    return true;
  } catch (std::filesystem::filesystem_error& ex) {
    return false;
  }
#else
  return rename(src.c_str(), dest.c_str()) == 0;
#endif
}

/**
 * @brief returns whether the file exists
 * @param string file
 * @return bool
 */

bool LogRotator::fileExists(const std::string& file) {
#if __cplusplus >= 201703L
  return std::filesystem::exists(file);
#else
  struct stat buffer;
  return (stat (file.c_str(), &buffer) == 0);
#endif
}

}
