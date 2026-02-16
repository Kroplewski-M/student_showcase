export default function validateStudentId(id: string): string | null {
  const trimmed = id.trim();
  if (!trimmed) return "Student ID is required";
  if (!/^\d{7}$/.test(trimmed)) return "Student ID must be exactly 7 digits";
  return null;
}
const UUID_RE =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

export function isValidUuid(value: string): boolean {
  return UUID_RE.test(value);
}
export function getPasswordStrength(password: string) {
  if (!password) return { score: 0, label: "", bg: "", text: "" };
  let score = 0;
  if (password.length >= 8) score++;
  if (password.length >= 12) score++;
  if (/[a-z]/.test(password) && /[A-Z]/.test(password)) score++;
  if (/\d/.test(password)) score++;
  if (/[^a-zA-Z0-9]/.test(password)) score++;

  if (score <= 1)
    return { score: 1, label: "Weak", bg: "bg-danger", text: "text-danger" };
  if (score <= 2)
    return {
      score: 2,
      label: "Fair",
      bg: "bg-amber-500",
      text: "text-amber-500",
    };
  if (score <= 3)
    return {
      score: 3,
      label: "Good",
      bg: "bg-secondary",
      text: "text-secondary",
    };
  return {
    score: 4,
    label: "Strong",
    bg: "bg-emerald-400",
    text: "text-emerald-400",
  };
}
export function validatePassword(password: string): string | null {
  if (!password) return "Password is required";
  if (password.length < 5) return "Password must be at least 5 characters";
  if (password.length > 20) return "Password must be at most 20 characters";
  return null;
}
