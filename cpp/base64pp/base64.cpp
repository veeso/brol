/**
 * libBase64pp
 * @author: Christian Visintin
**/

#include <B64pp/base64.hpp>
#include <cstring>

using namespace base64;

static const char B64Table[] = {
  'A',
  'B',
  'C',
  'D',
  'E',
  'F',
  'G',
  'H',
  'I',
  'J',
  'K',
  'L',
  'M',
  'N',
  'O',
  'P',
  'Q',
  'R',
  'S',
  'T',
  'U',
  'V',
  'W',
  'X',
  'Y',
  'Z',
  'a',
  'b',
  'c',
  'd',
  'e',
  'f',
  'g',
  'h',
  'i',
  'j',
  'k',
  'l',
  'm',
  'n',
  'o',
  'p',
  'q',
  'r',
  's',
  't',
  'u',
  'v',
  'w',
  'x',
  'y',
  'z',
  '0',
  '1',
  '2',
  '3',
  '4',
  '5',
  '6',
  '7',
  '8',
  '9',
  '+',
  '/'
};

Base64::Base64() {
}

/**
 * @function encode
 * @description encode source into base64 string
 * @param const uint8_t*
 * @param size_t sourceLength
 * @returns std::string
**/

std::string Base64::encode(const uint8_t* source, size_t sourceLength) {

  //Prepare encoded buffer
  std::string encodedSource = "";

  char* sourceBuffer = reinterpret_cast<char*>(const_cast<uint8_t*>(source));

  uint8_t* tmpBuf = new uint8_t[3];
  int index = 0;

  while (sourceLength--) {
    //Read up to 3 bytes a time into 'tmp'
    tmpBuf[index++] = *(sourceBuffer++);
    //if is 3 bytes large encode then into buffer
    if (index == 3) {
      encodedSource += this->applyCodec(tmpBuf);
      //Reset index
      index = 0;
    }
  }

  //Remaining parts
  if (index > 0) {
    //Fill tmpBuffer with 0x00 at most 3 times
    for (int j = index; j < 3; j++) {
      tmpBuf[j] = 0x00;
    }
    encodedSource += this->applyCodec(tmpBuf);

    //Append '=' for remaining characters
    while ((index++ < 3)) {
      encodedSource += "=";
    }
  }
  delete[] tmpBuf;
  //Finally return encodedSource
  return encodedSource;
}

uint8_t* Base64::decode(const std::string& source, size_t& outLength) {

  //Get source length
  size_t sourceLength = source.length();
  //Prepare decoded buffer
  outLength = (sourceLength / 1.33) + 1;
  uint8_t* decodedSource = new uint8_t[outLength];
  //Prepare other buffers and values
  uint8_t* tmpBuf = new uint8_t[4];
  int index = 0;
  int j = 0;
  size_t sizeCounter = 0;

  char* sourceBuffer = const_cast<char*>(source.c_str());

  while (sourceLength--) {
    //Break if '=' or no base64
    if (sourceBuffer[j] == '=') {
      break;
    }
    if (!(isalnum(sourceBuffer[j]) || sourceBuffer[j] == '+' || '/' == sourceBuffer[j])) {
      break;
    }
    //Read up to 4 bytes at a time into 'tmpbuffer'
    tmpBuf[index++] = sourceBuffer[j++];
    //If 4 bytes read and decode into buffer
    if (index == 4) {
      //Translate values in tmp starting from table
      uint8_t* outBuf = new uint8_t[4];
      this->applyDecodec(outBuf, tmpBuf);
      memcpy(decodedSource + sizeCounter, outBuf, 3);
      delete[] outBuf;
      sizeCounter += 3;
      //Reset index
      index = 0;
    }
  }

  //Remaining parts
  if (index > 0) {
    //Fill tmpBuffer with 0x00 at most 4 times
    for (j = index; j < 4; j++) {
      tmpBuf[j] = 0x00;
    }
    //Translate remaining part
    uint8_t* outBuf = new uint8_t[3];
    this->applyDecodec(outBuf, tmpBuf);
    memcpy(decodedSource + sizeCounter, outBuf, 3);
    delete[] outBuf;
    sizeCounter += 3;
  }
  decodedSource[sizeCounter] = 0x00;

  delete[] tmpBuf;
  return decodedSource;
}

/**
 * @function applyCodec
 * @description apply rotations for b64 encoding
 * @param uint8_t* inBuffer
 * @returns std::string
**/

std::string Base64::applyCodec(uint8_t* inBuffer) {

  std::string encoded = "";
  uint8_t buffer[4];
  buffer[0] = (inBuffer[0] & 0xFC) >> 2;
  buffer[1] = ((inBuffer[0] & 0x03) << 4) + ((inBuffer[1] & 0xF0) >> 4);
  buffer[2] = ((inBuffer[1] & 0x0F) << 2) + ((inBuffer[2] & 0xC0) >> 6);
  buffer[3] = inBuffer[2] & 0x3F;
  //Translate each encoded buffer part according to base 64 table
  for (int j = 0; j < 4; j++) {
    encoded += B64Table[buffer[j]];
  }
  return encoded;
}

/**
 * @function applyDecodec
 * @description apply rotations for b64 decoding
 * @param uint8_t* outBuffer
 * @param uint8_t* inBuffer
 * @returns std::string
**/

uint8_t* Base64::applyDecodec(uint8_t* outBuffer, uint8_t* inBuffer) {
  for (int i = 0; i < 4; i++) {
    // find translation char in `B64Table'
    for (int j = 0; j < 64; j++) {
      if (inBuffer[i] == B64Table[j]) {
        inBuffer[i] = j;
        break;
      }
    }
  }

  outBuffer[0] = (inBuffer[0] << 2) + ((inBuffer[1] & 0x30) >> 4);
  outBuffer[1] = ((inBuffer[1] & 0xf) << 4) + ((inBuffer[2] & 0x3c) >> 2);
  outBuffer[2] = ((inBuffer[2] & 0x3) << 6) + inBuffer[3];

  return outBuffer;
}
