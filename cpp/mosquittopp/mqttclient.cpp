/**
 *  terminale - Core application per terminali controllo accessi
 *  Author: Christian Visintin
 * 	Copyright: (C) 2019 Solari di Udine
 *  Owner: Solari di Udine
 *  Module: MQTT
**/

#include <mqttclient.hpp>

#include <iostream>

namespace mqtt {

/**
 * @function MQTTClient
 * @description MQTTClient class constructor
 * @param const std::string& address
 * @param const int port
 * @param const std::string& clientId
 * @param int qos
 */

MQTTClient::MQTTClient(const std::string& address, const int port, const std::string& clientId, const int qos /* = 1 */) {
  this->address = address;
  this->port = port;
  this->clientId = clientId;
  this->qos = qos;
  this->errorState = false;
  this->lastPublishedMessage = nullptr;
  mosqpp::lib_init();
}

/**
 * @function MQTTClient
 * @description MQTTClient class destructor
 */

MQTTClient::~MQTTClient() {
  disconnect();
  mosqpp::lib_cleanup();
  for (auto& message : messageQueue) {
    delete message;
  }
  delete lastPublishedMessage;
  lastPublishedMessage = nullptr;
}

/**
 * @function connect
 * @description establish a connection to the MQTT broker
 * @returns bool
 */

bool MQTTClient::connect() {
  int rc = mosqpp::mosquittopp::connect(address.c_str(), port);
  if (rc == MOSQ_ERR_SUCCESS)  {
    this->errorState = false;
    loop_start();
    return true;
  } else {
    error = mosqpp::strerror(rc);
    this->errorState = true;
    return false;
  }
}

/**
 * @function disconnect
 * @description disconnect from broker
 * @returns bool
 */

bool MQTTClient::disconnect() {
  int rc = mosqpp::mosquittopp::disconnect();
  if (rc == MOSQ_ERR_SUCCESS)  {
    this->errorState = false;
    loop_stop();
    return true;
  } else {
    error = mosqpp::strerror(rc);
    this->errorState = true;
    return false;
  }
}

/**
 * @function subscribe
 * @description subscribe to provided topic; topic is added to subbed topic
 * @param const std::string& topic
 * @returns bool
 */

bool MQTTClient::subscribe(const std::string& topic) {

  int rc = mosqpp::mosquittopp::subscribe(NULL, topic.c_str(), qos);
  if (rc == MOSQ_ERR_SUCCESS)  {
    this->errorState = false;
    subTopics.push_back(topic);
    return true;
  } else {
    error = mosqpp::strerror(rc);
    this->errorState = true;
    return false;
  }
}

/**
 * @function publish
 * @description publish message to topic
 * @param const std::string& topic
 * @param const std::string& payload
 * @returns bool
 */

bool MQTTClient::publish(const std::string& topic, const std::string& payload) {
  if (lastPublishedMessage != nullptr) {
    delete lastPublishedMessage;
    lastPublishedMessage = nullptr;
  }
  lastPublishedMessage = new MQTTMessage(topic, payload, 0);
  int rc = mosqpp::mosquittopp::publish(NULL, topic.c_str(), payload.length(), payload.c_str(), qos, false);
  if (rc == MOSQ_ERR_SUCCESS)  {
    this->errorState = false;
    return true;
  } else {
    error = mosqpp::strerror(rc);
    this->errorState = true;
    return false;
  }
}

/**
 * @function fetchMessageQueue
 * @description get current message queue; NOTE: returned message queue has to be destroyed with destroyMessageQueue()
 * @param std::list<MQTTMessage*>& messageQueue
 */

void MQTTClient::fetchMessageQueue(std::list<MQTTMessage*>& messageQueue) {
  for (auto& message : this->messageQueue) {
    messageQueue.push_back(new MQTTMessage(*message));
    delete message;
    message = nullptr;
  }
  this->messageQueue.clear();
}

/**
 * @function clearMessageQueue
 * @description clear message queue
 */

void MQTTClient::clearMessageQueue() {
  for (auto& message : this->messageQueue) {
    delete message;
    message = nullptr;
  }
  this->messageQueue.clear();
}

/**
 * @function destroyMessageQueue
 * @description destroy a previously passed message queue
 * @param std::list<MQTTMessage*>& message queue
 */

void MQTTClient::destroyMessageQueue(std::list<MQTTMessage*>& messageQueue) {
  for (auto& message : messageQueue) {
    delete message;
    message = nullptr;
  }
  messageQueue.clear();
}

/**
 * @function getError
 * @description get error state and message for MQTT client
 * @param std::string& error
 * @returns bool
 */

bool MQTTClient::getError(std::string& error) {
  error = this->error;
  return this->errorState;
}

//Event handlers

/**
 * @function on_connect
 * @description function called when device connects
 * @param int rc
 */

void MQTTClient::on_connect(int rc) {
  if (rc == MOSQ_ERR_SUCCESS) {
    std::cout << "Connected successfully to broker " << address << ":" << port << std::endl;
  } else {
    std::cout << "Error while connecting to broker " << address << ":" << port << " => " << mosqpp::strerror(rc) << std::endl;
  }
}

/**
 * @function on_disconnect
 * @description function called when device disconnects
 * @param int rc
 */

void MQTTClient::on_disconnect(int rc) {
  if (rc == MOSQ_ERR_SUCCESS) {
    std::cout << "Disconnected successfully from broker" << std::endl;
  } else {
    std::cout << "ERROR Disconnected successfully from broker" << error.c_str() << std::endl;
  }
}

/**
 * @function on_publish
 * @description function called when a message is published
 * @param int mid
 */

void MQTTClient::on_publish(int mid) {
  std::cout << "ON PUBLISH" << std::endl;
  if (lastPublishedMessage != nullptr) {
    printf("Successfully published message to topic %s with id %d and with payload \"%s\"\n", lastPublishedMessage->getTopic().c_str(), mid, lastPublishedMessage->getPayload().c_str());
  }
}

/**
 * @function on_message
 * @description function called when a message is received
 * @param const struct mosquitto_message* message
 */

void MQTTClient::on_message(const struct mosquitto_message* message) {
  std::cout << "ON MESSAGE" << std::endl;
  //If message is valid, add it to message queue
  if (message != NULL && message->payload != NULL) {
    std::string payload(reinterpret_cast<char*>(message->payload), message->payloadlen);
    MQTTMessage* recvMessage = new MQTTMessage(message->topic, payload, message->mid);
    printf("Received message from topic %s with id %d and with payload \"%s\"\n", recvMessage->getTopic().c_str(), recvMessage->getMessageId(), recvMessage->getPayload().c_str());
    messageQueue.push_back(recvMessage);
  }
}

}
