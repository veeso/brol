/**
 *  _       _   _       _____  _              __    ____
 * | |     (_) | |__   |__  / (_)  _ __      / /   / ___|    _       _
 * | |     | | | '_ \    / /  | | | '_ \    / /   | |      _| |_   _| |_
 * | |___  | | | |_) |  / /_  | | | |_) |  / /    | |___  |_   _| |_   _|
 * |_____| |_| |_.__/  /____| |_| | .__/  /_/      \____|   |_|     |_|
 *                                |_|
 * 
 * Written by Christian Visintin
 * Requirements: libzip <https://libzip.org/>
 * Required C++ version: c++11
*/

#include <fstream>
#include <iostream>
#include <list>
#include <sstream>
#include <string>

#include <zip.h>

#define USAGE "zip <outZipfile> <file1> ... <filen>"

std::string errorMessage;

bool zip(const std::string& zipFilePath, std::list<std::string>& files) {
  int err;
  struct zip* zipArchive;
  if ((zipArchive = zip_open(zipFilePath.c_str(), ZIP_CREATE, &err)) == NULL) {
    char buf[100];
    zip_error_to_str(buf, sizeof(buf), err, errno);
    errorMessage = "can't create zip archive: " + std::string(buf);
    return false;
  }
  //Create files
  for (auto& file : files) {
    zip_source *source = zip_source_file(zipArchive, file.c_str(), 0, 0);
    if (source == NULL) {
      char buf[100];
      zip_error_to_str(buf, sizeof(buf), err, errno);
      errorMessage = "Could not get data from file: " + std::string(buf);
      return false;
    }
    int index = static_cast<int>(zip_file_add(zipArchive, file.c_str(), source, ZIP_FL_OVERWRITE));
    if (index < 0) {
      char buf[100];
      zip_error_to_str(buf, sizeof(buf), err, errno);
      errorMessage = "Could not add file to archive: " + std::string(buf);
      return false;
    }
  }
  if (zip_close(zipArchive) == -1) {
    errorMessage = "Could not close ZIP archive";
    return false;
  }
  return true;
}

bool unzip(const std::string& zipFilePath, std::list<uint8_t*>& filesData) {
  
  struct zip* zipArchive;
  struct zip_file* zipFile;
  struct zip_stat zipStat;
  int err;
  if ((zipArchive = zip_open(zipFilePath.c_str(), 0, &err)) == NULL) {
    char buf[100];
    zip_error_to_str(buf, sizeof(buf), err, errno);
    errorMessage = "can't open zip archive: " + std::string(buf);
    return false;
  }

  //Get zip data
  int entries;
  if ((entries = zip_get_num_entries(zipArchive, 0)) > 0) {
    //Get all the files
    for (int i = 0; i < entries; i++) {
      if (zip_stat_index(zipArchive, i, 0, &zipStat) == 0) {
        zipFile = zip_fopen_index(zipArchive, 0, 0);
        if (!zipFile) {
          errorMessage = "Could not open zip file";
          return false;
        }
        //Allocate page data and Read archive
        std::cout << "Found file with name " << zipStat.name << std::endl;
        size_t dataSize = zipStat.size;
        uint8_t* pageData = new uint8_t[dataSize + 1];
        zip_fread(zipFile, pageData, zipStat.size);
        pageData[dataSize] = 0x00;
        zip_fclose(zipFile);
        filesData.push_back(pageData);
      }
    }
  } else {
    errorMessage = "ZIP archive is empty";
    return false;
  }

  if (zip_close(zipArchive) == -1) {
    errorMessage = "Could not close ZIP archive";
    return false;
  }
  return true;
}

int main(int argc, char* argv[]) {

  if (argc < 3) {
    std::cout << USAGE << std::endl;
    return 1;
  }

  std::list<std::string> files;
  std::string zipFile = argv[1];
  for (int i = 2; i < argc; i++) {
    files.push_back(argv[i]);
  }

  //Create zip file
  bool rc = zip(zipFile, files);
  if (!rc) {
    std::cout << "Could not compress files: " << errorMessage << std::endl;
    return 1;
  }
  std::cout << "File compressed successfully" << std::endl;
  std::list<uint8_t*> filesData;
  rc = unzip(zipFile, filesData);
  if (!rc) {
    std::cout << "Could not decompress file " << zipFile << std::endl;
    return false;
  }
  for (auto& file : filesData) {
    delete[] file;
  }
  std::cout << "Memory freed; decompressed successfully" << std::endl;

  return 0;
}
