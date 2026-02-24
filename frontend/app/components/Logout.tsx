"use client";
import { useState } from "react";
export default function Logout() {
  const [loading, setLoading] = useState(false);
  async function logoutUser() {
    setLoading(true);
    await fetch(`api/auth/logout`, {
      method: "POST",
      credentials: "include",
      cache: "no-store",
    }).then((x) => {
      setLoading(false);
      if (x.ok) {
        window.location.href = "/";
      }
    });
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
