/**
 * @brief get a node from a JSON using JSON query
 * @param object reference to data tree
 * @param string node: node to find in JSON notation "KEY1.KEY2.KEY3"
 * @return object reference to node found through the JSON query. NULL if not found
 */

function get_json_node($json, $node) {
    //Get nodes
    $nodes = explode(".", $node);
    //Iterate over node
    $current_node = &$json;
    foreach ($nodes as $inode) {
        //Check if inode exists in current node
        if (!array_key_exists($inode, $current_node)) {
            return NULL;
        }
        $current_node = &$current_node[$inode];
    }
    return $current_node;
}
