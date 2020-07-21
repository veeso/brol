/**
 *  _       _____  _  _        __   ____
 * | |     |__  / | || |      / /  / ___| _     _
 * | |       / /  | || |_    / /  | |   _| |_ _| |_
 * | |___   / /_  |__   _|  / /   | |__|_   _|_   _|
 * |_____| /____|    |_|   /_/     \____||_|   |_|
 * 
 * 
 * Written by Christian Visintin
 * Requirements: lz4 <https://lz4.github.io/lz4/>
 * Required C++ version: c++11
*/

#include <iostream>
#include <string>
#include <fstream>
#include <streambuf>

#include <lz4.h>

using namespace std;

int main(int argc, char** argv) {

  std::ifstream dataStream;
  dataStream.open("data.txt");
  if (!dataStream.is_open()) {
    cout << "Could not open data.txt" << endl;
    return 1;
  }

  //Read file
  std::string fileInput = std::string((std::istreambuf_iterator<char>(dataStream)),std::istreambuf_iterator<char>());

  //Close file
  dataStream.close();

  size_t toCompressDataSize = fileInput.length();
  char* compressedData = new char[toCompressDataSize];
  size_t compressedSize = LZ4_compress(fileInput.c_str(), compressedData, toCompressDataSize);

  std::ofstream outFile;
  outFile.open("compressed.dat", ofstream::binary);
  if (!outFile.is_open()) {
    cout << "Could not open compressed.dat" << endl;
    delete[] compressedData;
    return 1;
  }
  outFile.write(compressedData, compressedSize);
  outFile.close();
  delete[] compressedData;

  cout << "Data compressed have been written to compressed.dat\n";

  dataStream.open("compressed.dat", ifstream::binary);
  if (!dataStream.is_open()) {
    cout << "Could not open compressed.dat" << endl;
    return 1;
  }

  //Read LZ4 compressed data
  dataStream.seekg(0, dataStream.end);
  size_t compressedDataSize = dataStream.tellg();
  dataStream.clear();
  dataStream.seekg(0, ios::beg);
  compressedData = new char[compressedDataSize];
  dataStream.read(compressedData, compressedDataSize);
  dataStream.close();

  //Max decomperssed size
  size_t decompressedDataSize = (compressedDataSize << 8) - compressedDataSize - 2526;
  char* decompressedData = new char[decompressedDataSize];
  size_t realDecompressedDataSize = LZ4_decompress_safe(compressedData, decompressedData, compressedDataSize, decompressedDataSize);
  delete[] compressedData;
  compressedData = nullptr;

  outFile.open("decompressed.txt");
  if (! outFile.is_open()) {
    cout << "Could not open decompressed.txt\n";
    delete[] decompressedData;
    return 1;
  }
  outFile.write(decompressedData, realDecompressedDataSize);
  outFile.close();

  delete[] decompressedData;
  cout << "Data decompressed to decompressed.txt\n";

  return 0;

}
