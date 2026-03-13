import { createPortal } from "react-dom";
import Close from "../SVGS/Close";

interface ConfirmModalProps {
  title: string;
  description: string;
  confirmButtonText: string;
  confirmButtonClass: string;
  confirmFunction: () => void;
  onClose: () => void;
}

export default function ConfirmModal({
  title,
  description,
  confirmButtonText,
  confirmButtonClass,
  confirmFunction,
  onClose,
}: ConfirmModalProps) {
  return createPortal(
    <div className="fixed z-[1000] h-screen w-screen left-0 top-0 bg-primary/85 backdrop-blur-[8px] p-5 animate-[fadeIn_0.3s_ease] flex items-center justify-center">
      <div className="w-full max-w-[420px] flex flex-col rounded-2xl border border-secondary/12 bg-primary/35 backdrop-blur-[20px]">
        {/* Header */}
        <div className="flex items-center justify-between px-8 pt-7 pb-5 border-b border-secondary/10">
          <h2 className="text-[22px] font-bold text-white">{title}</h2>
          <button
            type="button"
            onClick={onClose}
            className="flex h-8 w-8 items-center justify-center rounded-lg text-secondary/50 transition-colors hover:bg-secondary/10 hover:text-secondary cursor-pointer"
          >
            <Close />
          </button>
        </div>

        {/* Body */}
        <div className="px-8 py-6">
          <p className="text-sm leading-relaxed text-secondary/70">
            {description}
          </p>
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-3  px-8 py-5">
          <button
            type="button"
            onClick={onClose}
            className="inline-flex cursor-pointer items-center gap-2 rounded-[10px] border border-secondary/20 bg-secondary/8 px-5 py-2.5 font-[Poppins] text-sm font-semibold text-secondary transition-all duration-250 ease-in-out hover:bg-secondary/15"
          >
            Cancel
          </button>
          <button
            type="button"
            onClick={confirmFunction}
            className={`inline-flex cursor-pointer items-center gap-2 rounded-[10px] border-none px-5 py-2.5 font-[Poppins] text-sm font-semibold transition-all duration-250 ease-in-out ${confirmButtonClass}`}
          >
            {confirmButtonText}
          </button>
        </div>
      </div>
    </div>,
    document.body,
  );
}
