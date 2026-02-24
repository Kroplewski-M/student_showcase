"use client";

import type { UserProfile } from "./page";
import { motion } from "framer-motion";
import GlassCard from "../components/GlassCard";
import Avatar from "./Avatar";

interface Props {
  profile: UserProfile;
}

export default function ProfileView({ profile }: Props) {
  return (
    <>
      <div className=" min-h-screen overflow-hidden pt-16 font-[Poppins] text-secondary bg-[radial-gradient(ellipse_at_20%_0%,#1a4a4e_0%,#0d2426_40%,#081618_100%)]">
        {/* Glow orbs */}
        <motion.div
          className="pointer-events-none absolute -bottom-[10%] right-[-10%] h-[35vw] w-[35vw] rounded-full blur-3xl"
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 2, delay: 0.3, ease: "easeOut" }}
        >
          <div className="h-full w-full rounded-full bg-third/20" />
        </motion.div>
        <motion.div
          className="pointer-events-none absolute -bottom-[15%] left-[-10%] h-[35vw] w-[35vw] rounded-full blur-3xl"
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 2, delay: 0.3, ease: "easeOut" }}
        >
          <div className="h-full w-full rounded-full bg-third/20" />
        </motion.div>
        {/* Content */}
        <motion.div
          className="z-10 mx-auto max-w-[960px] px-6 pt-10 pb-20"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{
            duration: 0.8,
            delay: 0.3,
            ease: [0.16, 1, 0.3, 1],
          }}
        >
          <h1 className="mb-5 text-2xl font-bold text-white">Profile</h1>
          {/* Profile Card */}
          <GlassCard className="mb-8 animate-[slideUp_0.6s_ease_0.1s_both] p-8">
            <div className="flex flex-wrap items-start gap-7">
              <Avatar image={profile.profile_image_name} />
              {/* Info */}
              <div className="min-w-[200px] flex-1">
                <div className="mb-4 flex flex-wrap items-start justify-between gap-3">
                  <div>
                    <h2 className="mb-0.5 text-2xl font-bold text-white">
                      U{profile.id}
                    </h2>
                  </div>
                </div>
                {/* Placeholder */}
                <div className="rounded-[10px] border border-dashed border-secondary/10 bg-primary/40 px-5 py-4 text-[13px] text-secondary/35">
                  Profile details (name, interests, links) will appear here once
                  available.
                </div>
              </div>
            </div>
          </GlassCard>
        </motion.div>
      </div>
    </>
  );
}
