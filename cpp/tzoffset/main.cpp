/**
 *  _____            ___     __    __                _
 * |_   _|  ____    / _ \   / _|  / _|  ___    ___  | |_
 *   | |   |_  /   | | | | | |_  | |_  / __|  / _ \ | __|
 *   | |    / /    | |_| | |  _| |  _| \__ \ |  __/ | |_
 *   |_|   /___|    \___/  |_|   |_|   |___/  \___|  \__|
 * 
 *
 */

#include <chrono>
#include <iomanip>
#include <sstream>

static constexpr time_t const NULL_TIME = -1;

// returns difference in seconds from UTC at given time
// or at current time if not specified
long tz_offset_sstream(time_t when = NULL_TIME) {
  if (when == NULL_TIME) {
    when = std::chrono::duration_cast<std::chrono::seconds>(std::chrono::system_clock::now().time_since_epoch()).count();
  }
  auto const tm = *std::localtime(&when);
  std::ostringstream os;
  os << std::put_time(&tm, "%z");
  std::string s = os.str();
  // s is in ISO 8601 format: "±HHMM"
  int h = std::stoi(s.substr(0,3), nullptr, 10);
  int m = std::stoi(s[0]+s.substr(3), nullptr, 10);

  return h * 3600 + m * 60;
}

long tz_offset_localtime(time_t when = NULL_TIME) {
  if (when == NULL_TIME) {
    when = std::chrono::duration_cast<std::chrono::seconds>(std::chrono::system_clock::now().time_since_epoch()).count();
  }
  auto const tm = *std::localtime(&when);
  return tm.tm_gmtoff;
}

#include <iostream>
int main(int argc, char* argv[]) {
  time_t when = NULL_TIME;
  if (argc > 1) {
    when = std::stol(argv[1]);
  }
  std::cout << "With sstream: " << tz_offset_sstream(when) << std::endl;
  std::cout << "With localtime: " << tz_offset_localtime(when) << std::endl;
  return 0;
}
