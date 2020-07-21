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

#ifndef STRINGUTILS_HPP
#define STRINGUTILS_HPP

#include <chrono>
#include <string>
#include <vector>

namespace strutils {

std::vector<std::string> split(const std::string& s, char delimiter);
bool startsWith(const std::string& haystack, const std::string& needle);
bool endsWith(const std::string& haystack, const std::string& needle);
std::string itrim(const std::string& haystack);
std::string ltrim(const std::string& haystack);
std::string rtrim(const std::string& haystack);
std::string trim(const std::string& haystack);
std::string substring(const std::string& str, size_t startIndex, size_t endIndex = -1);
std::string replaceAll(const std::string& str, const std::string& from, const std::string& to);
std::string epochToTimeString(const time_t epoch, const std::string& format = "%Y-%m-%dT%H:%M:%S");
std::vector<std::string> splitTextByLengthGracefully(const std::string& text, const char delim, const size_t maxTextLength);
time_t parseIso8601Timestamp(const std::string& isoTimestamp);

} //namespace strutils

#endif
