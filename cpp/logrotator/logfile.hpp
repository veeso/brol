/**
 *  @author: Christian Visintin
**/

#ifndef LOGFILE_HPP
#define LOGFILE_HPP

#include <string>
namespace log {

class Logfile {

public:
  Logfile(const std::string& logfile, const size_t maxRotations, const size_t maxFileSize);
  const std::string& getLogfile() const;
  size_t getMaxRotations() const;
  size_t getMaxFileSize() const;

private:
  std::string logfile;
  size_t maxRotations;
  size_t maxFileSize;

};
}

#endif // LOGFILE_HPP
