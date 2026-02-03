"use client";
import { motion } from "framer-motion";

type StretchWordProps = {
  text: string;
  origin?: string;
};

export const StretchWord = ({
  text,
  origin = "origin-bottom",
}: StretchWordProps) => (
  <span className="inline-flex">
    {text.split("").map((char: string, i: number) => (
      <motion.span
        key={i}
        whileHover={{ scaleY: 1.25 }}
        transition={{ type: "spring", stiffness: 300, damping: 18 }}
        className={`inline-block ${origin}`}
      >
        {char === " " ? "\u00A0" : char}
      </motion.span>
    ))}
  </span>
);
