/**
 * aixlog example
 * 
 *             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *                  Version 2, December 2004
 *
 * Copyright (C) 2020 Christian Visintin

 * Everyone is permitted to copy and distribute verbatim or modified
 * copies of this license document, and changing it is allowed as long
 * as the name is changed.
 *
 *          DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 * TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
 *
 * 0. You just DO WHAT THE FUCK YOU WANT TO.
 */

#include <CLI11.hpp>

#include <iostream>
#include <string>

int main(int argc, char* argv[]) {

  // Setup options
  CLI::App app{"Testapp"};
  std::string brokerAddress = "localhost";
  app.add_option("-b,--broker", brokerAddress, "Specify broker address");
  int brokerPort = 1883;
  app.add_option("-p,--port", brokerPort, "Specify broker port");
  int clid = 0;
  app.add_option("-c,--clientid", clid, "Specify client id");
  std::string pidfile = "";
  app.add_option("-P,--pidfile", pidfile, "Write pidfile");
  int loglevel = 4;
  app.add_option("-l,--loglevel", loglevel, "[1:FATAL, 2:ERROR, 3:WARNING, 4:INFO, 5:DEBUG, 6:TRACE]");
  std::string logfile = "/var/log/test.log";
  app.add_option("-L,--logfile", logfile, "Specify logfile");
  // Get options
  CLI11_PARSE(app, argc, argv);

  std::cout << "broker: " << brokerAddress << std::endl;
  std::cout << "port: " << brokerPort << std::endl;
  std::cout << "clid: " << clid << std::endl;
  std::cout << "pidfile: " << pidfile << std::endl;
  std::cout << "loglevel: " << loglevel << std::endl;
  std::cout << "logfile: " << logfile << std::endl;

  return 0;
}
