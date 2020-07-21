/**
 * @function getRowsFromRange
 * @description Convert a range string representation into number array
 * @description A row range can be defined using ',' (comma) to separate single rows or with '-' (dash) to define a range; both elements can be combined.
 * @example: '0,6-12' => [0,6,7,8,9,10,11,12]
 * @param {string} range
 * @returns {Array} null if the range is invalid
 */

function setRowsFromRange(range) {
  let rangeArray = [];
  let rangeRegex = new RegExp(/^[0123456789\-,]+$/);
  //Check if valid
  if (!rangeRegex.test(range)) {
    //Contains illegale characters
    return null;
  }

  //Check if ends or start with separator
  if (
    range.startsWith("-") ||
    range.startsWith(",") ||
    range.endsWith("-") ||
    range.endsWith(",")
  ) {
    //Must start with number
    return null;
  }

  //Split range into tokens separated by ','
  let rangeTokens = range.split(",");

  //Iterate over tokens
  for (let token of rangeTokens) {
    if (token.includes("-")) {
      //Is a range
      let bounds = token.split("-");
      let upperBound = parseInt(bounds[1]);
      let lowerBound = parseInt(bounds[0]);
      let subRange = Array.from(
        new Array(upperBound - lowerBound + 1),
        (x, i) => i + lowerBound
      );
      rangeArray = rangeArray.concat(subRange);
    } else {
      //Is a single token
      rangeArray.push(parseInt(token));
    }
  }
  return rangeArray;
}
