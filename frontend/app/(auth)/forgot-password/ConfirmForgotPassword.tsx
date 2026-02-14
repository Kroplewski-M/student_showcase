import EmailIcon from "@/app/SVGS/EmailIcon";
import { motion } from "framer-motion";
import Link from "next/link";

export default function ConfirmForgotPassword() {
  return (
    <motion.div
      key="submitted"
      initial={{ opacity: 0, y: 20, scale: 0.97 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: -20 }}
      transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
      className="relative z-10 w-full max-w-md rounded-2xl border border-third/40 bg-third/20 p-8 text-center backdrop-blur-sm"
    >
      <div className="mx-auto mb-5 flex h-16 w-16 items-center justify-center rounded-full bg-secondary/15">
        <EmailIcon />
      </div>

      <h1 className="mb-2 text-2xl font-extrabold tracking-tight text-light">
        Check your inbox
      </h1>
      <p className="mb-7 text-sm text-support">
        If an account with that Student ID exists, we&rsquo;ve sent an email
        with a link to reset your password.
      </p>

      <Link
        href="/login"
        className="block w-full rounded-xl bg-secondary py-3.5 text-center text-sm font-bold text-primary transition-all hover:bg-secondary/85 hover:shadow-lg hover:shadow-secondary/20 active:scale-[0.985]"
      >
        Back to Login
      </Link>
    </motion.div>
  );
}
