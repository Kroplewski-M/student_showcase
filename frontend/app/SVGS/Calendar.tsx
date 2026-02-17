type CalendarProps = {
  date: number | undefined;
};
export default function Calendar({ date = 19 }: CalendarProps) {
  return (
    <svg
      viewBox="0 0 44 48"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      className="h-12 w-11 shrink-0"
      aria-hidden="true"
    >
      {/* Calendar body */}
      <rect
        x="1"
        y="6"
        width="42"
        height="38"
        rx="6"
        stroke="currentColor"
        strokeWidth="1.5"
        className="text-secondary/50"
      />
      {/* Top bar */}
      <rect
        x="1"
        y="6"
        width="42"
        height="12"
        rx="6"
        className="fill-secondary/15"
      />
      {/* Rings */}
      <rect
        x="12"
        y="2"
        width="2"
        height="8"
        rx="1"
        className="fill-secondary"
      />
      <rect
        x="30"
        y="2"
        width="2"
        height="8"
        rx="1"
        className="fill-secondary"
      />
      {/* Day number */}
      <text
        x="22"
        y="37"
        textAnchor="middle"
        className="fill-light text-[16px] font-bold"
        style={{ fontFamily: "system-ui, sans-serif" }}
      >
        {date}
      </text>
    </svg>
  );
}
