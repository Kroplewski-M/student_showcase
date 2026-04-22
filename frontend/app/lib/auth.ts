import { cookies } from "next/headers";
import { AuthenticatedUser } from "./dtos";

export async function getUser(): Promise<AuthenticatedUser | null> {
  const cookieStore = await cookies();
  const cookieName = process.env.COOKIE_NAME;
  if (cookieName === undefined) return null;
  const sessionCookie = cookieStore.get(cookieName);

  if (!sessionCookie) return null;

  try {
    const res = await fetch(`${process.env.API_INTERNAL_URL}/auth/me`, {
      headers: {
        Cookie: `${cookieName}=${sessionCookie.value}`,
      },
      cache: "no-store",
    });
    if (!res.ok) return null;
    const data: AuthenticatedUser | null = await res.json();
    return data;
  } catch {
    return null;
  }
}
