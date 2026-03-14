"use client";
import { useState } from "react";
import ConfirmModal from "./ConfirmModal";
export default function Logout({
  onFinallyAction,
}: {
  onFinallyAction?: () => void;
}) {
  const [loading, setLoading] = useState(false);
  const [showConfirm, setShowConfirm] = useState(false);
  async function logoutUser() {
    setLoading(true);
    try {
      const res = await fetch(`/api/auth/logout`, {
        method: "POST",
        credentials: "include",
        cache: "no-store",
      });
      if (res.ok) {
        window.location.href = "/";
      } else {
        console.error("Logout failed:", res.status);
      }
    } catch (err) {
      console.error("Logout request failed:", err);
    } finally {
      setLoading(false);
      onFinallyAction?.();
    }
  }
  return (
    <>
      <button
        className="rounded-xl bg-danger px-5 py-2 text-sm font-bold text-white transition-all hover:bg-danger/85 hover:shadow-secondary/20 active:scale-[0.985] cursor-pointer"
        onClick={() => setShowConfirm(true)}
      >
        Logout
      </button>
      {showConfirm && (
        <ConfirmModal
          title="Logout"
          description="Are you sure you want to log out?"
          confirmButtonClass="bg-danger text-light"
          confirmButtonText="log out"
          confirmFunction={logoutUser}
          onClose={() => setShowConfirm(false)}
          disableConfirm={loading}
        />
      )}
    </>
  );
}
