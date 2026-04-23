"use client";

import { useState } from "react";
import type { UserProfile } from "./page";
import { motion } from "framer-motion";
import GlassCard from "../components/GlassCard";
import Avatar from "./Avatar";
import ProfileInfo from "./ProfileInfo";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faDownload, faPenToSquare, faShield } from "@fortawesome/free-solid-svg-icons";
import Link from "next/link";
import EditProfileForm from "./EditProfileForm";
import UserProjects from "./UserProjects";
import { getCvUrl } from "../lib/helpers";
import UpdateCVForm from "./UpdateCVForm";
import { useAuth } from "../context/auth-context";

interface Props {
  profile: UserProfile;
  canEdit?: boolean;
}

export default function ProfileView({ profile, canEdit = false }: Props) {
  const [editOpen, setEditOpen] = useState(false);
  const [cvFormOpen, setCvFormOpen] = useState(false);
  const { user } = useAuth();
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
          <div className="flex flex-wrap items-start justify-between">
            <h1 className="mb-5 text-2xl font-bold text-white">Profile</h1>
            <div className="flex items-center gap-2">
            {user?.is_admin && (
              <Link
                href="/admin"
                className="flex cursor-pointer items-center gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
              >
                <FontAwesomeIcon icon={faShield} className="w-[13px] h-[13px]" />
                Admin
              </Link>
            )}
            {canEdit && (
              <>
                <button
                  type="button"
                  onClick={() => setEditOpen(true)}
                  className="flex cursor-pointer items-center gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
                >
                  <FontAwesomeIcon
                    icon={faPenToSquare}
                    className="w-[13px] h-[13px]"
                  />
                  Edit Profile
                </button>
                {editOpen && (
                  <EditProfileForm onClose={() => setEditOpen(false)} />
                )}
              </>
            )}
            </div>
          </div>
          {/* Profile Card */}
          <GlassCard className="mb-8 animate-[slideUp_0.6s_ease_0.1s_both] p-8">
            <div className="flex flex-col md:flex-row flex-wrap items-start gap-7">
              <div className="flex flex-col">
                <Avatar image={profile.profileImageName} canEdit={canEdit} />
                {profile.profileCvName != null && (
                  <a
                    href={getCvUrl(profile.profileCvName)}
                    download
                    className="mt-3 flex cursor-pointer items-center justify-center gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
                  >
                    <FontAwesomeIcon icon={faDownload} /> CV
                  </a>
                )}
                {canEdit && (
                  <button
                    type="button"
                    onClick={() => setCvFormOpen(true)}
                    className="mt-2 flex cursor-pointer items-center justify-center gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
                  >
                    {profile.profileCvName != null ? "Update CV" : "Upload CV"}
                  </button>
                )}
                {cvFormOpen && canEdit && (
                  <UpdateCVForm onClose={() => setCvFormOpen(false)} />
                )}
              </div>
              <ProfileInfo user={profile} />
            </div>
          </GlassCard>
          <UserProjects
            projects={profile.projects}
            canEdit={canEdit}
            featuredProjectId={profile.featuredProjectId}
          />
        </motion.div>
      </div>
    </>
  );
}
