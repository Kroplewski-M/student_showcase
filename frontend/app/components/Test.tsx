"use client";

import { useEffect, useState } from "react";

export default function Test() {
  const [hasCookie, setHasCookie] = useState("unknown");

  useEffect(() => {
    async function test() {
      // 1️⃣ Hit /health to SET cookie
      await fetch("/api/health", { credentials: "include" });

      // 2️⃣ Hit /health/check to READ cookie
      const res = await fetch("/api/health/check", {
        credentials: "include",
      });

      const text = await res.text();
      setHasCookie(text);
    }

    test();
  }, []);

  return <p>Cookie present: {hasCookie}</p>;
}
