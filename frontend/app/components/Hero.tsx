"use client";

import { motion, useScroll, useTransform } from "framer-motion";
import { StretchWord } from "./StretchWord";
import ArrowSVG from "../SVGS/Arrow";

export default function Hero() {
  const { scrollY } = useScroll();

  const y = useTransform(scrollY, [0, 500], [0, -500]);
  const scale = useTransform(scrollY, [0, 500], [1, 0.9]);
  const opacity = useTransform(scrollY, [0, 300], [1, 0]);

  return (
    <section className="relative mt-90 text-light">
      <div
        className="
      grid h-full
      grid-rows-[1fr_auto_auto_1fr]
      px-4 sm:px-8
    "
      >
        <motion.h1
          initial={{ scaleX: 0.7, scaleY: 1.4, opacity: 0 }}
          animate={{ scaleX: 1, scaleY: 1.5, opacity: 1 }}
          transition={{ duration: 1.2, ease: [0.16, 1, 0.3, 1] }}
          style={{ y, scale, opacity }}
          className="
        origin-left
        font-extrabold tracking-tight
        text-[clamp(2.5rem,18vw,18rem)]
        sm:text-[clamp(3rem,15vw,20rem)]
        leading-none
        text-shadow-lg
        justify-self-start
      "
        >
          <StretchWord text="STUDENT" origin="origin-bottom" />
        </motion.h1>

        <motion.h1
          initial={{ scaleX: 0.7, scaleY: 1.4, opacity: 0 }}
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
        text-[clamp(2.5rem,18vw,18rem)]
        sm:text-[clamp(3rem,15vw,20rem)]
        leading-none
        text-shadow-lg
        justify-self-end
          mt-20
      "
        >
          <StretchWord text="SHOWCASE" origin="origin-top" />
        </motion.h1>

        <motion.div
          initial={{ opacity: 0, y: -20 }}
          animate={{ opacity: 0.7, y: 0 }}
          transition={{
            duration: 1,
            delay: 0.6,
            ease: [0.16, 1, 0.3, 1],
          }}
          className="
          absolute
          left-1/2
          bottom-10
          rotate-180
          animate-bounce
      "
        >
          <ArrowSVG fill="#CCCCCC" width={120} height={120} />
        </motion.div>
      </div>
    </section>
  );
}
