"use client";

import { useState } from "react";
import StudentIdForm from "./StudentIdForm";

type AuthStep =
  | "student_id"
  | "login_password"
  | "register_password"
  | "check_email";

export default function AuthSteps() {
  const [step, setStep] = useState<AuthStep>("student_id");
  const [studentId, setStudentId] = useState("");
  const [errors, setErrors] = useState("");
  async function submitStudentId() {
    const res = await fetch("/api/auth/identify", {
      method: "POST",
      headers: { "content-type": "application/json" },
      credentials: "include",
      body: JSON.stringify({ studentId }),
    });
    const data = await res.json().catch(() => null);

    if (!res.ok) {
      setErrors(data?.error ?? "An unexpected error occurred.");
      return;
    }
    switch (data.next) {
      case "login":
        setStep("login_password");
        break;
      case "register":
        setStep("register_password");
        break;
      default:
        setStep("check_email");
    }
  }
  return (
    <main className="min-h-svh flex justify-center pt-24">
      <div className="w-full max-w-md rounded-xl p-6">
        {step === "student_id" && (
          <StudentIdForm
            studentId={studentId}
            onChangeAction={setStudentId}
            onSubmitAction={submitStudentId}
          />
        )}
      </div>
    </main>
  );
}
