"use client";
import { useState } from "react";
export default function Logout({ onFinally }: { onFinally?: () => void }) {
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
      onFinally?.();
    }
  }
  return (
    <button
      className="text-white bg-danger rounded-xl p-3 cursor-pointer hover:bg-danger/90"
      onClick={logoutUser}
      disabled={loading}
    >
      Logout
    </button>
  );
}
