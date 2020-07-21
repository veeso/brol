/**
    @function strptime
    @description converts time string format to date string
    @param String format
    @param Date
    @returns String
   */

function strptime(format, date) {
  let last = -2;
  let result = "";
  let hour = date.getHours();

  /* Expand aliases */
  format = format.replace(/%D/, "%m/%d/%y");
  format = format.replace(/%R/, "%H:%M");
  format = format.replace(/%T/, "%H:%M:%S");

  /* Note: we fail on strings without format characters */

  while (1) {
    /* find next format char */
    let pos = format.indexOf("%", last + 2);

    if (-1 === pos) {
      /* dump rest of text if no more format chars */
      result += format.substr(last + 2);
      break;
    } else {
      /* dump text after last format code */
      result += format.substr(last + 2, pos - (last + 2));

      /* apply format code */
      let formatChar = format.charAt(pos + 1);
      switch (formatChar) {
        case "%":
          result += "%";
          break;
        case "C":
          result += date.getYear();
          break;
        case "H":
        case "k":
          if (hour < 10) result += "0";
          result += hour;
          break;
        case "M":
          if (date.getMinutes() < 10) result += "0";
          result += date.getMinutes();
          break;
        case "S":
          if (date.getSeconds() < 10) result += "0";
          result += date.getSeconds();
          break;
        case "m":
          if (date.getMonth() < 10) result += "0";
          result += date.getMonth();
          break;
        case "a":
          result += dt.toLocaleString(navigator.languages[0], {
            month: "short",
          });
          break;
        case "A":
          result += dt.toLocaleString(navigator.languages[0], {
            month: "long",
          });
          break;
        case "b":
          result += dt.toLocaleString(navigator.languages[0], {
            weekday: "short",
          });
          break;
        case "B":
        case "h":
          result += dt.toLocaleString(navigator.languages[0], {
            weekday: "long",
          });
          break;
        case "Y":
          result += date.getFullYear();
          break;
        case "d":
        case "e":
          if (date.getDate() < 10) result += "0";
          result += date.getDate();
          break;
        case "w":
          result += date.getDay();
          break;
        case "p":
        case "P":
          if (hour < 12) {
            result += "am";
          } else {
            result += "pm";
          }
          break;
        case "l":
        case "I":
          if (hour % 12 < 10) result += "0";
          result += hour % 12;
          break;
      }
    }
    last = pos;
  }
  return result;
}
