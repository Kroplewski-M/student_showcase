"use client";
import { motion } from "framer-motion";

export default function Nav() {
  return (
    <div className="sticky top-5 z-50">
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{
          duration: 1.2,
          delay: 1,
          ease: [0.16, 1, 0.3, 1],
        }}
        className="h-12.5 bg-support flex items-center justify-center max-w-4xl mx-auto rounded-full"
      >
        <p className="text-center text-dark font-bold">
          Student Showcase 2026 – @{" "}
          <a
            href="https://www.hud.ac.uk/"
            target="_blank"
            rel="noopener noreferrer"
            className="underline cursor-pointer-default"
          >
            University Of Huddersfield
          </a>
        </p>
      </motion.div>
    </div>
  );
}
