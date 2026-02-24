"use client";

import { useState, useRef, useCallback } from "react";
import { createPortal } from "react-dom";
import { useRouter } from "next/navigation";
import { getProfileImgUrl, MAX_IMAGE_SIZE_BYTES } from "../lib/helpers";
import Camera from "../SVGS/Camera";
import Loading from "../SVGS/Loading";
import ImageIcon from "../SVGS/ImageIcon";

interface Props {
  onClose: () => void;
  currentImageName: string | null;
}

export default function UpdateImageForm({ onClose, currentImageName }: Props) {
  const [file, setFile] = useState<File | null>(null);
  const [preview, setPreview] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [dragOver, setDragOver] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const router = useRouter();

  const handleFile = (f: File) => {
    if (f.size > MAX_IMAGE_SIZE_BYTES) {
      setError("Image must be 5 MiB or smaller.");
      return;
    }
    setFile(f);
    setError(null);
    const url = URL.createObjectURL(f);
    setPreview(url);
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
      setError("Please select an image first.");
      return;
    }
    setLoading(true);
    setError(null);

    const formData = new FormData();
    formData.append("image", file);

    try {
      const res = await fetch("/api/user/update_image", {
        method: "POST",
        credentials: "include",
        body: formData,
      });

      if (!res.ok) {
        const data = await res.json().catch(() => null);
        setError(data?.message ?? "Failed to update image. Please try again.");
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

  const currentImageUrl = currentImageName
    ? getProfileImgUrl(currentImageName)
    : null;

  return createPortal(
    <div
      className="fixed z-[1000] h-screen w-screen left-0 top-0 bg-primary/85 backdrop-blur-[8px] p-5 animate-[fadeIn_0.3s_ease] flex items-center justify-center"
      onClick={(e) => e.target === e.currentTarget && onClose()}
    >
      <div className="w-full max-w-[480px] rounded-2xl border border-secondary/12 bg-primary/35 p-8 backdrop-blur-[20px]">
        <h2 className="pb-5 text-[22px] font-bold text-secondary">
          Update Profile Picture
        </h2>

        {/* Current / Preview image */}
        <div className="mb-6 flex justify-center">
          <div className="h-[100px] w-[100px] overflow-hidden rounded-full border-3 border-secondary/20 shadow-[0_8px_32px_rgba(0,0,0,0.3)]">
            {preview || currentImageUrl ? (
              // eslint-disable-next-line @next/next/no-img-element
              <img
                src={preview ?? currentImageUrl!}
                alt="Profile preview"
                className="h-full w-full object-cover"
              />
            ) : (
              <div className="flex h-full w-full items-center justify-center bg-[linear-gradient(135deg,var(--color-secondary)/0.2,var(--color-primary)/0.6)] text-secondary/40">
                <Camera />
              </div>
            )}
          </div>
        </div>

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
            <ImageIcon />
          </div>
          <span className="text-sm text-secondary/60">
            {file ? file.name : "Drop an image here or click to browse"}
          </span>
          <span className="text-xs text-secondary/35">
            JPEG or PNG · max 5 MB
          </span>
        </div>

        <input
          ref={fileInputRef}
          type="file"
          accept="image/jpeg,image/png"
          onChange={handleFileChange}
          className="hidden"
        />

        {/* Error */}
        {error && (
          <p className="mb-4 rounded-lg text-[13px] text-danger">{error}</p>
        )}

        {/* Actions */}
        <div className="flex justify-end gap-3">
          <button
            onClick={onClose}
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
                <Loading />
                Uploading…
              </>
            ) : (
              <>Save Photo</>
            )}
          </button>
        </div>
      </div>
    </div>,
    document.body,
  );
}
