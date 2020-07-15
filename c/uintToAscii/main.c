/**
 * Convert a uint to an hex ascii stream
 */

#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>

/**
 * @brief get the bytes size required to represent as ascii the value; max 8 bytes!
 * @param uint64_t
 * @return size_t
 */

size_t get_bytes_size(const uint64_t value) {

  //Get nearest byte size
  const unsigned int min_power = 256;
  uint64_t target_power = min_power;
  size_t bytes_size = 1;
  while (target_power <= value && bytes_size < 8) {
    target_power = (target_power * min_power);
    bytes_size++;
  }
  return bytes_size;
}

/**
 * @brief convert uint (max 8 bytes) to ascii
 * @param char
 * @param size_t
 * @param uint64_t
 * @return size_t
 */

size_t uint_to_ascii(char* ascii, const size_t ascii_size, const uint64_t value) {
  for (size_t i = ascii_size; i > 0; i--) {
    const uint8_t right_shift = ((ascii_size - i) / 2) * 8;
    const uint8_t byte_value = ((value >> right_shift) & 0xFF);
    const int is_left_nibble = i % 2 == 0;
    const uint8_t nibble_value = is_left_nibble ? (byte_value & 0xF) : (byte_value >> 4);
    //Convert nibble value to char
    char nibble_ascii;
    if (nibble_value < 0xa) {
      nibble_ascii = '0' + nibble_value;
    } else {
      nibble_ascii = 'a' + (nibble_value - 0xa);
    }
    ascii[i - 1] = nibble_ascii;
  }
}

int main(int argc, char** argv) {

  if (argc < 2) {
    printf("Usage: %s <value>\n", argv[0]);
    return 255;
  }
  //Get value
  const uint64_t value = (uint64_t) atoll(argv[1]);
  const size_t bytes_size = get_bytes_size(value);
  printf("Bytes size is %lu\n", bytes_size);
  const size_t ascii_size = bytes_size * 2;
  char* ascii = (char*) malloc(sizeof(char) * (ascii_size + 1));
  uint_to_ascii(ascii, ascii_size, value);
  ascii[ascii_size] = 0x00;
  printf("Value %llu => Ascii: 0x%s\n", value, ascii);
  free(ascii);
  return 0;

}
