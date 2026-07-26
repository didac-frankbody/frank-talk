/**
 * Shared date/time formatters for the message timeline.
 *
 * - `formatTime` — short clock time ("2:34 PM"), used in message rows.
 * - `formatFullDateTime` — verbose string for tooltips
 *   ("Wednesday, April 2, 2026 at 2:34 PM").
 * - `formatDayHeading` — label for day dividers / sticky headers. Relative
 *   inside a week ("Today", "Yesterday", "Tue"), absolute beyond it
 *   ("12 Mar 2026"). Never ISO.
 * - `isSameDay` — compare two unix-second timestamps.
 */

const TIME_FORMATTER = new Intl.DateTimeFormat("en-US", {
  hour: "numeric",
  minute: "2-digit",
});

const DAY_PERIOD_SUFFIX_RE = /[\s\u00a0\u202f]*(?:AM|PM)$/i;

const FULL_DATE_TIME_FORMATTER = new Intl.DateTimeFormat("en-US", {
  weekday: "long",
  year: "numeric",
  month: "long",
  day: "numeric",
  hour: "numeric",
  minute: "2-digit",
});

/** Abbreviated weekday for the within-a-week day dividers, e.g. "Tue". */
const SHORT_WEEKDAY_FORMATTER = new Intl.DateTimeFormat("en-US", {
  weekday: "short",
});

/**
 * Absolute day label for anything older than a week, e.g. "12 Mar 2026".
 *
 * Day-month-year with an abbreviated month, per the brand's date rule — never
 * ISO, and never a bare numeric date that reads differently either side of the
 * Atlantic.
 */
const ABSOLUTE_DAY_FORMATTER = new Intl.DateTimeFormat("en-GB", {
  day: "numeric",
  month: "short",
  year: "numeric",
});

/** Days inside which a day divider stays relative rather than absolute. */
const RELATIVE_DAY_WINDOW = 7;

const SHORT_MONTH_FORMATTER = new Intl.DateTimeFormat("en-US", {
  month: "short",
});

/** Short clock time, e.g. "2:34 PM". */
export function formatTime(unixSeconds: number): string {
  return TIME_FORMATTER.format(new Date(unixSeconds * 1_000));
}

/** Short clock time with the AM/PM marker removed, e.g. "2:34". */
export function formatTimeWithoutDayPeriod(time: string): string {
  return time.replace(DAY_PERIOD_SUFFIX_RE, "").trim();
}

/** Full date + time for tooltips, e.g. "Wednesday, April 2, 2026 at 2:34 PM". */
export function formatFullDateTime(unixSeconds: number): string {
  return FULL_DATE_TIME_FORMATTER.format(new Date(unixSeconds * 1_000));
}

/**
 * Human-friendly day label for dividers and sticky headers.
 *
 * Relative inside a week — "Today", "Yesterday", then an abbreviated weekday
 * like "Tue" — and absolute beyond it, as "12 Mar 2026". This is the brand's
 * date rule: relative while a reader still has the week in their head, an
 * unambiguous day-month-year once they don't, and never an ISO string.
 *
 * The app splits date and time across two elements — the divider carries the
 * day and `MessageTimestamp` carries the clock time — so a single row reads as
 * the handoff's "Tue 4:02pm" without either piece repeating the other.
 */
export function formatDayHeading(unixSeconds: number): string {
  const date = new Date(unixSeconds * 1_000);
  const now = new Date();

  if (isSameDayDate(date, now)) {
    return "Today";
  }

  const yesterday = new Date(now);
  yesterday.setDate(yesterday.getDate() - 1);
  if (isSameDayDate(date, yesterday)) {
    return "Yesterday";
  }

  // Compare calendar days, not elapsed milliseconds, so "6 days ago" doesn't
  // flip to absolute partway through the day depending on the clock time.
  const daysApart = Math.round(
    (startOfLocalDaySeconds(Math.floor(now.getTime() / 1_000)) -
      startOfLocalDaySeconds(unixSeconds)) /
      86_400,
  );

  if (daysApart > 0 && daysApart < RELATIVE_DAY_WINDOW) {
    return SHORT_WEEKDAY_FORMATTER.format(date);
  }

  return ABSOLUTE_DAY_FORMATTER.format(date);
}

/** True when two unix-second timestamps fall on the same calendar day (local time). */
export function isSameDay(a: number, b: number): boolean {
  return isSameDayDate(new Date(a * 1_000), new Date(b * 1_000));
}

/**
 * Unix-seconds timestamp of local midnight for the calendar day containing
 * `unixSeconds`. Two timestamps on the same calendar day map to the same value,
 * so it is a stable identifier for a day group that does not shift when an
 * older message is prepended into that day.
 */
export function startOfLocalDaySeconds(unixSeconds: number): number {
  const date = new Date(unixSeconds * 1_000);
  date.setHours(0, 0, 0, 0);
  return Math.floor(date.getTime() / 1_000);
}

/** Short month + ordinal day, e.g. "May 19th". */
export function formatShortMonthDayOrdinal(unixSeconds: number): string {
  return formatMonthDayOrdinal(
    new Date(unixSeconds * 1_000),
    SHORT_MONTH_FORMATTER,
  );
}

/**
 * Relative thread-summary timestamp with expanded units, e.g. "3 hours ago",
 * falling back to "on May 19th" for older replies.
 */
export function formatThreadSummaryLastReplyTime(
  unixSeconds: number,
  nowSeconds = Date.now() / 1_000,
): string {
  const diff = Math.max(0, nowSeconds - unixSeconds);

  if (diff < 60) return "just now";
  if (diff < 3_600) return formatAgo(Math.floor(diff / 60), "minute");
  if (diff < 86_400) return formatAgo(Math.floor(diff / 3_600), "hour");
  if (diff < 604_800) return formatAgo(Math.floor(diff / 86_400), "day");

  return `on ${formatShortMonthDayOrdinal(unixSeconds)}`;
}

function isSameDayDate(a: Date, b: Date): boolean {
  return (
    a.getFullYear() === b.getFullYear() &&
    a.getMonth() === b.getMonth() &&
    a.getDate() === b.getDate()
  );
}

function formatMonthDayOrdinal(
  date: Date,
  monthFormatter: Intl.DateTimeFormat,
): string {
  return `${monthFormatter.format(date)} ${date.getDate()}${ordinalSuffix(
    date.getDate(),
  )}`;
}

function formatAgo(value: number, unit: string): string {
  return `${value} ${unit}${value === 1 ? "" : "s"} ago`;
}

function ordinalSuffix(day: number): string {
  const lastTwoDigits = day % 100;
  if (lastTwoDigits >= 11 && lastTwoDigits <= 13) {
    return "th";
  }

  switch (day % 10) {
    case 1:
      return "st";
    case 2:
      return "nd";
    case 3:
      return "rd";
    default:
      return "th";
  }
}
