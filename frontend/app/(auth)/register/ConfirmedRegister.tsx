import CheckIcon from "@/app/SVGS/CheckIcon";
import EmailIcon from "@/app/SVGS/EmailIcon";
import { motion } from "framer-motion";

export default function ConfirmedRegister() {
  return (
    <motion.div
      key="success"
      initial={{ opacity: 0, y: 20, scale: 0.97 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: -20 }}
      transition={{ duration: 0.5, ease: [0.16, 1, 0.3, 1] }}
      className="relative z-10 w-full max-w-md rounded-2xl border border-third/40 bg-third/20 backdrop-blur-sm p-8 text-center"
    >
      {/* Check icon */}
      <div className="mx-auto mb-5 flex h-16 w-16 items-center justify-center rounded-full bg-secondary/15">
        <CheckIcon />
      </div>

      <h1 className="text-2xl font-extrabold tracking-tight text-light mb-2">
        You&rsquo;re all set
      </h1>
      <p className="text-support text-sm mb-7">
        Your account has been created successfully.
      </p>

      {/* Email notice */}
      <div className="flex items-start gap-4 rounded-xl border border-third/40 bg-primary/40 p-5 text-left mb-5">
        <EmailIcon />
        <div>
          <p className="text-sm font-semibold text-light mb-1">
            Check your inbox
          </p>
          <p className="text-xs leading-relaxed text-support">
            We&rsquo;ve sent a verification link to your student email. Click
            the link to activate your account.
          </p>
        </div>
      </div>

      <p className="text-xs text-support/60 mb-6">
        Didn&rsquo;t receive the email? Check your spam folder
      </p>
    </motion.div>
  );
}
