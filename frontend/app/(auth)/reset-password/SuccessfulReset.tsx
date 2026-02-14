import CheckIcon from "@/app/SVGS/CheckIcon";
import { motion } from "framer-motion";
import Link from "next/link";

export default function SuccessfulReset() {
  return (
    <motion.div
      key="success"
      initial={{ opacity: 0, y: 20, scale: 0.97 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
      className="relative z-10 w-full max-w-md rounded-2xl border border-third/40 bg-third/20 p-8 text-center backdrop-blur-sm"
    >
      <div className="mx-auto mb-5 flex h-16 w-16 items-center justify-center rounded-full bg-secondary/15">
        <CheckIcon />
      </div>

      <h1 className="mb-2 text-2xl font-extrabold tracking-tight text-light">
        Password reset
      </h1>
      <p className="mb-7 text-sm text-support">
        Your password has been updated successfully. You can now login with your
        new password.
      </p>
      <Link
        href="/login"
        className="block w-full rounded-xl bg-secondary py-3.5 text-center text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985]"
      >
        Go to Login
      </Link>
    </motion.div>
  );
}
