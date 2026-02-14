export default function validateStudentId(id: string): string | null {
  const trimmed = id.trim();
  if (!trimmed) return "Student ID is required";
  if (!/^\d{7}$/.test(trimmed)) return "Student ID must be exactly 7 digits";
  return null;
}
