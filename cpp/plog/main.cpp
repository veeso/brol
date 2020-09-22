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

#include <plog/Log.h>
#include <plog/Appenders/ColorConsoleAppender.h>
#include <plog/Appenders/RollingFileAppender.h>

#include <fstream>
#include <iostream>
#include <string>

#define USAGE "plog <level[1-5]> <logfile>"

#define LOG_NAME "Plog::Main"

plog::Severity intToSeverity(const int loglevel) {
  switch (loglevel) {
    case 1:
      return plog::Severity::fatal;
    case 2:
      return plog::Severity::error;
    case 3:
      return plog::Severity::warning;
    case 4:
      return plog::Severity::info;
    case 5:
      return plog::Severity::debug;
    default:
      return plog::Severity::info;
  }
}

int main(int argc, char* argv[]) {
  if (argc < 3) {
    std::cout << USAGE << std::endl;
    return 255;
  }
  // Get severity
  const plog::Severity severity = intToSeverity(std::stoi(argv[1]));
  // Add file appender to file; max size 10M, 10 rotations
  static plog::RollingFileAppender<plog::TxtFormatter> fileAppender(argv[2], 10485760, 10);
  // Add console appender
  static plog::ColorConsoleAppender<plog::TxtFormatter> consoleAppender; // Create the 2nd appender.
  // Initialize logger
  plog::init(severity, &fileAppender).addAppender(&consoleAppender); // Initialize the logger with the both appenders.

  LOG_DEBUG << "This is a DEBUG Message";
  LOG_INFO << "This is an INFO Message";
  LOG_WARNING << "This is a WARNING Message";
  LOG_ERROR << "This is an ERROR Message";
  LOG_FATAL << "This is a FATAL Message";

  
  return 0;

}
