"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { motion } from "framer-motion";
import Link from "next/link";
import { isValidUuid } from "@/app/helpers";
import CheckIcon from "@/app/SVGS/CheckIcon";
import Loading from "@/app/SVGS/Loading";
import ErrorSVG from "@/app/SVGS/ErrorSVG";

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
          <Loading />
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
            <CheckIcon />
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
            <ErrorSVG />
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
