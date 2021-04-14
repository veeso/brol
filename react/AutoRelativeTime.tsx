/**
 * @author Christian Visintin <christian.visintin1997@gmail.com>
 * @copyright: Christian Visintin (C) 2021
 */

import React from "react";
import { FormattedRelativeTime } from "react-intl";

interface OwnProps {
  seconds: number;
}

export default function AutoRelativeTime(props: OwnProps) {
  const days = Math.floor((props.seconds % 31536000) / 86400);
  const hours = Math.floor(((props.seconds % 31536000) % 86400) / 3600);
  const minutes = Math.floor(
    (((props.seconds % 31536000) % 86400) % 3600) / 60
  );
  if (days > 0) {
    return <FormattedRelativeTime value={-days} unit="day" />;
  } else if (hours > 0) {
    return <FormattedRelativeTime value={-hours} unit="hour" />;
  } else if (minutes > 0) {
    return <FormattedRelativeTime value={-minutes} unit="minute" />;
  } else {
    return <FormattedRelativeTime value={-props.seconds} unit="second" />;
  }
}

