"use client";

import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import Link from "next/link";
import validateStudentId from "@/app/lib/helpers";
import ConfirmForgotPassword from "./ConfirmForgotPassword";
import Loading from "@/app/SVGS/Loading";

export default function ForgotPasswordPage() {
  const [studentId, setStudentId] = useState("");
  const [error, setError] = useState("");
  const [touched, setTouched] = useState(false);
  const [serverError, setServerError] = useState("");
  const [loading, setLoading] = useState(false);
  const [submitted, setSubmitted] = useState(false);

  function handleChange(value: string) {
    setStudentId(value);
    setServerError("");
    if (touched) {
      const e = validateStudentId(value);
      setError(e ?? "");
    }
  }

  function handleBlur() {
    setTouched(true);
    const e = validateStudentId(studentId);
    setError(e ?? "");
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (loading) return;
    setTouched(true);
    const err = validateStudentId(studentId);
    setError(err ?? "");
    if (err) return;

    setLoading(true);
    setServerError("");

    try {
      const res = await fetch("/api/auth/reset-password", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ id: studentId.trim() }),
      });

      if (res.ok) {
        setSubmitted(true);
      } else {
        setServerError("Something went wrong. Please try again later.");
      }
    } catch {
      setServerError(
        "Unable to connect to the server. Please check your connection.",
      );
    } finally {
      setLoading(false);
    }
  }

  return (
    <section className="relative flex min-h-screen items-center justify-center px-4 py-12">
      <div className="pointer-events-none absolute -top-1/3 -left-1/4 h-[80vw] w-[80vw] rounded-full bg-secondary/5 blur-3xl" />

      <AnimatePresence mode="wait">
        {submitted ? (
          <ConfirmForgotPassword />
        ) : (
          <motion.div
            key="form"
            initial={{ opacity: 0, y: 20, scale: 0.97 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -20 }}
            transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
            className="relative z-10 w-full max-w-md rounded-2xl border border-third/40 bg-third/20 p-8 backdrop-blur-sm"
          >
            <div className="mb-8">
              <h1 className="text-3xl font-extrabold tracking-tight text-light">
                Forgot password
              </h1>
              <p className="mt-1 text-sm text-support">
                Enter your Student ID and we&rsquo;ll send you a reset link
              </p>
            </div>

            <form onSubmit={handleSubmit} noValidate autoComplete="off">
              <div className="mb-6">
                <label
                  htmlFor="studentId"
                  className="mb-1.5 block text-xs font-semibold uppercase tracking-wider text-support/70"
                >
                  Student ID
                </label>
                <input
                  id="studentId"
                  type="text"
                  inputMode="numeric"
                  pattern="\d*"
                  maxLength={7}
                  className={`w-full rounded-xl border bg-primary/50 px-4 py-3 text-sm text-light placeholder-support/40 outline-none transition-all focus:bg-primary/70 focus:ring-2 ${
                    touched && error
                      ? "border-danger focus:ring-danger/30"
                      : "border-third/50 focus:border-secondary focus:ring-secondary/20"
                  }`}
                  placeholder="e.g. 2272098"
                  value={studentId}
                  onChange={(e) =>
                    handleChange(e.target.value.replace(/\D/g, ""))
                  }
                  onBlur={handleBlur}
                  disabled={loading}
                />
                {touched && error && (
                  <motion.p
                    initial={{ opacity: 0, y: -4 }}
                    animate={{ opacity: 1, y: 0 }}
                    className="mt-1.5 text-xs text-danger"
                  >
                    {error}
                  </motion.p>
                )}
              </div>

              <AnimatePresence>
                {serverError && (
                  <motion.div
                    initial={{ opacity: 0, height: 0 }}
                    animate={{ opacity: 1, height: "auto" }}
                    exit={{ opacity: 0, height: 0 }}
                    className="mb-5 overflow-hidden"
                  >
                    <div
                      className="px-4 py-3 text-sm leading-relaxed font-bold text-danger"
                      role="alert"
                    >
                      {serverError}
                    </div>
                  </motion.div>
                )}
              </AnimatePresence>

              <button
                type="submit"
                disabled={loading}
                className="flex w-full items-center justify-center gap-2 rounded-xl bg-secondary py-3.5 text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985] disabled:cursor-not-allowed disabled:opacity-50 cursor-pointer"
              >
                {loading ? (
                  <>
                    <Loading />
                    Sending…
                  </>
                ) : (
                  "Send reset link"
                )}
              </button>
            </form>

            <p className="mt-6 text-center text-xs text-support/60">
              Remember your password?{" "}
              <Link
                href="/login"
                className="font-medium text-secondary transition-colors hover:text-secondary/80"
              >
                Login
              </Link>
            </p>
          </motion.div>
        )}
      </AnimatePresence>
    </section>
  );
}
