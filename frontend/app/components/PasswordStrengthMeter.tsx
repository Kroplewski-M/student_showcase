type PasswordStrength = {
  score: number;
  label: string;
  bg: string;
  text: string;
};

export default function PasswordStrengthMeter({
  strength,
}: {
  strength: PasswordStrength;
}) {
  return (
    <div className="mt-2 flex items-center gap-2.5">
      <div className="flex flex-1 gap-1">
        {[1, 2, 3, 4].map((i) => (
          <div
            key={i}
            className={`h-1 flex-1 rounded-full transition-colors duration-300 ${
              i <= strength.score ? strength.bg : "bg-third/40"
            }`}
          />
        ))}
      </div>
      <span
        className={`text-[10px] font-semibold uppercase tracking-widest ${strength.text}`}
      >
        {strength.label}
      </span>
    </div>
  );
}
