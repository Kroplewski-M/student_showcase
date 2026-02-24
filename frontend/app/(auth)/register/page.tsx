"use client";

import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import Link from "next/link";
import EyeIcon from "@/app/SVGS/EyeIcon";
import ConfirmedRegister from "./ConfirmedRegister";
import Loading from "@/app/SVGS/Loading";
import validateStudentId, {
  getPasswordStrength,
  validatePassword,
} from "@/app/lib/helpers";
import PasswordStrengthMeter from "@/app/components/PasswordStrengthMeter";
import LogoWritten from "@/app/components/LogoWritten";
import ErrorDisplay from "@/app/components/ErrorDisplay";

function validateConfirmation(
  password: string,
  confirmation: string,
): string | null {
  if (!confirmation) return "Please confirm your password";
  if (password !== confirmation) return "Passwords do not match";
  return null;
}
type FormFields = {
  id: string;
  password: string;
  passwordConfirmation: string;
};

export default function RegisterPage() {
  const [form, setForm] = useState<FormFields>({
    id: "",
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
  const [registered, setRegistered] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [showConfirm, setShowConfirm] = useState(false);

  const strength = getPasswordStrength(form.password);

  function validate(fields: FormFields = form) {
    const errs: Partial<Record<keyof FormFields, string>> = {};
    const idErr = validateStudentId(fields.id);
    if (idErr) errs.id = idErr;
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
        if (field === "id") {
          const e = validateStudentId(value);
          if (e) updated.id = e;
          else delete updated.id;
        }
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
    setTouched({ id: true, password: true, passwordConfirmation: true });
    const errs = validate();
    setErrors(errs);
    if (Object.keys(errs).length > 0) return;

    setLoading(true);
    setServerError("");

    try {
      const res = await fetch(`/api/auth/register`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          id: form.id.trim(),
          password: form.password,
          passwordConfirmation: form.passwordConfirmation,
        }),
      });

      if (res.status === 201) {
        setRegistered(true);
        return;
      }

      const data = await res.json().catch(() => null);

      if (res.status === 409) {
        setServerError("An account with this Student ID already exists.");
      } else if (res.status === 400 && data?.message) {
        setServerError(data.message);
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
    <section className="relative min-h-screen flex items-center justify-center px-4 py-12 flex-col">
      <div className="text-3xl mb-5">
        <Link href="/">
          <LogoWritten />
        </Link>
      </div>
      {/* Background accent glow */}
      <div className="pointer-events-none absolute -top-1/3 -left-1/4 h-[80vw] w-[80vw] rounded-full bg-secondary/5 blur-3xl" />
      <AnimatePresence mode="wait">
        {registered ? (
          <ConfirmedRegister />
        ) : (
          <motion.div
            key="form"
            initial={{ opacity: 0, y: 20, scale: 0.97 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -20 }}
            transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
            className="relative z-10 w-full max-w-md rounded-2xl border border-third/40 bg-third/20 backdrop-blur-sm p-8"
          >
            {/* Header */}
            <div className="mb-8">
              <h1 className="text-3xl font-extrabold tracking-tight text-light">
                Create account
              </h1>
              <p className="mt-1 text-sm text-support">
                Register with your student credentials
              </p>
            </div>

            <form onSubmit={handleSubmit} noValidate autoComplete="off">
              {/* Student ID */}
              <div className="mb-5">
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
                    touched.id && errors.id
                      ? "border-danger focus:ring-danger/30"
                      : "border-third/50 focus:border-secondary focus:ring-secondary/20"
                  }`}
                  placeholder="e.g. 2272098"
                  value={form.id}
                  onChange={(e) =>
                    handleChange("id", e.target.value.replace(/\D/g, ""))
                  }
                  onBlur={() => handleBlur("id")}
                  disabled={loading}
                />
                {touched.id && errors.id && (
                  <motion.p
                    initial={{ opacity: 0, y: -4 }}
                    animate={{ opacity: 1, y: 0 }}
                    className="mt-1.5 text-xs text-danger"
                  >
                    {errors.id}
                  </motion.p>
                )}
              </div>

              {/* Password */}
              <div className="mb-5">
                <label
                  htmlFor="password"
                  className="mb-1.5 block text-xs font-semibold uppercase tracking-wider text-support/70"
                >
                  Password
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
                    aria-label={
                      showPassword ? "Hide password" : "Show password"
                    }
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

              {/* Confirm Password */}
              <div className="mb-6">
                <label
                  htmlFor="confirmPassword"
                  className="mb-1.5 block text-xs font-semibold uppercase tracking-wider text-support/70"
                >
                  Confirm password
                </label>
                <div className="relative">
                  <input
                    id="confirmPassword"
                    type={showConfirm ? "text" : "password"}
                    maxLength={20}
                    className={`w-full rounded-xl border bg-primary/50 px-4 py-3 pr-11 text-sm text-light placeholder-support/40 outline-none transition-all focus:bg-primary/70 focus:ring-2 ${
                      touched.passwordConfirmation &&
                      errors.passwordConfirmation
                        ? "border-danger focus:ring-danger/30"
                        : "border-third/50 focus:border-secondary focus:ring-secondary/20"
                    }`}
                    placeholder="Re-enter your password"
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
                {touched.passwordConfirmation &&
                  errors.passwordConfirmation && (
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
                    <ErrorDisplay text={serverError} />
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
                    Creating account…
                  </>
                ) : (
                  "Create account"
                )}
              </button>
            </form>

            <p className="mt-6 text-center text-xs text-support/60">
              Already have an account?{" "}
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
