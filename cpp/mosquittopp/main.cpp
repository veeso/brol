#include <iostream>
#include <unistd.h>

#include <mqttclient.hpp>

using namespace std;
using namespace mqtt;

int main(int argc, char* argv[]) {

  const std::string address = "localhost";
  const std::string clid = "Provola";
  MQTTClient* myCli = new MQTTClient(address, 1883, clid, 1);

  if (!myCli->connect()) {
    string error;
    myCli->getError(error);
    cout << error << endl;
    delete myCli;
    return 1;
  }
  if (!myCli->subscribe("display/event")) {
    string error;
    myCli->getError(error);
    cout << error << endl;
    myCli->disconnect();
    delete myCli;
    return 1;
  }
  if (!myCli->publish("display/command", "{\"cmd\":\"CONNECT\", \"cmdId\": 0}")) {
    string error;
    myCli->getError(error);
    cout << "NON VA UN CAZZO: " << error << endl;
    myCli->disconnect();
    delete myCli;
    return 1;
  }
  for (int i = 0; i < 5; i++) {
    std::list<MQTTMessage*> msg;
    myCli->fetchMessageQueue(msg);
    //Iterate over messages
    for (auto& message : msg) {
      std::cout << message->getTopic() << " => " << message->getPayload() << std::endl;
    }
    myCli->destroyMessageQueue(msg);
    std::cout << "Waiting for other 10 seconds for other messages..." << std::endl;
    sleep(10);
  }

  if (!myCli->disconnect()) {
    string error;
    myCli->getError(error);
    cout << error << endl;
  }

  delete myCli;
  return 0;

}
