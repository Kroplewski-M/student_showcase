import { cookies } from "next/headers";
import { AuthenticatedUser } from "./dtos";

export async function authFetch(
  url: string,
  options: RequestInit = {},
): Promise<Response> {
  const cookieStore = await cookies();
  const cookieName = process.env.COOKIE_NAME;
  if (!cookieName) {
    throw new Error("COOKIE_NAME env not set");
  }
  const sessionCookie = cookieStore.get(cookieName);
  if (!sessionCookie) {
    return fetch(url, options);
  }
  return fetch(url, {
    ...options,
    headers: {
      ...options.headers,
      Cookie: `${cookieName}=${sessionCookie?.value}`,
    },
  });
}

export async function getUser(): Promise<AuthenticatedUser | null> {
  try {
    const res = await authFetch(`${process.env.API_INTERNAL_URL}/auth/me`, {
      cache: "no-store",
    });
    if (!res.ok) return null;
    const data: AuthenticatedUser | null = await res.json();
    return data;
  } catch {
    return null;
  }
}
