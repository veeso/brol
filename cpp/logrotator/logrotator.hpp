/**
 *  @author: Christian Visintin
**/

#ifndef LOGROTATOR_HPP
#define LOGROTATOR_HPP

#include "logfile.hpp"

#include <list>
#include <mutex>
#include <string>
#include <thread>

namespace log {

class LogRotator {

public:
  LogRotator();
  ~LogRotator();
  bool monitorLogfile(const std::string& logfile, const size_t maxRotations, const size_t maxFilesize);
  bool unmonitorLogfile(const std::string& logfile);
  bool startLogThread();
  bool stopLogThread();

private:
  void run(); //Thread loop
  //Utilities
  size_t getFilesize(const std::string& file);
  bool splitFile(const std::string& file, const size_t maxSize, const std::string& rotatedFilePath);
  bool moveFile(const std::string& src, const std::string& dest);
  bool fileExists(const std::string& file);
  //Attributes
  bool running;
  std::list<Logfile*> files;
  std::thread logThread;
  std::mutex threadMutex;

};

}

#endif //LOGROTATOR_HPP
