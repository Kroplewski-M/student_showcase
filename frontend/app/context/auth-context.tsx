"use client";

import { createContext, ReactNode, useContext, useEffect, useRef } from "react";
import { usePathname, useRouter } from "next/navigation";
import { User } from "../lib/auth";

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthState | undefined>(undefined);

export function AuthProvider({
  initialUser,
  children,
}: {
  initialUser: User | null;
  children: ReactNode;
}) {
  const user: User | null = initialUser;
  const pathname = usePathname();
  const router = useRouter();

  const didMountRef = useRef(false);
  useEffect(() => {
    if (!didMountRef.current) {
      didMountRef.current = true;
      return;
    }
    router.refresh();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [pathname]);
  return (
    <AuthContext.Provider value={{ user, isAuthenticated: user != null }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthState {
  const ctx = useContext(AuthContext);
  if (ctx === undefined) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return ctx;
}
