import { cookies } from "next/headers";

export interface User {
  id: string;
}

export async function getUser(): Promise<User | null> {
  const cookieStore = await cookies();
  const cookieName = process.env.COOKIE_NAME;
  if (cookieName === undefined) return null;
  const sessionCookie = cookieStore.get(cookieName);

  if (!sessionCookie) return null;

  try {
    const res = await fetch("/api/auth/me", {
      credentials: "include",
      cache: "no-store",
    });
    if (!res.ok) return null;
    const data: { userId: string } = await res.json();

    return {
      id: data.userId,
    };
  } catch {
    return null;
  }
}
