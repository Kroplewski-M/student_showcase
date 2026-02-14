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
