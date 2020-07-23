/**
 * libBase64pp
 * @author: Christian Visintin
**/

#ifndef BASE64_HPP
#define BASE64_HPP

#include <string>
#include <cinttypes>

namespace base64 {

class Base64 {

public:
  Base64();
  std::string encode(const uint8_t* source, size_t sourceLength);
  uint8_t* decode(const std::string& source, size_t& outLength);
private:
  std::string applyCodec(uint8_t* inBuffer);
  uint8_t* applyDecodec(uint8_t* outBuffer, uint8_t* inBuffer);
};

} // namespace base64

#endif
