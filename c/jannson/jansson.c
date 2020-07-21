#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <jansson.h>

#define USAGE "jansson <json> <keyToGet>"

int main(int argc, char** argv) {
  if (argc < 3) {
    printf("%s\n", USAGE);
    return 1;
  }
  char* jsonText = argv[1];
  char* k = argv[2];

  char key[512];
  strcpy(key, k);
  int rc = 0;

  json_error_t error;
  json_t* root = json_loads(jsonText, 0, &error);
  json_t* json = root;
  if (json == NULL) {
    printf("error on line %d: %s\n", error.line, error.text);
    return 1;
  }
  

  json_t* keyJson;
  size_t index;
explore:
  printf("Looking for key '%s'\n", key);
   keyJson = json_object_get(json, key);
  if (keyJson == NULL) {
    printf("Key %s does not exist\n", key);
    rc = 1;
    goto cleanup;
  }
  if (json_is_string(keyJson)) {
    const char* stringValue = json_string_value(keyJson);
    printf("%s: %s\n", key, stringValue);
    //free(stringValue);
  } else if (json_is_number(keyJson)) {
    printf("%s: %f\n", json_number_value(keyJson));
  } else if (json_is_null(keyJson)) {
    printf("%s: null\n", keyJson);
  } else if (json_is_boolean(keyJson)) {
    printf("%s: %d\n", key, json_boolean_value(keyJson));
  } else if (json_is_object(keyJson)) {
    printf("%s is an object; type next key to get: ");
    scanf("%s", key);
    json = keyJson;
    goto explore;
  } else if (json_is_array(keyJson)) {
    printf("%s is an array; type index to select: ");
    scanf("%u", &index);
    json = json_array_get(keyJson, index);
    goto explore;
  } else {
    printf("Unknown type\n");
    rc = 1;
    goto cleanup;
  }

cleanup:
  //Free resource
  json_decref(root);
  return rc;
}
