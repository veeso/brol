/**
 *  terminale - Core application per terminali controllo accessi
 *  Author: Christian Visintin
 * 	Copyright: (C) 2019 Solari di Udine
 *  Owner: Solari di Udine
 *  Module: MQTT
**/

#ifndef MQTTCLIENT_HPP
#define MQTTCLIENT_HPP

#include <mqttmessage.hpp>

#include <mosquittopp.h>

#include <list>
#include <vector>

namespace mqtt {

class MQTTClient : public mosqpp::mosquittopp {

public:
  MQTTClient(const std::string& address, const int port, const std::string& clientId, const int qos = 1);
  ~MQTTClient();
  bool connect();
  bool disconnect();
  bool subscribe(const std::string& topic);
  bool publish(const std::string& topic, const std::string& payload);
  void fetchMessageQueue(std::list<MQTTMessage*>& messageQueue);
  void clearMessageQueue();
  void destroyMessageQueue(std::list<MQTTMessage*>& messageQueue);
  bool getError(std::string& error);
  //Events
  void on_connect(int rc);
  void on_disconnect(int rc);
  void on_publish(int mid);
  void on_message(const struct mosquitto_message* message);
private:

  //Attributes
  std::string address;
  int port;
  std::string clientId;
  int qos;
  std::vector<std::string> subTopics;
  std::list<MQTTMessage*> messageQueue;
  MQTTMessage* lastPublishedMessage;
  std::string error;
  bool errorState;
};

}

#endif
