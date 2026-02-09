"use client";
import { useState } from "react";
type Props = {
  studentId: string;
  onChangeAction: (value: string) => void;
  onSubmitAction: () => Promise<void> | void;
};
export default function StudentIdForm({
  studentId,
  onChangeAction,
  onSubmitAction,
}: Props) {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (!studentId.trim()) {
      setError("Student ID is required");
      return;
    }

    try {
      setError(null);
      setLoading(true);
      await onSubmitAction();
    } catch {
      setError("Something went wrong");
    } finally {
      setLoading(false);
    }
  }
  return (
    <form onSubmit={handleSubmit} className="w-full max-w-md sm:max-w-lg">
      <h1
        className="
        font-bold
        text-3xl sm:text-4xl md:text-5xl
        text-support
      "
      >
        Sign in
      </h1>

      <div className="mt-12 sm:mt-16 md:mt-20">
        <label
          htmlFor="studentId"
          className="
          block mb-3
          text-xs sm:text-lg
          uppercase 
          tracking-widest
          text-support/70
        "
        >
          Student ID
        </label>

        <div
          className="
          flex items-end gap-3
          border-b-2 border-support/40
          focus-within:border-secondary
          transition-colors
        "
        >
          <span
            className="
            pb-3
            text-xl sm:text-2xl md:text-3xl
            font-medium
            text-support/70
            select-none
          "
          >
            U
          </span>

          <input
            id="studentId"
            type="text"
            inputMode="numeric"
            pattern="[0-9]*"
            value={studentId}
            onChange={(e) => onChangeAction(e.target.value.replace(/\D/g, ""))}
            required
            placeholder="2272098"
            className="
            w-full
            bg-transparent
            py-3 sm:py-4
            text-xl sm:text-2xl md:text-3xl
            text-light
            placeholder-support/60
            tracking-wide
            outline-none
            transition-all duration-300 ease-out
            focus:drop-shadow-[0_0_16px_rgba(161,233,240,0.35)]
          "
          />
        </div>
      </div>

      {error && (
        <p className="mt-4 text-sm sm:text-base text-red-400">{error}</p>
      )}

      <button
        type="submit"
        disabled={loading}
        className="
        mt-12 sm:mt-14
        w-full
        rounded-xl
        bg-secondary
        px-6 sm:px-8
        py-4 sm:py-5
        text-lg sm:text-xl
        font-semibold
        tracking-wide
        text-primary

        transition-all duration-300 ease-out

        hover:brightness-110
        hover:shadow-[0_12px_36px_rgba(161,233,240,0.3)]
        active:scale-[0.97]
        cursor-pointer

        disabled:opacity-60
        disabled:shadow-none
      "
      >
        {loading ? "Checking…" : "Continue"}
      </button>
    </form>
  );
}
