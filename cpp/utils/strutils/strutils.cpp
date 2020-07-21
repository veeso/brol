/** 
 *  ____    _            _   _   _     _   _
 * / ___|  | |_   _ __  | | | | | |_  (_) | |  ___
 * \___ \  | __| | '__| | | | | | __| | | | | / __|
 *  ___) | | |_  | |    | |_| | | |_  | | | | \__ \
 * |____/   \__| |_|     \___/   \__| |_| |_| |___/
 * 
 *  
 * Author: Christian Visintin
**/

#include "strutils.hpp"

#include <iomanip>
#include <sstream>

namespace strutils {

/**
 * @function split
 * @description split std::string into vector of string dividing each token using delimiter
 * @param std::string string to split
 * @param char delimiter
 * @returns std::vector<std::string>
**/

std::vector<std::string> split(const std::string& s, char delimiter) {
  std::vector<std::string> tokens;
  std::string token;
  std::istringstream tokenStream(s);
  while (std::getline(tokenStream, token, delimiter)) {
    tokens.push_back(token);
  }
  return tokens;
}

/**
 * @function startsWith
 * @description check if a string starts with a certain string
 * @param std::string haystack
 * @param std::string needle
 * @returns bool: true if haystack starts with needle
**/

bool startsWith(const std::string& haystack, const std::string& needle) {

  if (needle.length() > haystack.length()) {
      return false;
  }

  std::string startString = haystack.substr(0, needle.length());
  return startString == needle;
}

/**
 * @function endsWith
 * @description check if a string ends with a certain string
 * @param std::string haystack
 * @param std::string needle
 * @returns bool: true if haystack ends with needle
**/

bool endsWith(const std::string& haystack, const std::string& needle) {

  if (needle.length() > haystack.length()) {
      return false;
  }

  std::string endString = haystack.substr(haystack.length() - needle.length(), needle.length());
  return endString == needle;
}

/**
 * @function itrim
 * @description trim multiple spaces (more than one) inside a string
 * @param std::string
 * @returns std::string trimmed string
**/

std::string itrim(const std::string& haystack) {

  std::string trimmed = haystack;

  size_t multiBlankPos = haystack.find("  "); //Two spaces
  if (multiBlankPos == std::string::npos) {
    return trimmed;
  } else {
    //Create string without a space
    return itrim(trimmed.erase(multiBlankPos, 1));
  }

}

/**
 * @function ltrim
 * @description trim left side of provided string
 * @param std::string
 * @returns std::string trimmed string
**/

std::string ltrim(const std::string& haystack) {
  std::string trimmed = haystack;
  //Recursive call for ltrim
  if (trimmed.length() > 0 && (trimmed.at(0) == 0x20 || trimmed.at(0) == 0x09)) {
    return ltrim(trimmed.substr(1));
  }
  return trimmed;
}

/**
 * @function rtrim
 * @description trim right side of provided string
 * @param std::string
 * @returns std::string trimmed string
**/

std::string rtrim(const std::string& haystack) {
  std::string trimmed = haystack;
  //Recursive call for ltrim
  size_t lastPos = trimmed.length() > 0 ? trimmed.length() - 1 : 0;
  if (trimmed.length() > 0 && (trimmed.at(lastPos) == 0x20 || trimmed.at(lastPos) == 0x09)) {
    return rtrim(trimmed.substr(0, lastPos));
  }
  return trimmed;
}

/**
 * @function ltrim
 * @description trim both sides of provided string
 * @param std::string
 * @returns std::string trimmed string
**/

std::string trim(const std::string& haystack) {
  std::string trimmed = haystack;
  trimmed = ltrim(trimmed);
  trimmed = rtrim(trimmed);
  return trimmed;
}

/**
 * @function substring
 * @description Returns a new string that is a substring of str. The new string is made up of the character of str between beginIndex and endIndex
 * @param std::string str
 * @param size_t startIndex
 * @param size_t endIndex
 * @returns strd::string substring
**/

std::string substring(const std::string& str, size_t startIndex, size_t endIndex /* = -1 */) {
  return str.substr(startIndex, endIndex - startIndex);
}

std::string replaceAll(const std::string& str, const std::string& from, const std::string& to) {
    size_t start_pos = 0;
    std::string outStr = str;
    while((start_pos = outStr.find(from, start_pos)) != std::string::npos) {
        outStr.replace(start_pos, from.length(), to);
        start_pos += to.length(); // Handles case where 'to' is a substring of 'from'
    }
    return outStr;
}

/**
 * @function epochToTimeString
 * @description convert time_t UNIX epoch to YYYY/MM/DD-hh:mm:ss format
 * @param time_t UNIX epoch
 * @param string format
 * @param string locale
 * @returns std::string
 */

std::string epochToTimeString(const time_t epoch, const std::string& format /* = "%Y/%m/%d-%H:%M:%S" */, const std::string& locale /* = "en_EN" */) {
  std::stringstream timeStream;
  struct tm epochTm;
  localtime_r(&epoch, &epochTm);
  timeStream << std::put_time(std::localtime(&epoch), format.c_str());
  return timeStream.str();
}

/**
 * @function parseIso8601Timestamp
 * @description: parse ISO8601 timestamp and convert it into UNIX epoch
 * @param const std::string&
 * @returns time_t
 * NOTE: isoTimestamp format: "YYYY-mm-ddTHH:MM:SS[+/-]THTM"
 * NOTE: TH: timezone hours
 * NOTE: TM: timezone minutes
**/

time_t parseIso8601Timestamp(const std::string& isoTimestamp) {

  struct tm timeStruct;
  bool withTz = false;
  //Check length
  if (isoTimestamp.length() < 19) {
    //Invalid length
    return 0;
  }
  //Check if contains tzdata
  if (isoTimestamp.length() == 24) {
    withTz = true;
  }
  strptime(isoTimestamp.c_str(), "%Y-%m-%dT%H:%M:%S", &timeStruct);
  time_t outEpoch;
  if (withTz) {
     outEpoch = timegm(&timeStruct);
    //Get last 4 characters from isTimestamp
    std::string timezoneStr = isoTimestamp.substr(isoTimestamp.length() - 5);
    //Parse timezone
    char tOperator = timezoneStr.at(0);
    int tzHours = std::stoi(timezoneStr.substr(1, 2));
    int tzMinutes = std::stoi(timezoneStr.substr(3, 2));
    int tzSeconds = (tzHours * 3600) + (tzMinutes * 60);
    /**
     * You're going to wonder: why not to use mktime?
     * Well try the same code inverting operators in tzSeconds
     * with mktime and you'll see it will give everytime a different result, 
     * I don't know why the hell
     * time_t outEpoch = timegm(&timeStruct);
     * Apply timezone to convert to UTC (reverse logic)
    **/
    if (tOperator == '+') {
      outEpoch -= tzSeconds;
    } else {
      outEpoch += tzSeconds;
    }
  } else {
    outEpoch = mktime(&timeStruct);
  }
  return outEpoch;
}

/**
 * @function splitTextGracefully
 * @brief split a text into tokens by delimeter and length, preserving whole word
 * @param string text
 * @param char delimeter
 * @param size_t max text token length
 * @returns std::vector<std::string>
 */

std::vector<std::string> splitTextByLengthGracefully(const std::string& text, const char delim, const size_t maxTextLength) {
  std::vector<std::string> tokens = split(text, delim);
  std::vector<std::string> result;
  std::string currentLine = "";
  for (auto& token : tokens) {
    if (currentLine.length() + token.length() > maxTextLength) {
      result.push_back(currentLine.substr(0, currentLine.length() - 1));
      currentLine = "";
    }
    currentLine +=  token + delim;
  }
  if (currentLine.length() > 0) {
    result.push_back(currentLine.substr(0, currentLine.length() - 1));
  }
  return result;
}

}

