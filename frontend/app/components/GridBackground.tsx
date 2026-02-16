"use client";
import { motion } from "framer-motion";

export default function GridBackground() {
  return (
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
  );
}
