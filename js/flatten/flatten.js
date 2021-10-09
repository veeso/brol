/**
 * @description flatten a JS object
 * @param {object} dict
 * @returns {object}
 */

function flatten(dict) {
  const iterNode = (flatten, path, node) => {
    for (const key of Object.keys(node)) {
      const child = node[key];
      const childKey = path ? path + "." + key : key;
      if (typeof child === "object") {
        flatten = iterNode(flatten, childKey, child);
      } else {
        flatten[childKey] = child;
      }
    }
    return flatten;
  };

  return iterNode({}, null, dict);
}
