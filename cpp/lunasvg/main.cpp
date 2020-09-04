/**
 *  @author: Christian Visintin
 */

#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>


#include <png.h>
#include <lunasvg/svgdocument.h>

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

bool bitmapToPng(const uint8_t* bitmap, const size_t width, const size_t height, const std::string& outFile) {

  bool rc = false;
  FILE* fp = NULL;
  png_structp png = NULL;
  png_infop info = NULL;
  png_bytep *rowPointers = NULL;
  const size_t dataSize = width * height * 4; // RGBA

  fp = fopen(outFile.c_str(), "wb");
  if (!fp) {
    goto exit;
  }
  png = png_create_write_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
  if (!png) {
    goto exit;
  }
  info = png_create_info_struct(png);
  if (!info) {
    goto exit;
  }
  if (setjmp(png_jmpbuf(png))) {
    goto exit;
  }

  png_init_io(png, fp);

  // Output is 8bit depth, RGBA format.
  png_set_IHDR(
    png,
    info,
    width, height,
    8,
    PNG_COLOR_TYPE_RGBA,
    PNG_INTERLACE_NONE,
    PNG_COMPRESSION_TYPE_DEFAULT,
    PNG_FILTER_TYPE_DEFAULT
  );
  png_write_info(png, info);

  // Allocate row pointers
  rowPointers = (png_bytep*) malloc(sizeof(png_bytep) * height);
  for (size_t y = 0; y < height; y++) {
    rowPointers[y] = (png_byte*) malloc(sizeof(png_byte) * png_get_rowbytes(png, info));
    png_byte* thisRow = rowPointers[y];
    const size_t bitmapY = y * 4;
    for (size_t x = 0; x < width; x++) {
      const size_t bitmapX = x * 4;
      const size_t rowIndex = x * 4;
      thisRow[rowIndex + 0] = bitmap[(width * bitmapY) + (bitmapX + 0)];
      thisRow[rowIndex + 1] = bitmap[(width * bitmapY) + (bitmapX + 1)];
      thisRow[rowIndex + 2] = bitmap[(width * bitmapY) + (bitmapX + 2)];
      thisRow[rowIndex + 3] = bitmap[(width * bitmapY) + (bitmapX + 3)];
    }
  }
  png_write_image(png, rowPointers);
  png_write_end(png, NULL);

  for (size_t y = 0; y < height; y++) {
    free(rowPointers[y]);
  }
  free(rowPointers);

  rc = true;
exit:

  if (fp) {
    fclose(fp);
  }
  if (png && info) {
    //Free PNG struct
    png_destroy_write_struct(&png, &info);
  }

  return rc;
}

int main(int argc, char* argv[]) {

  if (argc < 3) {
    std::cout << "Usage: " << argv[0] << " <svgfile> <outfile> [size]" << std::endl;
    return 255;
  }

  // Read svg file
  std::ifstream svgfileStream;
  svgfileStream.open(argv[1]);
  if (!svgfileStream.is_open()) {
    std::cout << "Could not open file " << argv[1] << std::endl;
    return 1;
  }
  const std::string svgData = std::string((std::istreambuf_iterator<char>(svgfileStream)), std::istreambuf_iterator<char>());
  svgfileStream.close();

  // Load SVG
  lunasvg::SVGDocument document;
  document.loadFromData(svgData);

  uint32_t width = 0;
  uint32_t height = 0;
  if (argc > 3) {
    const std::string sizeStr = argv[3];
    const std::vector<std::string> sizeTokens = split(sizeStr, 'x');
    if (sizeTokens.size() != 2) {
      std::cout << "Invalid size argument (expexted: widthxheight; got: " << sizeStr << ")" << std::endl;
      return 1;
    }
    width = static_cast<uint32_t>(std::stoul(sizeTokens.at(0)));
    height = static_cast<uint32_t>(std::stoul(sizeTokens.at(1)));
  }

  const lunasvg::Bitmap bitmap = document.renderToBitmap(width, height);
  // Convert bitmap to PNG
  const std::string outFile = std::string(argv[2]);
  if (!bitmapToPng(bitmap.data(), bitmap.width(), bitmap.height(), outFile)) {
    std::cout << "Could not convert bitmap to PNG" << std::endl;
    return 1;
  }

  return 0;
}
