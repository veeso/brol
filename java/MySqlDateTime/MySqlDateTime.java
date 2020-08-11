/**
 * @author Christian Visintin <christian.visintin1997@gmail.com>
 * @version 0.1.0
 */

import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeFormatterBuilder;
import java.time.format.DateTimeParseException;

public class MySqlDateTime {

  /**
   * <p>
   * Convert a MySqlDateTime string to LocalDateTime
   * </p>
   * 
   * @param dateStr
   * @return LocalDateTime
   * @throws DateTimeParseException
   */

  public static LocalDateTime parse(String dateStr) throws DateTimeParseException {
    DateTimeFormatter formatter = new DateTimeFormatterBuilder()
        .parseCaseInsensitive()
        .appendPattern("uuuu-MM-dd HH:mm:ss")
        .appendPattern("[.SSSSSSSSS][.SSSSSS][.SSS][.SS][.S]")
        // create formatter
        .toFormatter();
    return LocalDateTime.parse(dateStr, formatter);
  }

}
