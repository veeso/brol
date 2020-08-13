/**
 * aixlog example
 * 
 *             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *                  Version 2, December 2004
 *
 * Copyright (C) 2020 Christian Visintin

 * Everyone is permitted to copy and distribute verbatim or modified
 * copies of this license document, and changing it is allowed as long
 * as the name is changed.
 *
 *          DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 * TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
 *
 * 0. You just DO WHAT THE FUCK YOU WANT TO.
 */

#include <aixlog.hpp>

#include <fstream>
#include <iostream>
#include <string>

#define USAGE "aixlog <level[1-6]> <logfile>"

#define LOG_NAME "AixLog::Main"

AixLog::Severity intToSeverity(const int loglevel) {
  switch (loglevel) {
    case 1:
      return AixLog::Severity::fatal;
    case 2:
      return AixLog::Severity::error;
    case 3:
      return AixLog::Severity::warning;
    case 4:
      return AixLog::Severity::info;
    case 5:
      return AixLog::Severity::debug;
    case 6:
      return AixLog::Severity::trace;
    default:
      return AixLog::Severity::info;
  }
}

int main(int argc, char* argv[]) {
  if (argc < 3) {
    std::cout << USAGE << std::endl;
    return 255;
  }
  const AixLog::Severity loglevel = intToSeverity(std::stoi(argv[1]));
  const std::string logfile = argv[2];
  // Configure aixlog
  AixLog::Log::init(
    {
      // Log normal to stdout
      std::make_shared<AixLog::SinkCout>(loglevel, "%Y-%m-%dT%H:%M:%S.#ms [#severity] (#tag) #message"),
      std::make_shared<AixLog::SinkCallback>(loglevel, [logfile](const AixLog::Metadata& metadata, const std::string& message) {
        std::ofstream logFileStream;
        logFileStream.open(logfile, std::ofstream::out | std::ofstream::app);
        if (logFileStream.is_open()) {
          logFileStream << metadata.timestamp.to_string("%Y-%m-%dT%H:%M:%S.#ms") << " [" << metadata.severity << "] (" << metadata.tag << ") " << message << std::endl;
          logFileStream.close();
        }
      })
    }
  );

  LOG(FATAL, LOG_NAME) << "This is a FATAL log message";
  LOG(ERROR, LOG_NAME) << "This is a ERROR log message";
  LOG(WARNING, LOG_NAME) << "This is a WARNING log message";
  LOG(INFO, LOG_NAME) << "This is a INFO log message";
  LOG(DEBUG, LOG_NAME) << "This is a DEBUG log message";
  LOG(TRACE, LOG_NAME) << "This is a TRACE log message";
  
  return 0;

}
