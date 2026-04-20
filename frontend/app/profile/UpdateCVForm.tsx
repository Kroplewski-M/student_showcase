"use client";

import { useState, useRef, useCallback } from "react";
import { createPortal } from "react-dom";
import { useRouter } from "next/navigation";
import { MAX_CV_SIZE_BYTES, MAX_CV_SIZE_MB } from "../lib/helpers";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFileLines, faSpinner } from "@fortawesome/free-solid-svg-icons";
import ErrorDisplay from "../components/ErrorDisplay";

interface Props {
  onClose: () => void;
}

export default function UpdateCVForm({ onClose }: Props) {
  const [file, setFile] = useState<File | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [dragOver, setDragOver] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  const handleFile = (f: File) => {
    if (f.type !== "application/pdf") {
      setError("Only PDF files are supported.");
      setFile(null);
      return;
    }
    if (f.size > MAX_CV_SIZE_BYTES) {
      setError(`CV must be ${MAX_CV_SIZE_MB} MiB or smaller.`);
      setFile(null);
      return;
    }
    setFile(f);
    setError(null);
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const f = e.target.files?.[0];
    if (f) handleFile(f);
  };

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setDragOver(false);
    const f = e.dataTransfer.files?.[0];
    if (f) handleFile(f);
  }, []);

  const handleSubmit = async () => {
    if (!file) {
      setError("Please select a PDF file first.");
      return;
    }
    setLoading(true);
    setError(null);

    const formData = new FormData();
    formData.append("cv", file);

    try {
      const res = await fetch("/api/user/update_cv", {
        method: "POST",
        credentials: "include",
        body: formData,
      });

      if (!res.ok) {
        const data = await res.json().catch(() => null);
        setError(data?.message ?? "Failed to update CV. Please try again.");
        return;
      }

      onClose();
      router.refresh();
    } catch {
      setError("Network error. Please check your connection.");
    } finally {
      setLoading(false);
    }
  };

  return createPortal(
    <div
      className="fixed z-[1000] h-screen w-screen left-0 top-0 bg-primary/85 backdrop-blur-[8px] p-5 animate-[fadeIn_0.3s_ease] flex items-center justify-center"
      onClick={(e) => e.target === e.currentTarget && !loading && onClose()}
    >
      <div className="w-full max-w-[480px] rounded-2xl border border-secondary/12 bg-primary/35 p-8 backdrop-blur-[20px]">
        <h2 className="pb-5 text-[22px] font-bold text-white">Update CV</h2>

        {/* Drop zone */}
        <div
          onClick={() => fileInputRef.current?.click()}
          onDragOver={(e) => {
            e.preventDefault();
            setDragOver(true);
          }}
          onDragLeave={() => setDragOver(false)}
          onDrop={handleDrop}
          className={`mb-5 flex cursor-pointer flex-col items-center gap-2 rounded-xl border-2 border-dashed p-7 transition-all duration-200 ease-in-out ${
            dragOver
              ? "border-secondary/50 bg-secondary/5"
              : "border-secondary/20 bg-primary/30"
          }`}
        >
          <div className="text-secondary/40">
            <FontAwesomeIcon icon={faFileLines} className="w-8 h-8" />
          </div>
          <span className="text-sm text-secondary/60">
            {file ? file.name : "Drop a PDF here or click to browse"}
          </span>
          <span className="text-xs text-secondary/35">PDF only</span>
          <span className="text-xs text-secondary/35 block">
            max {MAX_CV_SIZE_MB} MiB
          </span>
        </div>

        <input
          ref={fileInputRef}
          type="file"
          accept="application/pdf"
          onChange={handleFileChange}
          className="hidden"
        />

        {/* Error */}
        {error && (
          <div className="mb-2">
            <ErrorDisplay text={error} />{" "}
          </div>
        )}

        {/* Actions */}
        <div className="flex justify-end gap-3">
          <button
            onClick={onClose}
            disabled={loading}
            className="inline-flex cursor-pointer items-center gap-2 rounded-[10px] border border-secondary/20 bg-secondary/8 px-5 py-2.5 font-[Poppins] text-sm font-semibold text-secondary transition-all duration-250 ease-in-out hover:bg-secondary/15"
          >
            Cancel
          </button>
          <button
            onClick={handleSubmit}
            disabled={loading || !file}
            className={`inline-flex items-center gap-2 rounded-[10px] border-none px-5 py-2.5 font-[Poppins] text-sm font-semibold text-primary transition-all duration-250 ease-in-out ${
              loading || !file
                ? "cursor-not-allowed bg-secondary/30"
                : "cursor-pointer bg-[linear-gradient(135deg,var(--color-secondary),var(--color-support))] shadow-[0_4px_20px_var(--color-secondary)/0.25] hover:shadow-[0_6px_24px_var(--color-secondary)/0.35]"
            }`}
          >
            {loading ? (
              <>
                <FontAwesomeIcon
                  icon={faSpinner}
                  className="animate-spin w-[18px] h-[18px]"
                />
                Uploading…
              </>
            ) : (
              <>Save CV</>
            )}
          </button>
        </div>
      </div>
    </div>,
    document.body,
  );
}
