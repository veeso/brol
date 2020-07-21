/**
 *      _ ____   ___  _   _    ___
 *     | / ___| / _ \| \ | |  / _ \ _   _  ___ _ __ _   _
 *  _  | \___ \| | | |  \| | | | | | | | |/ _ \ '__| | | |
 * | |_| |___) | |_| | |\  | | |_| | |_| |  __/ |  | |_| |
 *  \___/|____/ \___/|_| \_|  \__\_\\__,_|\___|_|   \__, |
 *                                                  |___/
 * Written by Christian Visintin
 * Requirements: nlohmann
 * Required C++ version: c++11
 * NOTE: Export function findNode
 */

#include <nlohmann/json.hpp>
#include <queue>
#include <sstream>

#include <exception>

namespace strutils {

std::vector<std::string> split(const std::string& s, char delimiter);
bool startsWith(const std::string& haystack, const std::string& needle);
bool endsWith(const std::string& haystack, const std::string& needle);
std::string substring(const std::string& str, size_t startIndex, size_t endIndex = -1);

/**
 * @function split
 * @description split std::string into vector of string dividing each token using delimiter
 * @param std::string string to split
 * @param char delimiter
 * @returns std::vector<std::string>
**/

std::vector<std::string> split(const std::string& s, char delimiter) {
  std::vector<std::string> tokens;
  std::string token;
  std::istringstream tokenStream(s);
  while (std::getline(tokenStream, token, delimiter)) {
    tokens.push_back(token);
  }
  return tokens;
}

/**
 * @function startsWith
 * @description check if a string starts with a certain string
 * @param std::string haystack
 * @param std::string needle
 * @returns bool: true if haystack starts with needle
**/

bool startsWith(const std::string& haystack, const std::string& needle) {

  if (needle.length() > haystack.length()) {
      return false;
  }

  std::string startString = haystack.substr(0, needle.length());
  return startString == needle;
}

/**
 * @function endsWith
 * @description check if a string ends with a certain string
 * @param std::string haystack
 * @param std::string needle
 * @returns bool: true if haystack ends with needle
**/

bool endsWith(const std::string& haystack, const std::string& needle) {

  if (needle.length() > haystack.length()) {
      return false;
  }

  std::string endString = haystack.substr(haystack.length() - needle.length(), needle.length());
  return endString == needle;
}

/**
 * @function substring
 * @description Returns a new string that is a substring of str. The new string is made up of the character of str between beginIndex and endIndex
 * @param std::string str
 * @param size_t startIndex
 * @param size_t endIndex
 * @returns strd::string substring
**/

std::string substring(const std::string& str, size_t startIndex, size_t endIndex /* = -1 */) {
  return str.substr(startIndex, endIndex - startIndex);
}

}

nlohmann::json& findNode(const std::string& key);
nlohmann::json& getChild(nlohmann::json& parent, std::queue<std::string>& nodes);

/**
 * @function findNode
 * @description find node in json object using json syntax (e.g a.b[x].d)
 * @param const std::string&
 * @returns nlohmann::json
 * @callme
 */

nlohmann::json& findNode(nlohmann::json& root, const std::string& node) {
  //Split into tokens
  std::vector<std::string> queryTokens = strutils::split(node, '.');
  //Create node queue
  std::queue<std::string> nodes;
  for (auto& token : queryTokens) {
    bool isVector = strutils::endsWith(token, "]");
    if (isVector) {
      //Look for index mark
      size_t indexMarkPosition;
      if ((indexMarkPosition = token.find('[')) != std::string::npos) {
        //Has index; get index and remove marks from token
        const std::string indexStr = strutils::substring(token, indexMarkPosition + 1, token.length() - 1);
        token = strutils::substring(token, 0, indexMarkPosition);
        //Push to queue
        nodes.push(token);
        nodes.push(indexStr);
      } else {
        isVector = false;
        nodes.push(token);
      }
    } else {
      nodes.push(token);
    }
  }
  //Get child using recursion
  try {
    return getChild(root, nodes);
  } catch (std::runtime_error& ex) {
    throw ex;
  }
}

/**
 * @function getChild
 * @description: get the last child of a parent JSON node following the nodes queue using recursion
 * @param nlohmann::json& parent
 * @param std::queue<std::string> nodes
 * @returns nlohmann::json&
 * @throws: std::runtime_error: if node doesn't exist
 * NOTE: base case: nodes queue is empty => returns last child
 * NOTE: recursive case: return next child following the queue
 * NOTE: why to use recursion? It's the only way to get iteratively the last child of a json following a tree query
 */

nlohmann::json& getChild(nlohmann::json& parent, std::queue<std::string>& nodes) {
  std::string node = nodes.front(); //Get element to find
  nodes.pop(); //Remove first element from queue
  if (parent.is_array()) { //@! If it is array...
    size_t index;
    //Check if index is valid
    try {
      index = std::stoul(node);
    } catch (std::invalid_argument& ex) {
      std::stringstream exStream;
      exStream << "Index '" << node << "' is not a number";
      throw std::runtime_error(exStream.str().c_str());
    }
    //Check if index exists in parent array
    if (parent.size() < index) {
      std::stringstream exStream;
      exStream << "Index '" << node << "' is out of range";
      throw std::runtime_error(exStream.str().c_str());
    }
    //Get child and switch between cases
    nlohmann::json& child = parent[index];
    if (nodes.size() == 0) {
      //Base Case
      return child;
    } else {
      //Recursive case
      return getChild(child, nodes);
    }
  } else if (parent.find(node) != parent.end()) { //@! Not an array; check if node exists
    //It exists, get element and switch between cases
    nlohmann::json& child = parent[node];
    if (nodes.size() == 0) {
      //Base Case
      return child;
    } else {
      //Recursive case
      return getChild(child, nodes);
    }
  } else {
    //@! Node does not exist
    std::stringstream exStream;
    exStream << "Could not find node '" << node << "' in JSON";
    throw std::runtime_error(exStream.str().c_str());
  }
}
