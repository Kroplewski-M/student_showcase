import { NextRequest, NextResponse } from "next/server";
import { AuthenticatedUser } from "./app/lib/dtos";

const COOKIE_NAME = process.env.COOKIE_NAME;
if (!COOKIE_NAME) {
  throw new Error("COOKIE_NAME environment variable is not set");
}
const PROTECTED_ROUTES = ["/profile"];
const ADMIN_ROUTES = ["/admin"];
const AUTH_ROUTES = ["/login", "/register"];

export async function proxy(req: NextRequest) {
  const token = req.cookies.get(COOKIE_NAME!)?.value;
  const { pathname } = req.nextUrl;

  const isProtected = PROTECTED_ROUTES.some((r) => pathname.startsWith(r));
  const isAdminRoute = ADMIN_ROUTES.some((r) => pathname.startsWith(r));
  const isAuthRoute = AUTH_ROUTES.includes(pathname);

  if (!token) {
    if (isProtected || isAdminRoute) {
      return NextResponse.redirect(new URL("/login", req.url));
    }
    return NextResponse.next();
  }

  if (isProtected || isAdminRoute) {
    const res = await fetch(`${process.env.API_INTERNAL_URL}/auth/me`, {
      headers: { Cookie: `${COOKIE_NAME}=${token}` },
    });
    if (!res.ok) {
      const response = NextResponse.redirect(new URL("/login", req.url));
      response.cookies.delete(COOKIE_NAME!);
      return response;
    }
    if (isAdminRoute) {
      const authenticatedUser: AuthenticatedUser | null = await res.json();
      if (authenticatedUser?.is_admin != true) {
        return NextResponse.redirect(new URL("/", req.url));
      }
    }
    const next = NextResponse.next();
    const setCookie = res.headers.getSetCookie?.();
    if (setCookie) {
      for (const cookie of setCookie) {
        next.headers.append("Set-Cookie", cookie);
      }
    }
    return next;
  }

  if (isAuthRoute) {
    return NextResponse.redirect(new URL("/profile", req.url));
  }

  return NextResponse.next();
}

export const config = {
  matcher: ["/profile/:path*", "/login", "/register", "/admin/:path*"],
};
