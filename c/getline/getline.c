/**
 * @author Christian Visintin
 * @brief getline implementation for MinGW
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

ssize_t getline(char** line, size_t* line_sz, FILE* file) {

  // Get file size
  const ssize_t curr_pos = ftell(file);
  fseek(file, 0, SEEK_END);
  const size_t file_sz = ftell(file);
  // Restore pos
  fseek(file, curr_pos, SEEK_SET);
  if (curr_pos >= file_sz) { // EOF
    return -1;
  }
  const size_t read_sz = 0;
  *line_sz = read_sz;
  char* line_ptr = NULL;
  ssize_t line_len = 0;
  while (1) {
    // Read 2048
    char buffer[2048];
    size_t bytes_read = fread(buffer, sizeof(char), 2048, file);
    //printf("READ: %s\n", buffer);
    if (bytes_read == 0) {
      break;
    }
    // Look for 0x0A in buffer
    for (size_t i = 0; i < bytes_read; i++) {
      if (buffer[i] == 0x0A) {
        bytes_read = i;
        break;
      }
    }
    // Increment line sz
    const size_t prev_line_sz = *line_sz;
    *line_sz += bytes_read + 1;
    // Reallocate line
    char* prev_ptr = line_ptr;
    line_ptr = (char*) realloc(line_ptr, sizeof(char) * (*line_sz));
    if (line_ptr == NULL) {
      line_ptr = prev_ptr;
    }
    // Copy buffer to line
    memcpy(line_ptr + prev_line_sz, buffer, bytes_read);
    line_ptr[(*line_sz - 1)] = 0x00; // NULL terminate
    line_len += bytes_read;
  }
  // Restore file pointer (or set at the end)
  const size_t file_pos = curr_pos + line_len + 1;
  if (file_pos < file_sz) {
    fseek(file, file_pos, SEEK_SET);
  } else {
    fseek(file, 0, SEEK_END);
  }
  // Return
  *line = line_ptr;
  return line_len;
}

int main(int argc, char** argv) {

  if (argc < 2) {
    printf("Usage: %s <filename>\n", argv[0]);
    return 1;
  }

  const char* file_name = argv[1];
  FILE* fptr = fopen(file_name, "r");
  if (fptr == NULL) {
    printf("Could not open file %s\n", file_name);
    return 1;
  }
  // Read lines
  char* line = NULL;
  size_t line_sz = 0;
  ssize_t line_len = 0;
  // Iter lines
  size_t row = 0;
  while ((line_len = getline(&line, &line_sz, fptr)) != -1) {
    // Skip empty rows
    if (line_len == 0) {
      goto getline_continue;
    }
    // Remove newline
    while (line[line_len - 1] == 0x0a || line[line_len - 1] == 0x0d) {
      line[line_len - 1] = 0x00;
      line_len--;
      if (line_len <= 0) {
        goto getline_continue;
      }
    }
    // If line is empty continue (double check; now and before)
    if (line_len == 0) {
      goto getline_continue;
    }
    printf("trimmed line: %s\n", line);

getline_continue:
    // Free line
    free(line);
    line = NULL;
    line_sz = 0;
  } // End of iterator

  if (line != NULL) {
    free(line);
  }

  fclose(fptr);
  return 0;

}
