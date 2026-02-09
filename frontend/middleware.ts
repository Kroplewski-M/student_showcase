import { NextRequest, NextResponse } from "next/server";

const COOKIE_NAME = process.env.COOKIE_NAME;
if (!COOKIE_NAME) {
  throw new Error("COOKIE_NAME env var is not set");
}

export function middleware(req: NextRequest) {
  const COOKIE_NAME: string = process.env.COOKIE_NAME!;
  const hasAuthCookie = Boolean(req.cookies.get(COOKIE_NAME) !== undefined);

  const isProtected = req.nextUrl.pathname.startsWith("/profile");

  if (isProtected && !hasAuthCookie) {
    return NextResponse.redirect(new URL("/signin", req.url));
  }
  if (req.nextUrl.pathname == "/signin" && hasAuthCookie) {
    return NextResponse.redirect(new URL("/profile", req.url));
  }
  return NextResponse.next();
}
