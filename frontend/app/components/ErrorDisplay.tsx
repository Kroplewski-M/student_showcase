interface ErrorDisplayProps {
  text: string | null;
}
export default function ErrorDisplay({ text }: ErrorDisplayProps) {
  return (
    <>
      {text && (
        <div
          className="px-4 py-3 text-sm leading-relaxed text-danger font-semibold bg-danger/10 border border-danger rounded-xl"
          role="alert"
        >
          {text}
        </div>
      )}
    </>
  );
}
