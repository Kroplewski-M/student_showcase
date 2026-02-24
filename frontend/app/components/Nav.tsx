"use client";

import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import Link from "next/link";
import LogoWritten from "./LogoWritten";
import { useAuth } from "../context/auth-context";
import { usePathname } from "next/navigation";
import Logout from "./Logout";

export default function Nav() {
  const [menuOpen, setMenuOpen] = useState(false);
  const { isAuthenticated } = useAuth();
  const pathname = usePathname();
  return (
    <nav className="fixed top-0 z-50 w-screen backdrop-blur">
      <div className="mx-auto flex items-center justify-between px-4 py-4 sm:px-8">
        {/* Logo */}
        <motion.div
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ duration: 0.8, delay: 0.8, ease: [0.16, 1, 0.3, 1] }}
        >
          <Link
            href="/"
            className="text-lg font-extrabold tracking-tight text-light sm:text-xl"
          >
            <LogoWritten />
          </Link>
        </motion.div>

        {/* Desktop links */}
        <motion.div
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ duration: 0.8, delay: 1, ease: [0.16, 1, 0.3, 1] }}
          className="hidden items-center gap-2 sm:flex"
        >
          {isAuthenticated === false ? (
            <>
              <Link
                href="/login"
                className="rounded-xl px-5 py-2 text-sm font-semibold text-support transition-colors hover:text-light"
              >
                Login
              </Link>
              <Link
                href="/register"
                className="rounded-xl bg-secondary px-5 py-2 text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985]"
              >
                Register
              </Link>
            </>
          ) : (
            <>
              {pathname === "/profile" ? (
                <Logout />
              ) : (
                <Link
                  href="/profile"
                  className="rounded-xl bg-secondary px-5 py-2 text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985]"
                >
                  Profile
                </Link>
              )}
            </>
          )}
        </motion.div>

        {/* Mobile hamburger */}
        <motion.button
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{ duration: 0.8, delay: 1, ease: [0.16, 1, 0.3, 1] }}
          onClick={() => setMenuOpen((v) => !v)}
          className="relative z-50 flex h-10 w-10 items-center justify-center sm:hidden"
          aria-label={menuOpen ? "Close menu" : "Open menu"}
        >
          <div className="flex w-5 flex-col items-end gap-[5px]">
            <span
              className={`block h-[2px] rounded-full bg-light transition-all duration-300 ${
                menuOpen ? "w-5 translate-y-[7px] rotate-45" : "w-5"
              }`}
            />
            <span
              className={`block h-[2px] rounded-full bg-light transition-all duration-300 ${
                menuOpen ? "w-0 opacity-0" : "w-3.5"
              }`}
            />
            <span
              className={`block h-[2px] rounded-full bg-light transition-all duration-300 ${
                menuOpen ? "w-5 -translate-y-[7px] -rotate-45" : "w-5"
              }`}
            />
          </div>
        </motion.button>
      </div>

      {/* Mobile menu overlay */}
      <AnimatePresence>
        {menuOpen && (
          <>
            {/* Backdrop */}
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              transition={{ duration: 0.25 }}
              className="fixed inset-0 z-40 bg-primary/80 backdrop-blur-sm sm:hidden"
              onClick={() => setMenuOpen(false)}
            />

            {/* Menu panel */}
            <motion.div
              initial={{ opacity: 0, y: -10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -10 }}
              transition={{ duration: 0.3, ease: [0.16, 1, 0.3, 1] }}
              className="fixed inset-x-0 top-0 z-40 border-b border-third/40 bg-primary/95 px-4 pb-8 pt-20 backdrop-blur-md sm:hidden"
            >
              <div className="flex flex-col gap-3">
                {isAuthenticated === false ? (
                  <>
                    <Link
                      href="/login"
                      onClick={() => setMenuOpen(false)}
                      className="rounded-xl border border-third/40 bg-third/20 px-5 py-3.5 text-center text-sm font-semibold text-light transition-colors hover:bg-third/30"
                    >
                      Login
                    </Link>
                    <Link
                      href="/register"
                      onClick={() => setMenuOpen(false)}
                      className="rounded-xl bg-secondary px-5 py-3.5 text-center text-sm font-bold text-primary transition-all hover:bg-secondary/85 active:scale-[0.985]"
                    >
                      Register
                    </Link>
                  </>
                ) : (
                  <>
                    {pathname === "/profile" ? (
                      <Logout />
                    ) : (
                      <Link
                        href="/profile"
                        onClick={() => setMenuOpen(false)}
                        className="rounded-xl bg-secondary px-5 py-3.5 text-center text-sm font-bold text-primary transition-all hover:bg-secondary/85 active:scale-[0.985]"
                      >
                        Profile
                      </Link>
                    )}
                  </>
                )}
              </div>
            </motion.div>
          </>
        )}
      </AnimatePresence>
    </nav>
  );
}
