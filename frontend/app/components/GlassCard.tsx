export default function GlassCard({
  children,
  className = "",
}: {
  children: React.ReactNode;
  className?: string;
}) {
  return (
    <div
      className={`${className} rounded-2xl border border-secondary/12 bg-primary/35 backdrop-blur-[20px]`}
    >
      {children}
    </div>
  );
}
