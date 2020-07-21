#include <iostream>
#include <fstream>
#include <list>
#include <string>

#include <libxml/parser.h>
#include <libxml/tree.h>

#include <map>
#include <regex>

void printElementNames(xmlNode* node) {
  if (node->type == XML_ELEMENT_NODE) {
    std::cout << "Element name: " << node->name << std::endl;
    xmlAttr* attr = node->properties;
    while(attr) {
      xmlChar* ac = xmlGetProp(node, attr->name);
      std::cout << "\tFile has property: " << attr->name << " = " << ac << std::endl;
      xmlFree(ac);
      attr = attr->next;
    }
  }
  for (xmlNode* child = node->children; child; child = child->next) {
    printElementNames(child);
  }
}

xmlNode* findElement(xmlNode* node, const std::string& id) {
  if (node->type == XML_ELEMENT_NODE) {
    xmlAttr* attr = node->properties;
    while(attr) {
      const std::string property = reinterpret_cast<const char*>(attr->name);
      if (property == "id") {
        xmlChar* ac = xmlGetProp(node, attr->name);
        const std::string elementId = reinterpret_cast<const char*>(ac);
        xmlFree(ac);
        if (id == elementId) {
          return node;
        }
      }
      attr = attr->next;
    }
  }
  for (xmlNode* child = node->children; child; child = child->next) {
    xmlNode* foundNode;
    if ((foundNode = findElement(child, id)) != nullptr) {
      return foundNode;
    }
  }
  return nullptr;
}

bool nestElement(xmlNode* root, const std::string& id, const std::string& element) {
  //Find element
  xmlNode* target = findElement(root, id);
  if (target == NULL) {
    return false;
  }
  xmlNode* child = NULL;
  xmlParseInNodeContext(target, element.c_str(), element.size(), 0, &child);
  if (child == NULL) {
    return false;
  }
  //printElementNames(child);
  //for (xmlNode* pChild; pChild != NULL; pChild = pChild->next) {
  //  xmlAddChild(target, xmlCopyNode(pChild, 1));
  //}
  xmlAddChild(target, xmlCopyNode(child, 1));
  xmlFreeNode(child);
  return true;
}

bool appendSvgElement(xmlNode* root, const std::string& parentId, const std::string& elementStr) {
  xmlNode* target = findElement(root, parentId);
  if (target == NULL) {
    return false;
  }
  xmlNode* child = NULL;
  xmlParseInNodeContext(target, elementStr.c_str(), elementStr.size(), 0, &child);
  if (child == NULL) {
    return false;
  }
  //printElementNames(child);
  xmlAddChild(target, xmlCopyNode(child, 1));
  ///child->children = NULL;
  ///child->next = NULL;
  xmlFreeNode(child);
  return true;
}


bool replaceValue(xmlNode* root, const std::string& id, const std::string& value) {
  xmlNode* target = findElement(root,id);
  if (target == nullptr) {
    return false;
  }
  if (target->children != NULL && target->children->content != NULL) {
    std::cout << "Current content: " << target->children->content << std::endl;
    xmlNodeSetContent(target, reinterpret_cast<const xmlChar*>(value.c_str()));
    return true;
  }
}

bool replaceValuesInNode(xmlNode* node, const std::map<std::string, std::string>& dict, bool replaceEmptyKeys, const std::string replaceEmptyKeysWith, const std::string keyPrefix, const std::string keyPostfix) {
  if (node->type == XML_ELEMENT_NODE) {
    //Get content
    std::cout << "Current node: " << node->name << std::endl;
    if (node->children != NULL && node->children->content != NULL) {
      bool alreadyReplaced = false;
      const std::string currentNodeValue = reinterpret_cast<const char*>(node->children->content);
      std::cout << "This node has text: " << currentNodeValue << std::endl;
      //Iterate over dict
      for (auto& dictPair : dict) {
        const std::string key = keyPrefix + dictPair.first + keyPostfix;
        std::cout << "Comparing: '" << key << "' with '" << currentNodeValue << "'" << std::endl;
        //replace key with value
        if (key == currentNodeValue) {
          std::cout << "key is equal to the other value " << key << std::endl;
          //Then replace value
          xmlNodeSetContent(node, reinterpret_cast<const xmlChar*>(dictPair.second.c_str()));
          alreadyReplaced = true;
          break; //Stop searching
        }
      }
      if (replaceEmptyKeys && !alreadyReplaced) {
        std::stringstream regexStream;
        regexStream << keyPrefix << "(.*)" << keyPostfix;
        std::regex replaceKeysExpr(regexStream.str());
        const std::string newNodeValue = std::regex_replace(currentNodeValue, replaceKeysExpr, replaceEmptyKeysWith);
        if (newNodeValue != currentNodeValue) {
          xmlNodeSetContent(node, reinterpret_cast<const xmlChar*>(newNodeValue.c_str()));
        }
      }
    }
  }
  //Look into children
  for (xmlNode* child = node->children; child; child = child->next) {
    replaceValuesInNode(child, dict, replaceEmptyKeys, replaceEmptyKeysWith, keyPrefix, keyPostfix);
  }
  return true;
}

