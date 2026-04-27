"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faArrowUp,
  faMagnifyingGlass,
} from "@fortawesome/free-solid-svg-icons";
import { motion, useScroll, useTransform } from "framer-motion";
import { useRouter } from "next/navigation";
import { useEffect, useRef, useState } from "react";
import GridBackground from "./GridBackground";

interface SearchStudentsProps {
  query: string | undefined;
  children?: React.ReactNode;
}

export default function SearchStudents({
  query,
  children,
}: SearchStudentsProps) {
  const sectionRef = useRef<HTMLDivElement>(null);
  const router = useRouter();
  const [input, setInput] = useState(query ?? "");
  const [focused, setFocused] = useState(false);

  useEffect(() => {
    const saved = sessionStorage.getItem("studentsScroll");
    if (saved === null) return;
    sessionStorage.removeItem("studentsScroll");
    const y = Number(saved);
    // Retry a few times to win against hydration / framer-motion layout shifts
    const attempts = [0, 100, 300];
    const ids = attempts.map((delay) =>
      setTimeout(() => window.scrollTo({ top: y, behavior: "instant" }), delay),
    );
    return () => ids.forEach(clearTimeout);
  }, []);

  const { scrollYProgress } = useScroll({
    target: sectionRef,
    offset: ["start end", "start 0.5"],
  });

  const rotate = useTransform(scrollYProgress, [0, 1], [-3, 0]);
  function submitSearch() {
    const trimmed = input.trim();
    if (trimmed) {
      router.push(`?query=${encodeURIComponent(trimmed)}#students`);
    } else {
      router.push(`/#students`);
    }
  }
  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    submitSearch();
  }

  return (
    <motion.section
      ref={sectionRef}
      id="students"
      style={{ rotate, transformOrigin: "bottom center" }}
      className="relative z-30 bg-primary shadow-[0_-40px_80px_rgba(0,0,0,0.5)] min-h-screen pb-16"
    >
      <GridBackground />
      <div className="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-secondary/40 to-transparent" />

      {/* Page fold — top-right corner */}
      <div className="pointer-events-none absolute top-0 right-0 z-30">
        {/* Crease shadow */}
        <div className="absolute top-0 right-0 w-28 h-28 bg-gradient-to-bl from-black/45 to-transparent" />
        {/* Folded flap */}
        <div className="w-0 h-0 border-solid border-t-[85px] border-l-[85px] border-b-0 border-r-0 border-t-[#192e30] border-l-transparent [filter:drop-shadow(-2px_3px_6px_rgba(0,0,0,0.6))]" />
        {/* Crease highlight line */}
        <div className="absolute top-0 right-0 w-[120px] h-px bg-gradient-to-l from-secondary/20 to-transparent -rotate-45 origin-top-right" />
      </div>

      {/* Glow orbs */}
      <div className="pointer-events-none absolute inset-0 overflow-hidden">
        <div className="absolute -left-[15%] top-[10%] h-[40vw] w-[40vw] rounded-full bg-secondary/[0.03] blur-3xl" />
        <div className="absolute -right-[10%] bottom-[10%] h-[30vw] w-[30vw] rounded-full bg-third/[0.05] blur-3xl" />
      </div>

      <div className="relative z-10 mx-auto w-full max-w-7xl px-4 py-24 sm:px-8 sm:py-32">
        {/* Section header */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.8, ease: [0.16, 1, 0.3, 1] }}
        >
          <span className="inline-flex items-center gap-2 rounded-full border border-secondary/20 bg-secondary/5 px-4 py-1.5 text-[11px] font-semibold uppercase tracking-[0.2em] text-secondary backdrop-blur-sm">
            <span className="h-1.5 w-1.5 animate-pulse rounded-full bg-secondary" />
            Student Directory
          </span>

          <h2 className="mt-6 text-[clamp(2rem,5vw,4.5rem)] font-extrabold leading-[0.9] tracking-tighter text-light">
            Find your
            <br />
            <span className="text-secondary">student</span>
          </h2>

          <p className="mt-6 max-w-2xl text-sm leading-relaxed text-support/60 sm:text-base">
            Search through our talented pool of final-year Computing &amp;
            Engineering students. Filter by name, course, or skill to discover
            the candidate that&rsquo;s right for you.
          </p>
        </motion.div>

        {/* Search bar */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-60px" }}
          transition={{ duration: 0.8, delay: 0.15, ease: [0.16, 1, 0.3, 1] }}
          className="mt-12"
        >
          <form onSubmit={handleSubmit} className="relative max-w-5xl">
            {/* Ambient glow — always on, intensifies on focus */}
            <div
              className="absolute -inset-3 rounded-3xl transition-opacity duration-500"
              style={{
                background:
                  "radial-gradient(ellipse at center, #a1e9f020 0%, transparent 70%)",
                opacity: focused ? 1 : 0.5,
              }}
            />

            {/* Hard glow ring — focus only */}
            <div
              className={`absolute -inset-[2px] rounded-2xl transition-opacity duration-500 ${
                focused ? "opacity-100" : "opacity-0"
              }`}
              style={{
                background:
                  "linear-gradient(135deg, #a1e9f0 0%, #476d70 50%, #a1e9f0 100%)",
                filter: "blur(8px)",
              }}
            />

            {/* Search container */}
            <div
              className={`relative rounded-2xl border-2 bg-[#172e30] backdrop-blur-md transition-all duration-300 ${
                focused
                  ? "border-secondary/60 shadow-[0_0_0_1px_rgba(161,233,240,0.15)]"
                  : "border-third/50 hover:border-third/70"
              }`}
            >
              {/* Top shimmer line */}
              <div className="absolute inset-x-4 top-0 h-px bg-gradient-to-r from-transparent via-secondary/50 to-transparent" />

              <div className="flex items-end gap-3 px-5 py-5">
                {/* Search icon */}
                <FontAwesomeIcon
                  icon={faMagnifyingGlass}
                  className={`mb-0.5 h-5 w-5 shrink-0 transition-colors duration-300 ${
                    focused ? "text-secondary" : "text-support/40"
                  }`}
                />

                {/* Input */}
                <textarea
                  rows={1}
                  value={input}
                  onChange={(e) => {
                    setInput(e.target.value);
                    e.target.style.height = "auto";
                    e.target.style.height = `${e.target.scrollHeight}px`;
                  }}
                  onFocus={() => setFocused(true)}
                  onBlur={() => setFocused(false)}
                  onKeyDown={(e) => {
                    if (e.key === "Enter" && !e.shiftKey) {
                      e.preventDefault();
                      submitSearch();
                    }
                  }}
                  placeholder="Search by name, course, or skill…"
                  className="flex-1 resize-none overflow-hidden bg-transparent text-base text-light placeholder-support/30 outline-none leading-relaxed"
                />

                {/* Submit button */}
                <button
                  type="submit"
                  disabled={!input.trim()}
                  className="mb-0.5 flex h-9 w-9 shrink-0 items-center justify-center rounded-xl bg-secondary/15 text-secondary/70 transition-all duration-200 hover:bg-secondary/25 hover:text-secondary disabled:cursor-not-allowed disabled:opacity-20"
                >
                  <FontAwesomeIcon icon={faArrowUp} className="h-4 w-4" />
                </button>
              </div>

              {/* Hint row */}
              <div className="border-t border-third/10 px-5 py-2.5 flex items-center justify-between">
                <span className="text-[10px] text-support/25 font-medium tracking-wide">
                  Press Enter to search · Shift+Enter for new line
                </span>
                {query && (
                  <button
                    type="button"
                    onClick={() => {
                      setInput("");
                      router.push("/#students");
                    }}
                    className="text-[10px] text-support/30 hover:text-secondary transition-colors"
                  >
                    Clear
                  </button>
                )}
              </div>
            </div>
          </form>

          {/* Active query pill */}
          {query && (
            <motion.div
              initial={{ opacity: 0, y: 8 }}
              animate={{ opacity: 1, y: 0 }}
              className="mt-4 flex justify-center"
            >
              <span className="inline-flex items-center gap-2 rounded-full border border-secondary/20 bg-secondary/5 px-4 py-1.5 text-[11px] text-secondary">
                Showing results for&nbsp;
                <span className="font-semibold">&ldquo;{query}&rdquo;</span>
              </span>
            </motion.div>
          )}
        </motion.div>
      </div>

      <div className="relative z-10 mx-auto w-full max-w-7xl px-4 pb-8 sm:px-8">
        {children}
      </div>

      {/* Bottom corner accent */}
      <div className="pointer-events-none">
        <div className="absolute bottom-8 left-4 h-16 w-16 border-b border-l border-third/15 sm:left-8 sm:h-20 sm:w-20" />
      </div>
    </motion.section>
  );
}
