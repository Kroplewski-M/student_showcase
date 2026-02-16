"use client";

import { createContext, ReactNode, useContext } from "react";
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
