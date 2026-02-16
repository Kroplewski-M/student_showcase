"use client";

import { motion } from "framer-motion";
import Link from "next/link";
import HeroVisuals from "./ParticleNetwork";

export default function Hero() {
  return (
    <section className="relative flex min-h-screen items-start pt-32 overflow-hidden px-4 sm:px-8">
      {/* ── Atmospheric layers ── */}

      {/* Grid overlay */}
      <motion.div className="pointer-events-none absolute inset-0 opacity-[0.1]">
        <div
          className="h-full w-full"
          style={{
            backgroundImage:
              "linear-gradient(rgba(161,233,240,0.4) 1px, transparent 1px), linear-gradient(90deg, rgba(161,233,240,0.4) 1px, transparent 1px)",
            backgroundSize: "80px 80px",
          }}
        />
      </motion.div>

      {/* Primary glow orb */}
      <motion.div
        className="pointer-events-none absolute -top-[20%] left-[15%] h-[50vw] w-[50vw] rounded-full blur-3xl"
        initial={{ opacity: 0, scale: 0.8 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 2, ease: "easeOut" }}
      >
        <div className="h-full w-full rounded-full bg-secondary/8" />
      </motion.div>

      {/* Secondary glow orb */}
      <motion.div
        className="pointer-events-none absolute -bottom-[10%] right-[5%] h-[35vw] w-[35vw] rounded-full blur-3xl"
        initial={{ opacity: 0, scale: 0.8 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ duration: 2, delay: 0.3, ease: "easeOut" }}
      >
        <div className="h-full w-full rounded-full bg-third/20" />
      </motion.div>

      {/* ── Content ── */}
      <div className="relative z-10 mx-auto w-full max-w-7xl">
        <div className="grid gap-8 lg:grid-cols-[1fr_auto] lg:items-end lg:gap-16">
          {/* Left — Title block */}
          <div>
            {/* Event badge */}
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{
                duration: 0.8,
                delay: 0.3,
                ease: [0.16, 1, 0.3, 1],
              }}
              className="mb-6 sm:mb-8"
            >
              <span className="inline-flex items-center gap-2 rounded-full border border-secondary/20 bg-secondary/5 px-4 py-1.5 text-[11px] font-semibold uppercase tracking-[0.2em] text-secondary backdrop-blur-sm">
                <span className="h-1.5 w-1.5 animate-pulse rounded-full bg-secondary" />
                Student Showcase Event
              </span>
            </motion.div>

            {/* Main title */}
            <motion.div className="relative">
              <motion.h1
                initial={{ opacity: 0, y: 40 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{
                  duration: 1,
                  delay: 0.1,
                  ease: [0.16, 1, 0.3, 1],
                }}
                className="font-extrabold leading-[0.85] tracking-tighter text-light text-[clamp(3.5rem,12vw,11rem)]"
              >
                C&amp;E
              </motion.h1>
              <motion.h1
                initial={{ opacity: 0, y: 40 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{
                  duration: 1,
                  delay: 0.25,
                  ease: [0.16, 1, 0.3, 1],
                }}
                className="font-extrabold leading-[0.85] tracking-tighter text-light text-[clamp(3.5rem,12vw,11rem)]"
              >
                Futures
                <span className="text-secondary">&nbsp;&rsquo;</span>
                <span>26</span>
              </motion.h1>

              {/* Decorative glow behind title */}
              <div className="pointer-events-none absolute -left-[10%] top-1/2 h-[120%] w-[60%] -translate-y-1/2 rounded-full bg-secondary/5 blur-3xl" />
            </motion.div>

            {/* Tagline */}
            <motion.p
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{
                duration: 0.8,
                delay: 0.6,
                ease: [0.16, 1, 0.3, 1],
              }}
              className="mt-6 max-w-lg text-sm leading-relaxed text-support/70 sm:mt-8 sm:text-base"
            >
              Discover the next generation of talent. Explore student projects,
              connect with future professionals, and find your next hire.
            </motion.p>
          </div>

          {/* Right — CTA + stats */}
          <motion.div
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{
              duration: 0.8,
              delay: 0.8,
              ease: [0.16, 1, 0.3, 1],
            }}
            className="flex flex-col gap-6 lg:items-end"
          >
            {/* Stats row */}
            <div className="flex gap-8">
              <div>
                <p className="text-2xl font-extrabold tracking-tight text-light sm:text-3xl">
                  &mdash;
                </p>
                <p className="mt-0.5 text-[10px] font-semibold uppercase tracking-[0.15em] text-support/40">
                  Students
                </p>
              </div>
              <div className="w-px bg-third/30" />
              <div>
                <p className="text-2xl font-extrabold tracking-tight text-light sm:text-3xl">
                  &mdash;
                </p>
                <p className="mt-0.5 text-[10px] font-semibold uppercase tracking-[0.15em] text-support/40">
                  Projects
                </p>
              </div>
              <div className="w-px bg-third/30" />
              <div>
                <p className="text-2xl font-extrabold tracking-tight text-light sm:text-3xl">
                  2026
                </p>
                <p className="mt-0.5 text-[10px] font-semibold uppercase tracking-[0.15em] text-support/40">
                  Cohort
                </p>
              </div>
            </div>

            {/* CTA buttons */}
            <div className="flex flex-col gap-3 sm:flex-row lg:flex-col">
              <Link
                href="/register"
                className="rounded-xl bg-secondary px-8 py-3.5 text-center text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/25 active:scale-[0.985]"
              >
                Showcase your work
              </Link>
              <Link
                href="#students"
                className="rounded-xl border border-third/40 bg-third/10 px-8 py-3.5 text-center text-sm font-semibold text-light backdrop-blur-sm transition-all hover:border-third/60 hover:bg-third/20 active:scale-[0.985]"
              >
                Browse students
              </Link>
            </div>
          </motion.div>
        </div>
        <div className="pt-16">
          <HeroVisuals />
        </div>
      </div>

      {/* ── Corner accents ── */}
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: 1.2, duration: 1 }}
        className="pointer-events-none"
      >
        <div className="absolute right-4 top-4 h-16 w-16 border-r border-t border-third/20 sm:right-8 sm:top-8 sm:h-20 sm:w-20" />
        <div className="absolute bottom-4 left-4 h-16 w-16 border-b border-l border-third/20 sm:bottom-8 sm:left-8 sm:h-20 sm:w-20" />
      </motion.div>
    </section>
  );
}
