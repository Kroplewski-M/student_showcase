"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { motion } from "framer-motion";
import Link from "next/link";
import { isValidUuid } from "@/app/helpers";

type Status = "loading" | "success" | "error";

export default function ValidateUserPage() {
  const { token } = useParams<{ token: string }>();
  const router = useRouter();
  const [status, setStatus] = useState<Status>("loading");

  useEffect(() => {
    if (!token || !isValidUuid(token)) {
      router.replace("/404");
      return;
    }

    const controller = new AbortController();

    async function verify() {
      try {
        const res = await fetch(`/api/auth/validate-user/${token}`, {
          method: "POST",
          signal: controller.signal,
        });

        if (res.ok) {
          setStatus("success");
        } else if (res.status === 400) {
          router.replace("/404");
        } else {
          setStatus("error");
        }
      } catch (err: unknown) {
        if (err instanceof DOMException && err.name === "AbortError") return;
        setStatus("error");
      }
    }

    verify();

    return () => controller.abort();
  }, [token, router]);

  return (
    <section className="relative flex min-h-screen items-center justify-center px-4 py-12">
      <div className="pointer-events-none absolute -top-1/3 -left-1/4 h-[80vw] w-[80vw] rounded-full bg-secondary/5 blur-3xl" />

      {status === "loading" && (
        <motion.div
          key="loading"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="relative z-10 flex flex-col items-center gap-4"
        >
          <svg
            className="animate-spin text-secondary"
            width="36"
            height="36"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
          >
            <path d="M12 2a10 10 0 0 1 10 10" />
          </svg>
          <p className="text-sm text-support">Verifying your account…</p>
        </motion.div>
      )}

      {status === "success" && (
        <motion.div
          key="success"
          initial={{ opacity: 0, y: 20, scale: 0.97 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
          className="relative z-10 w-full max-w-md rounded-2xl border border-third/40 bg-third/20 p-8 text-center backdrop-blur-sm"
        >
          <div className="mx-auto mb-5 flex h-16 w-16 items-center justify-center rounded-full bg-secondary/15">
            <svg
              width="32"
              height="32"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="text-secondary"
            >
              <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
              <polyline points="22 4 12 14.01 9 11.01" />
            </svg>
          </div>

          <h1 className="mb-2 text-2xl font-extrabold tracking-tight text-light">
            Account verified
          </h1>
          <p className="mb-7 text-sm text-support">
            Your email has been verified successfully. You can now sign in to
            your account.
          </p>

          <Link
            href="/login"
            className="block w-full rounded-xl bg-secondary py-3.5 text-center text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985]"
          >
            Go to Login
          </Link>
        </motion.div>
      )}

      {status === "error" && (
        <motion.div
          key="error"
          initial={{ opacity: 0, y: 20, scale: 0.97 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
          className="relative z-10 w-full max-w-md rounded-2xl border border-third/40 bg-third/20 p-8 text-center backdrop-blur-sm"
        >
          <div className="mx-auto mb-5 flex h-16 w-16 items-center justify-center rounded-full bg-danger/15">
            <svg
              width="32"
              height="32"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="text-danger"
            >
              <circle cx="12" cy="12" r="10" />
              <line x1="15" y1="9" x2="9" y2="15" />
              <line x1="9" y1="9" x2="15" y2="15" />
            </svg>
          </div>

          <h1 className="mb-2 text-2xl font-extrabold tracking-tight text-light">
            Something went wrong
          </h1>
          <p className="mb-7 text-sm text-support">
            We couldn&rsquo;t verify your account. Please try again later or
            contact support if the issue persists.
          </p>

          <Link
            href="/"
            className="block w-full rounded-xl bg-secondary py-3.5 text-center text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985]"
          >
            Go to Home
          </Link>
        </motion.div>
      )}
    </section>
  );
}
