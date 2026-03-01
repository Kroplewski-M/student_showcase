export const MAX_IMAGE_SIZE_BYTES = 5 * 1024 * 1024; // 5 MiB
export const MAX_IMAGE_SIZE_MB = MAX_IMAGE_SIZE_BYTES / (1024 * 1024);
export const ALLOWED_IMAGE_TYPES = [
  "image/jpeg",
  "image/png",
  "image/webp",
  "image/gif",
];
export const ALLOWED_IMAGE_EXTENSIONS = ["jpeg", "jpg", "png", "webp", "gif"];
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

export function getProfileImgUrl(img_name: string) {
  return `/uploads/user_images/${encodeURIComponent(img_name)}`;
}

export function validateLinkUrl(linkType: string, url: string): string | null {
  let hostname: string;
  try {
    hostname = new URL(url).hostname.replace(/^www\./, "");
  } catch {
    return "Enter a valid URL";
  }

  switch (linkType.toLowerCase()) {
    case "github":
      if (hostname !== "github.com") return "Must be a GitHub URL (github.com)";
      break;
    case "youtube":
      if (hostname !== "youtube.com" && hostname !== "youtu.be")
        return "Must be a YouTube URL (youtube.com)";
      break;
    case "linkedin":
      if (hostname !== "linkedin.com")
        return "Must be a LinkedIn URL (linkedin.com)";
      break;
    case "gitlab":
      if (hostname !== "gitlab.com")
        return "Must be a GitLab URL (gitlab.com)";
      break;
    case "bitbucket":
      if (hostname !== "bitbucket.org")
        return "Must be a Bitbucket URL (bitbucket.org)";
      break;
    case "stack overflow":
      if (hostname !== "stackoverflow.com")
        return "Must be a Stack Overflow URL (stackoverflow.com)";
      break;
    case "figma":
      if (hostname !== "figma.com")
        return "Must be a Figma URL (figma.com)";
      break;
    // "live preview" and any unknown type: any safe URL is fine
  }

  return null;
}

export function isSafeLink(link: string) {
  try {
    const url = new URL(link);
    if (url.protocol !== "https:" && url.protocol !== "http:") return false;
    if (
      url.hostname === "localhost" ||
      /^(127\.|0\.|10\.|172\.(1[6-9]|2\d|3[01])\.|192\.168\.)/.test(
        url.hostname,
      ) ||
      url.hostname === "[::1]" ||
      /^\[::ffff:/i.test(url.hostname) ||
      /^\[f[cd][0-9a-f]{2}:/i.test(url.hostname) ||
      /^\[fe[89ab][0-9a-f]:/i.test(url.hostname)
    ) {
      return false;
    }
    return true;
  } catch {
    return false;
  }
}
