"use client";

import { useEffect, useState } from "react";
import { useParams, useRouter } from "next/navigation";
import { motion, AnimatePresence } from "framer-motion";
import Link from "next/link";
import EyeIcon from "@/app/SVGS/EyeIcon";
import {
  getPasswordStrength,
  isValidUuid,
  validatePassword,
} from "@/app/lib/helpers";
import Loading from "@/app/SVGS/Loading";
import CheckIcon from "@/app/SVGS/CheckIcon";
import ErrorSVG from "@/app/SVGS/ErrorSVG";
import SuccessfulReset from "../SuccessfulReset";
import PasswordStrengthMeter from "@/app/components/PasswordStrengthMeter";

function validateConfirmation(
  password: string,
  confirmation: string,
): string | null {
  if (!confirmation) return "Please confirm your password";
  if (password !== confirmation) return "Passwords do not match";
  return null;
}

type PageStatus = "loading" | "form" | "success" | "error";

type FormFields = {
  password: string;
  passwordConfirmation: string;
};

export default function ResetPasswordPage() {
  const { token } = useParams<{ token: string }>();
  const router = useRouter();

  const [status, setStatus] = useState<PageStatus>("loading");
  const [form, setForm] = useState<FormFields>({
    password: "",
    passwordConfirmation: "",
  });
  const [errors, setErrors] = useState<
    Partial<Record<keyof FormFields, string>>
  >({});
  const [touched, setTouched] = useState<
    Partial<Record<keyof FormFields, boolean>>
  >({});
  const [serverError, setServerError] = useState("");
  const [loading, setLoading] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirm, setShowConfirm] = useState(false);

  const strength = getPasswordStrength(form.password);

  // Validate token on mount
  useEffect(() => {
    if (!token || !isValidUuid(token)) {
      router.replace("/404");
      return;
    }

    const controller = new AbortController();

    async function checkToken() {
      try {
        const res = await fetch(`/api/auth/reset-password-exists/${token}`, {
          signal: controller.signal,
        });

        if (res.ok) {
          setStatus("form");
        } else if (res.status === 401) {
          router.replace("/404");
        } else {
          setStatus("error");
        }
      } catch (err: unknown) {
        if (err instanceof DOMException && err.name === "AbortError") return;
        setStatus("error");
      }
    }

    checkToken();

    return () => controller.abort();
  }, [token, router]);

  function validate(fields: FormFields = form) {
    const errs: Partial<Record<keyof FormFields, string>> = {};
    const pwErr = validatePassword(fields.password);
    if (pwErr) errs.password = pwErr;
    const confErr = validateConfirmation(
      fields.password,
      fields.passwordConfirmation,
    );
    if (confErr) errs.passwordConfirmation = confErr;
    return errs;
  }

  function handleChange(field: keyof FormFields, value: string) {
    const next = { ...form, [field]: value };
    setForm(next);
    setServerError("");
    if (touched[field]) {
      setErrors((prev) => {
        const updated = { ...prev };
        if (field === "password") {
          const e = validatePassword(value);
          if (e) updated.password = e;
          else delete updated.password;
          if (touched.passwordConfirmation) {
            const ce = validateConfirmation(value, next.passwordConfirmation);
            if (ce) updated.passwordConfirmation = ce;
            else delete updated.passwordConfirmation;
          }
        }
        if (field === "passwordConfirmation") {
          const e = validateConfirmation(next.password, value);
          if (e) updated.passwordConfirmation = e;
          else delete updated.passwordConfirmation;
        }
        return updated;
      });
    }
  }

  function handleBlur(field: keyof FormFields) {
    setTouched((prev) => ({ ...prev, [field]: true }));
    const errs = validate();
    if (errs[field]) {
      setErrors((prev) => ({ ...prev, [field]: errs[field] }));
    } else {
      setErrors((prev) => {
        const updated = { ...prev };
        delete updated[field];
        return updated;
      });
    }
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (loading) return;
    setTouched({ password: true, passwordConfirmation: true });
    const errs = validate();
    setErrors(errs);
    if (Object.keys(errs).length > 0) return;

    setLoading(true);
    setServerError("");

    try {
      const res = await fetch("/api/auth/reset-password-confirm", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          token,
          password: form.password,
          passwordConfirmation: form.passwordConfirmation,
        }),
      });

      if (res.ok) {
        setStatus("success");
      } else if (res.status === 401) {
        router.replace("/404");
      } else {
        const data = await res.json().catch(() => null);
        setServerError(
          data?.message ?? "Something went wrong. Please try again later.",
        );
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

      {/* Loading */}
      {status === "loading" && (
        <motion.div
          key="loading"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="relative z-10 flex flex-col items-center gap-4"
        >
          <Loading />
          <p className="text-sm text-support">Validating reset link…</p>
        </motion.div>
      )}

      {/* Password reset form */}
      {status === "form" && (
        <motion.div
          key="form"
          initial={{ opacity: 0, y: 20, scale: 0.97 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
          className="relative z-10 w-full max-w-md rounded-2xl border border-third/40 bg-third/20 p-8 backdrop-blur-sm"
        >
          <div className="mb-8">
            <h1 className="text-3xl font-extrabold tracking-tight text-light">
              Reset password
            </h1>
            <p className="mt-1 text-sm text-support">
              Choose a new password for your account
            </p>
          </div>

          <form onSubmit={handleSubmit} noValidate autoComplete="off">
            {/* Password */}
            <div className="mb-5">
              <label
                htmlFor="password"
                className="mb-1.5 block text-xs font-semibold uppercase tracking-wider text-support/70"
              >
                New password
              </label>
              <div className="relative">
                <input
                  id="password"
                  type={showPassword ? "text" : "password"}
                  maxLength={20}
                  className={`w-full rounded-xl border bg-primary/50 px-4 py-3 pr-11 text-sm text-light placeholder-support/40 outline-none transition-all focus:bg-primary/70 focus:ring-2 ${
                    touched.password && errors.password
                      ? "border-danger focus:ring-danger/30"
                      : "border-third/50 focus:border-secondary focus:ring-secondary/20"
                  }`}
                  placeholder="5–20 characters"
                  value={form.password}
                  onChange={(e) => handleChange("password", e.target.value)}
                  onBlur={() => handleBlur("password")}
                  disabled={loading}
                />
                <button
                  type="button"
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-support/50 transition-colors hover:text-support"
                  onClick={() => setShowPassword((v) => !v)}
                  tabIndex={-1}
                  aria-label={showPassword ? "Hide password" : "Show password"}
                >
                  <EyeIcon open={showPassword} />
                </button>
              </div>
              {form.password && <PasswordStrengthMeter strength={strength} />}
              {touched.password && errors.password && (
                <motion.p
                  initial={{ opacity: 0, y: -4 }}
                  animate={{ opacity: 1, y: 0 }}
                  className="mt-1.5 text-xs text-danger"
                >
                  {errors.password}
                </motion.p>
              )}
            </div>

            {/* Confirm password */}
            <div className="mb-6">
              <label
                htmlFor="confirmPassword"
                className="mb-1.5 block text-xs font-semibold uppercase tracking-wider text-support/70"
              >
                Confirm new password
              </label>
              <div className="relative">
                <input
                  id="confirmPassword"
                  type={showConfirm ? "text" : "password"}
                  maxLength={20}
                  className={`w-full rounded-xl border bg-primary/50 px-4 py-3 pr-11 text-sm text-light placeholder-support/40 outline-none transition-all focus:bg-primary/70 focus:ring-2 ${
                    touched.passwordConfirmation && errors.passwordConfirmation
                      ? "border-danger focus:ring-danger/30"
                      : "border-third/50 focus:border-secondary focus:ring-secondary/20"
                  }`}
                  placeholder="Re-enter your new password"
                  value={form.passwordConfirmation}
                  onChange={(e) =>
                    handleChange("passwordConfirmation", e.target.value)
                  }
                  onBlur={() => handleBlur("passwordConfirmation")}
                  disabled={loading}
                />
                <button
                  type="button"
                  className="absolute right-3 top-1/2 -translate-y-1/2 text-support/50 transition-colors hover:text-support"
                  onClick={() => setShowConfirm((v) => !v)}
                  tabIndex={-1}
                  aria-label={showConfirm ? "Hide password" : "Show password"}
                >
                  <EyeIcon open={showConfirm} />
                </button>
              </div>
              {touched.passwordConfirmation && errors.passwordConfirmation && (
                <motion.p
                  initial={{ opacity: 0, y: -4 }}
                  animate={{ opacity: 1, y: 0 }}
                  className="mt-1.5 text-xs text-danger"
                >
                  {errors.passwordConfirmation}
                </motion.p>
              )}
            </div>

            {/* Server error */}
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

            {/* Submit */}
            <button
              type="submit"
              disabled={loading}
              className="flex w-full items-center justify-center gap-2 rounded-xl bg-secondary py-3.5 text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985] disabled:cursor-not-allowed disabled:opacity-50 cursor-pointer"
            >
              {loading ? (
                <>
                  <Loading />
                  Resetting…
                </>
              ) : (
                "Reset password"
              )}
            </button>
          </form>
        </motion.div>
      )}

      {/* Success */}
      {status === "success" && <SuccessfulReset />}

      {/* Error */}
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
            We couldn&rsquo;t process your request. Please try again later or
            request a new reset link.
          </p>

          <Link
            href="/forgot-password"
            className="block w-full rounded-xl bg-secondary py-3.5 text-center text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985]"
          >
            Request new link
          </Link>
        </motion.div>
      )}
    </section>
  );
}
