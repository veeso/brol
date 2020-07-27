/**
 * @author Christian Visintin
 */

import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeFormatterBuilder;
import java.time.format.DateTimeParseException;

public class ISO8601 {

  /**
   * <p>
   * Convert a iso8601 string to LocalDateTime
   * </p>
   * 
   * @param dateStr
   * @return LocalDateTime
   * @throws DateTimeParseException
   */

  public static LocalDateTime toLocalDateTime(String dateStr) throws DateTimeParseException {
    DateTimeFormatter formatter = new DateTimeFormatterBuilder().optionalStart().appendPattern("'+'").optionalEnd()
        // date/time
        .append(DateTimeFormatter.ISO_LOCAL_DATE_TIME)
        // offset (hh:mm - "+00:00" when it's zero)
        .optionalStart().appendOffset("+HH:MM", "+00:00").optionalEnd()
        // offset (hhmm - "+0000" when it's zero)
        .optionalStart().appendOffset("+HHMM", "+0000").optionalEnd()
        // offset (hh - "Z" when it's zero)
        .optionalStart().appendOffset("+HH", "Z").optionalEnd()
        // create formatter
        .toFormatter();
    return LocalDateTime.parse(dateStr, formatter);
  }

}
