/**
 *  terminale - Core application per terminali controllo accessi
 *  Author: Christian Visintin
 * 	Copyright: (C) 2019 Solari di Udine
 *  Owner: Solari di Udine
 *  Module: MQTT
**/

#ifndef MQTTMESSAGE_HPP
#define MQTTMESSAGE_HPP

#include <string>

namespace mqtt {

class MQTTMessage {
  
public:
  MQTTMessage(const std::string& topic, const std::string& payload, int messageId);
  ~MQTTMessage() = default;
  std::string getTopic();
  std::string getPayload();
  int getMessageId();
private:
  std::string topic;
  std::string payload;
  int messageId;
};

}

#endif
