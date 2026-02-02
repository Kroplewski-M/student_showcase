"use client";

import { motion, useScroll, useTransform } from "framer-motion";
import { StretchWord } from "./StretchWord";

export default function Hero() {
  const { scrollY } = useScroll();

  const y = useTransform(scrollY, [0, 500], [0, -500]);
  const scale = useTransform(scrollY, [0, 500], [1, 0.9]);
  const opacity = useTransform(scrollY, [0, 300], [1, 0]);

  return (
    <section className="relative h-screen overflow-hidden text-light flex flex-col">
      <motion.h1
        initial={{ scaleX: 0.7, scaleY: 1.5, opacity: 0 }}
        animate={{ scaleX: 1, scaleY: 1.5, opacity: 1 }}
        transition={{ duration: 1.2, ease: [0.16, 1, 0.3, 1] }}
        style={{ y, scale, opacity }}
        className="
    origin-left
    font-extrabold tracking-tight
    text-[clamp(3rem,20vw,22rem)]
    leading-none
    mt-[10vh]
    text-shadow-lg
  "
      >
        <StretchWord text="STUDENT" origin="origin-bottom" />
      </motion.h1>

      <motion.h1
        initial={{ scaleX: 0.7, scaleY: 1.5, opacity: 0 }}
        animate={{ scaleX: 1, scaleY: 1.5, opacity: 1 }}
        transition={{
          duration: 1.2,
          delay: 0.15,
          ease: [0.16, 1, 0.3, 1],
        }}
        style={{ y, scale, opacity }}
        className="
        origin-right
        text-end
        text-support
        font-extrabold tracking-tight
        text-[clamp(3rem,20vw,22rem)]
        leading-none
        mt-[10vh]
        text-shadow-lg
        "
      >
        <StretchWord text="SHOWCASE" origin="origin-top" />
      </motion.h1>
    </section>
  );
}
