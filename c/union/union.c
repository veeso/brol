/**
 * union example
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

#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <string.h>

#define MAX_DATA_SIZE 256

typedef enum Endianness {
  ENDIANNESS_BIG_ENDIAN,
  ENDIANNESS_LITTLE_ENDIAN
} Endianness;

typedef enum EventType {
  EventInput,
  EventOutput,
  EventReset
} EventType;

// Since we need structs aligned to the buffer, we need to use packed structure (packing prevents compiler from doing padding)

typedef struct __attribute__((__packed__)) HeartBeatMessage {
  uint8_t error_state;
} HeartBeatMessage;

typedef struct __attribute__((__packed__)) EventMessage {
  uint8_t event_type;
  uint16_t event_id;
  uint32_t duration;
} EventMessage;

typedef union MessageData {
  uint8_t buffer[MAX_DATA_SIZE];
  HeartBeatMessage heartbeat;
  EventMessage event;
} MessageData;

typedef enum MessageType {
  Heartbeat,
  Event
} MessageType;

typedef struct Message {
  MessageType msg_type;
  size_t data_size;
  MessageData data;
} Message;

void print_message(const Message* message) {
  for (size_t i = 0; i < message->data_size; i++) {
    printf("%02x ", message->data.buffer[i]);
  }
  printf("\n");
  switch (message->msg_type) {
    case Heartbeat:
      printf("Heartbeat - error_state: %u\n", message->data.heartbeat.error_state);
      break;
    case Event:
      printf("Event - event_type %u; event_id %u; duration %lu\n", message->data.event.event_type, message->data.event.event_id, message->data.event.duration);
      break;
  }
}

int main(int argc, char** argv) {
  // @! Union example
  Message message;
  // Buffer to struct
  printf("Buffer to struct\n");
  const uint16_t ev_id = 8134;
  const uint32_t duration = 12481297;
  uint8_t data[MAX_DATA_SIZE] = {0};
  data[0] = 0x02; // Reset
  data[2] = ev_id >> 8;
  data[1] = ev_id & 0xFF;
  data[6] = (duration >> 24) & 0xFF;
  data[5] = (duration >> 16) & 0xFF;
  data[4] = (duration >> 8) & 0xFF;
  data[3] = duration & 0xFF;
  
  // Set message data
  message.msg_type = Event;
  message.data_size = 7;
  for (size_t i = 0; i < message.data_size; i++) {
    message.data.buffer[i] = data[i];
  }
  print_message(&message);
  // Reset message
  printf("\n===================================================================\n\n");
  memset(message.data.buffer, 0x00, MAX_DATA_SIZE);
  message.data_size = 0;

  // Struct to buffer
  printf("Struct to buffer\n");
  message.data.event.duration = 12481297;
  message.data.event.event_id = 8134;
  message.data.event.event_type = EventReset;
  message.data_size = 7;
  print_message(&message);

  return 0;
}