bool nodeToString(std::string& outSvg, const std::string& id, xmlNode* root, xmlDoc* doc) {
  xmlNode* target = findElement(root, id);
  xmlBufferPtr buffer = xmlBufferCreate();
  if (buffer == NULL) {
    return false;
  }
  xmlNodeDump(buffer, doc, target, 0, 1);
  outSvg = reinterpret_cast<const char*>(buffer->content);
  xmlBufferFree(buffer);
  return true;
}

xmlNode* findElementByTag(xmlNode* node, const std::string& tag) {
  if (node->type == XML_ELEMENT_NODE) {
    if (node->name != NULL) {
      const std::string thisNodeName = reinterpret_cast<const char*>(node->name);
      if (thisNodeName == tag) {
        std::cout << "Found " << tag << std::endl;
        return node;
      }
    }
  }
  for (xmlNode* child = node->children; child; child = child->next) {
    xmlNode* foundNode;
    if ((foundNode = findElementByTag(child, tag)) != nullptr) {
      return foundNode;
    }
  }
  return NULL;
}

bool setAttrToNodeById(xmlNode* root, const std::string id, const std::string attr, const std::string value) {
  xmlNode* target = findElement(root, id);
  return xmlSetProp(target, reinterpret_cast<const xmlChar*>(attr.c_str()), reinterpret_cast<const xmlChar*>(value.c_str())) != NULL;
}

int main(int argc, char* argv[]) {
  if (argc < 3) {
    std::cout << "Please provide svg file, id to find" << std::endl;
    return 1;
  }

  const std::string svgFile = argv[1];
  const std::string idToFind = argv[2];
  xmlDoc* doc = nullptr;
  xmlNode* root = nullptr;

  const std::string elementToNest = "<text font-weight=\"bold\" xml:space=\"preserve\" font-size=\"32\" id=\"ligne56\" y=\"36\" x=\"96\" stroke-opacity=\"null\" style=\"font-weight:bold;font-size:32px;font-family:Helvetica, Arial, sans-serif;text-align:center;fill:#0c0c0c;stroke:#fbc4b0;stroke-width:0\">Ligne 56</text>";
  
  xmlInitParser(); 

  if ((doc = xmlReadFile(svgFile.c_str(), NULL, 0)) == NULL) {
    std::cout << "Could not parse file " << svgFile << std::endl;
    return 1;
  }

  //Get root element
  root = xmlDocGetRootElement(doc);

  printElementNames(root);

  //Find element
  xmlNode* target = findElement(root, idToFind);
  if (target == nullptr) {
    std::cout << "Could not find " << idToFind << std::endl;
    xmlFreeDoc(doc);
    xmlCleanupParser();
    return 1;
  }
  target = findElementByTag(root, "text");
  if (target == nullptr) {
    std::cout << "Could not find " << "text" << std::endl;
    xmlFreeDoc(doc);
    xmlCleanupParser();
    return 1;
  }
  //Nest element
  if (!appendSvgElement(root, idToFind, elementToNest)) {
    std::cout << "Could not nest " << elementToNest << " into " << idToFind << std::endl;
    xmlFreeDoc(doc);
    xmlCleanupParser();
    return 1;
  }
  
  std::cout << "element successfully nested" << std::endl;

  //Replace value
  replaceValue(root, "testText", "aiutoooo");

  std::map<std::string, std::string> keys;
  keys["ligne"] = "56";

  replaceValuesInNode(root, keys, true, "???", "<", ">");

  //Set attr
  if (!setAttrToNodeById(root, "testLigne", "font-size", "36")) {
    std::cout << "Failed to set attribute" << std::endl;
  }

  //Encode xml to string
  xmlBufferPtr buffer = xmlBufferCreate();
  size_t bufSize = xmlNodeDump(buffer, doc, root, 0, 1);
  const std::string outXML = reinterpret_cast<const char*>(buffer->content);
  xmlBufferFree(buffer);

  std::cout << "\n\n\nOUT XML: \n" << outXML << std::endl;

  std::string schedulesSvg;
  nodeToString(schedulesSvg, "schedules", root, doc);
  std::cout << "\n\n\nSCHEDULES XML: \n\n\n" << schedulesSvg << "\n\n\n";

  xmlFreeDoc(doc);
  xmlCleanupParser();

  return 0;
}
