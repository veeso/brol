#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>

uint64_t traslate_bit(uint64_t value, uint8_t from_power, uint8_t to_power) {
  while (from_power < to_power) {
    value = (value * 2) + 1;
    from_power++;
  }
  return value;
}

int main(int argc, char** argv) {

  if (argc < 4) {
    printf("Usage: %s <value> <from> <to>\n", argv[0]);
    return 255;
  }

  const uint64_t value = atoll(argv[1]);
  const uint8_t from = atol(argv[2]);
  const uint8_t to = atol(argv[3]);

  const uint64_t traslated_bit = traslate_bit(value, from, to);
  printf("%zu => %zu\n", value, traslated_bit);

  return 0;
}
