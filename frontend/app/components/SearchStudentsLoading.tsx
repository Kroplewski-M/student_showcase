"use client";

import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faMagnifyingGlass } from "@fortawesome/free-solid-svg-icons";
import { motion, AnimatePresence } from "framer-motion";
import { useEffect, useState } from "react";

const thoughts = [
  "Scanning student profiles…",
  "Matching skills to your query…",
  "Analysing project portfolios…",
  "Identifying top candidates…",
  "Almost there…",
];

export default function SearchStudentsLoading() {
  const [thoughtIndex, setThoughtIndex] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setThoughtIndex((i) => (i + 1) % thoughts.length);
    }, 1800);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="mt-12">
      {/* Thinking indicator */}
      <div className="flex flex-col items-center gap-4 py-8">
        <div className="relative">
          <motion.div
            animate={{ scale: [1, 1.15, 1], opacity: [0.6, 1, 0.6] }}
            transition={{ duration: 2, repeat: Infinity, ease: "easeInOut" }}
            className="flex h-12 w-12 items-center justify-center rounded-2xl border border-secondary/20 bg-secondary/5"
          >
            <FontAwesomeIcon
              icon={faMagnifyingGlass}
              className="h-5 w-5 text-secondary"
            />
          </motion.div>
          {/* Orbiting dot */}
          <motion.div
            animate={{ rotate: 360 }}
            transition={{ duration: 3, repeat: Infinity, ease: "linear" }}
            className="absolute -inset-2"
          >
            <div className="h-2 w-2 rounded-full bg-secondary/60" />
          </motion.div>
        </div>

        {/* Cycling thought text */}
        <div className="h-5 overflow-hidden">
          <AnimatePresence mode="wait">
            <motion.p
              key={thoughtIndex}
              initial={{ opacity: 0, y: 8 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -8 }}
              transition={{ duration: 0.3, ease: "easeOut" }}
              className="text-xs font-medium tracking-wide text-secondary/60"
            >
              {thoughts[thoughtIndex]}
            </motion.p>
          </AnimatePresence>
        </div>

        {/* Dot pulse row */}
        <div className="flex items-center gap-1.5">
          {[0, 1, 2].map((i) => (
            <motion.div
              key={i}
              animate={{ opacity: [0.2, 1, 0.2] }}
              transition={{
                duration: 1.2,
                repeat: Infinity,
                delay: i * 0.2,
                ease: "easeInOut",
              }}
              className="h-1.5 w-1.5 rounded-full bg-secondary/50"
            />
          ))}
        </div>
      </div>

      {/* Skeleton cards */}
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {Array.from({ length: 6 }).map((_, i) => (
          <motion.div
            key={i}
            initial={{ opacity: 0, y: 16 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.4, delay: i * 0.07, ease: "easeOut" }}
            className="relative overflow-hidden rounded-2xl border border-third/20 bg-third/[0.04] p-5"
          >
            {/* Shimmer sweep */}
            <motion.div
              animate={{ x: ["-100%", "200%"] }}
              transition={{
                duration: 1.6,
                repeat: Infinity,
                delay: i * 0.15,
                ease: "easeInOut",
              }}
              className="absolute inset-0 -skew-x-12 bg-gradient-to-r from-transparent via-secondary/5 to-transparent"
            />
            <div className="flex items-center gap-3">
              {/* Avatar skeleton */}
              <div className="h-10 w-10 shrink-0 rounded-full bg-third/30" />
              <div className="flex-1 space-y-2">
                <div className="h-3 w-3/4 rounded-full bg-third/30" />
                <div className="h-2.5 w-1/2 rounded-full bg-third/20" />
              </div>
            </div>
            <div className="mt-4 space-y-2">
              <div className="h-2.5 w-full rounded-full bg-third/20" />
              <div className="h-2.5 w-5/6 rounded-full bg-third/20" />
            </div>
            <div className="mt-4 flex gap-2">
              <div className="h-5 w-14 rounded-full bg-third/20" />
              <div className="h-5 w-16 rounded-full bg-third/20" />
              <div className="h-5 w-12 rounded-full bg-third/20" />
            </div>
          </motion.div>
        ))}
      </div>
    </div>
  );
}
