interface ErrorDisplayProps {
  text: string | null;
}
export default function ErrorDisplay({ text }: ErrorDisplayProps) {
  return (
    <>
      {text && (
        <div
          className="px-4 py-3 text-sm leading-relaxed font-semibold bg-danger/10 border border-danger rounded-xl"
          role="alert"
        >
          <span className="text-white">{text}</span>
        </div>
      )}
    </>
  );
}
