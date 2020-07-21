/**
 *  terminale - Core application per terminali controllo accessi
 *  Author: Christian Visintin
 * 	Copyright: (C) 2019 Solari di Udine
 *  Owner: Solari di Udine
 *  Module: MQTT
**/

#include <mqttmessage.hpp>

namespace mqtt {

/**
 * @function MQTTMessage
 * @description MQTTMessage class constructor
 * @param const std::string& topic
 * @param const std::string& payload
 * @param int messageId
 */

MQTTMessage::MQTTMessage(const std::string& topic, const std::string& payload, int messageId) {
  this->topic = topic;
  this->payload = payload;
  this->messageId = messageId;
}

std::string MQTTMessage::getTopic() {
  return this->topic;
}

std::string MQTTMessage::getPayload() {
  return this->payload;
}

int MQTTMessage::getMessageId() {
  return this->messageId;
}

}
