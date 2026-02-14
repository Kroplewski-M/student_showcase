"use client";

import { motion } from "framer-motion";
import Link from "next/link";

export default function NotFoundPage() {
  return (
    <section className="relative flex min-h-screen items-center justify-center px-4 py-12">
      <div className="pointer-events-none absolute -top-1/3 -left-1/4 h-[80vw] w-[80vw] rounded-full bg-secondary/5 blur-3xl" />

      <motion.div
        initial={{ opacity: 0, y: 20, scale: 0.97 }}
        animate={{ opacity: 1, y: 0, scale: 1 }}
        transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
        className="relative z-10 w-full max-w-md rounded-2xl border border-third/40 bg-third/20 p-8 text-center backdrop-blur-sm"
      >
        <p className="mb-3 text-7xl font-extrabold tracking-tighter text-secondary">
          404
        </p>
        <h1 className="mb-2 text-2xl font-extrabold tracking-tight text-light">
          Page not found
        </h1>
        <p className="mb-8 text-sm text-support">
          The page you&rsquo;re looking for doesn&rsquo;t exist or has been
          moved.
        </p>

        <Link
          href="/"
          className="block w-full rounded-xl bg-secondary py-3.5 text-center text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985]"
        >
          Back to Home
        </Link>
      </motion.div>
    </section>
  );
}
