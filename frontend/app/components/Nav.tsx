"use client";
import { motion } from "framer-motion";
import Link from "next/link";

export default function Nav() {
  return (
    <div className="sticky top-5 z-50">
      <div className="flex">
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          transition={{
            duration: 1.2,
            delay: 1,
            ease: [0.16, 1, 0.3, 1],
          }}
          className="bg-support  max-w-4xl ml-auto rounded-full py-2 px-5 mr-2"
        >
          <Link href={"/login"} className="font-semibold text-dark">
            Student Login
          </Link>
        </motion.div>
      </div>
    </div>
  );
}
