"use client";
import { useState } from "react";
export default function Logout({
  onFinallyAction,
}: {
  onFinallyAction?: () => void;
}) {
  const [loading, setLoading] = useState(false);
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
    <button
      className="rounded-xl bg-danger px-5 py-2 text-sm font-bold text-white transition-all hover:bg-danger/85 hover:shadow-secondary/20 active:scale-[0.985] cursor-pointer"
      onClick={logoutUser}
      disabled={loading}
    >
      Logout
    </button>
  );
}
