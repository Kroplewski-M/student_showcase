"use client";

import { useState } from "react";
import type { UserProfile } from "./page";
import { motion } from "framer-motion";
import GlassCard from "../components/GlassCard";
import Avatar from "./Avatar";
import ProfileInfo from "./ProfileInfo";
import Edit from "../SVGS/Edit";
import EditProfileForm from "./EditProfileForm";

interface Props {
  profile: UserProfile;
}

export default function ProfileView({ profile }: Props) {
  const [editOpen, setEditOpen] = useState(false);

  return (
    <>
      <div className="min-h-screen overflow-hidden pt-16 font-[Poppins] text-secondary bg-[radial-gradient(ellipse_at_20%_0%,#1a4a4e_0%,#0d2426_40%,#081618_100%)]">
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
          className="z-10 mx-auto max-w-[1000px] px-6 pt-10 pb-20"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{
            duration: 0.8,
            delay: 0.3,
            ease: [0.16, 1, 0.3, 1],
          }}
        >
          <div className="flex flex-wrap items-start justify-between ">
            <h1 className="mb-5 text-2xl font-bold text-white">Profile</h1>
            <button
              type="button"
              onClick={() => setEditOpen(true)}
              className="flex cursor-pointer items-center gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
            >
              <Edit />
              Edit Profile
            </button>
            {editOpen && <EditProfileForm onClose={() => setEditOpen(false)} />}
          </div>
          {/* Profile Card */}
          <GlassCard className="mb-8 animate-[slideUp_0.6s_ease_0.1s_both] p-8">
            <div className="flex flex-col md:flex-row flex-wrap items-start gap-7">
              <Avatar image={profile.profileImageName} />
              <ProfileInfo user={profile} />
            </div>
          </GlassCard>
        </motion.div>
      </div>
    </>
  );
}
