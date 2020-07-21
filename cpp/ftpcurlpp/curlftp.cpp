/**
 *         _   _   ____    _                       _____   _____   ____
 *   ___  | | | | |  _ \  | |      _ __    _ __   |  ___| |_   _| |  _ \
 *  / __| | | | | | |_) | | |     | '_ \  | '_ \  | |_      | |   | |_) |
 * | (__  | |_| | |  _ <  | |___  | |_) | | |_) | |  _|     | |   |  __/
 *  \___|  \___/  |_| \_\ |_____| | .__/  | .__/  |_|       |_|   |_|
 *                                |_|     |_|
 * 
 * Written by Christian Visintin
 * Requirements: libcurlpp
 * Required C++ version: c++11
 * 
*/


#include <cstring>
#include <dirent.h>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include <curlpp/cURLpp.hpp>
#include <curlpp/Easy.hpp>
#include <curlpp/Exception.hpp>
#include <curlpp/Infos.hpp>
#include <curlpp/Options.hpp>

#define USAGE "curlftp <ftp://address> <ftpPort> <file/directory> [username] [password]"


/**
 * @function getDir
 * @description returns files in a directory
 * @param const std::string& directory to search in
 * @param std::vector<std::string>& files found
 * @param const std::string& match to find files
 * @returns int: amount of files found
 */

int getDir(const std::string& dir, std::vector<std::string>& files, const std::string& match) {
  DIR* dp;
  struct dirent* dirp;
  if ((dp = opendir(dir.c_str())) == NULL) {
    return -1;
  }

  files.clear();
  int entryCount = 0;

  while ((dirp = readdir(dp)) != NULL) {
    if (match == "*") {
      files.push_back(std::string(dirp->d_name));
      entryCount++;
    } else {
      //Check if file name contains match
      if (std::string(dirp->d_name).find(match) != std::string::npos) {
        files.push_back(std::string(dirp->d_name));
        entryCount++;
      }
    }
  }
  closedir(dp);
  return entryCount;
}

int main(int argc, char* argv[]) {

  if (argc < 4) {
    std::cout << USAGE << std::endl;
    return 1;
  }

  //Get opts
  const std::string address = argv[1];
  const int ftpPort = std::stoi(argv[2]);
  const std::string fileArg = argv[3];
  const std::string username = (argc > 5) ? argv[4] : "";
  const std::string password = (argc > 5) ? argv[5] : "";
  const bool useAuth = (argc > 5);

  std::vector<std::string> files;
  if (getDir(fileArg, files, "*") == -1) {
    std::cout << "In my opinion file argument is not a directory" << std::endl;
    files.push_back(fileArg);
  }

  for (auto& file : files) {
    if (file == "." || file == "..") {
      continue;
    }
    std::cout << file << " will be sent" << std::endl;
  }

  std::cout << std::endl << "===================================================" << std::endl << std::endl;

  curlpp::Easy FTPRequest;

  //FTP options
  FTPRequest.setOpt(curlpp::Options::Port(ftpPort));
  FTPRequest.setOpt(curlpp::Options::Upload(true));
  FTPRequest.setOpt(curlpp::Options::FtpCreateMissingDirs(true));
  //Timeouts
  FTPRequest.setOpt(cURLpp::Options::Timeout(30));
  FTPRequest.setOpt(cURLpp::Options::ConnectTimeout(30));
  //Authentication
  if (useAuth) {
    const std::string authString = username + ":" + password;
    FTPRequest.setOpt(curlpp::Options::UserPwd(authString));
  }

  //Make a request for each file
  for (auto& file : files) {
    if (file == "." || file == "..") {
      continue;
    }
    //Read file
    FILE* ftpFilePtr = fopen(file.c_str(), "rb");
    //Set header
    std::list<std::string> header;
    //Add file path to address
    const std::string ftpAddress = address + "/" + file;
    std::cout << "Trying to send file " << file << " to " << ftpAddress << std::endl;
    //Set options for current file
    FTPRequest.setOpt(curlpp::Options::Url(ftpAddress));
    FTPRequest.setOpt(curlpp::Options::ReadFile(ftpFilePtr));
    FTPRequest.setOpt(curlpp::Options::PostQuote(header));
    //Send request
    try {
      FTPRequest.perform();
      std::cout << "Request completed" << std::endl;
      int rc = static_cast<int>(curlpp::infos::ResponseCode::get(FTPRequest));
      std::cout << "Ret code: " << rc << std::endl;
      if (rc >= 200 && rc < 300) {
        std::cout << "File " << file << " transferred successfully" << std::endl;
      }
    } catch (curlpp::RuntimeError& ex) {
      std::cout << "Runtime error: " << ex.what() << std::endl;
    }
    fclose(ftpFilePtr);
  }
  
  //Clean structures
  FTPRequest.reset();
  curlpp::Cleanup();
  curlpp::terminate();

  return 0;

}
