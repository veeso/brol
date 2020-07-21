/**
 *  @author: Christian Visintin
**/


#include <logfile.hpp>

namespace log {

/**
 * @brief Logfile class constructor
 * @param string logfile
 * @param size_t max rotations
 * @param size_t max file size
 */

Logfile::Logfile(const std::string& logfile, const size_t maxRotations, const size_t maxFileSize) {
  this->logfile = logfile;
  this->maxRotations = maxRotations;
  this->maxFileSize = maxFileSize;
}

/**
 * @brief returns the log file path
 * @return size_t
 */

const std::string& Logfile::getLogfile() const {
  return this->logfile;
}

/**
 * @brief returns the maximum amount of rotations for this log file
 * @return size_t
 */

size_t Logfile::getMaxRotations() const {
  return this->maxRotations;
}

/**
 * @brief returns the maximum file size for this log file
 * @return size_t
 */

size_t Logfile::getMaxFileSize() const {
  return this->maxFileSize;
}

}
