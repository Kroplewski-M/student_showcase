"use client";

import { useState, useRef, useEffect } from "react";
import Image from "next/image";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faArrowUpRightFromSquare,
  faChevronLeft,
  faChevronRight,
  faEllipsisVertical,
} from "@fortawesome/free-solid-svg-icons";
import GlassCard from "../components/GlassCard";
import { getLinkIcon } from "../components/LinkIcon";
import { getProjectImgUrl, isSafeLink } from "../lib/helpers";
import UpsertProjectModal from "./UpsertProjectModal";
import type { Project } from "./page";
import { useRouter } from "next/navigation";
import ConfirmModal from "../components/ConfirmModal";

interface Props {
  project: Project;
  canEdit: boolean;
}

export default function ProjectCard({ project, canEdit }: Props) {
  const [editOpen, setEditOpen] = useState(false);
  const [menuOpen, setMenuOpen] = useState(false);
  const [slideIndex, setSlideIndex] = useState(0);
  const menuRef = useRef<HTMLDivElement>(null);

  const router = useRouter();
  const [deleteProjectConfirm, setDeleteProjectConfirm] = useState(false);
  const [deleteLoading, setDeleteLoading] = useState(false);
  const [deleteError, setDeleteError] = useState<string | null>(null);
  async function deleteProject() {
    if (!canEdit) return;
    setDeleteLoading(true);
    setDeleteError(null);
    try {
      const res = await fetch(`/api/user/delete_project/${project.id}`, {
        method: "delete",
        cache: "no-store",
      });
      if (res.ok) {
        router.refresh();
      } else {
        setDeleteError("Failed to delete project. Please try again.");
      }
    } catch (e) {
      console.log(e);
      setDeleteError("Something went wrong. Please try again.");
    } finally {
      setDeleteLoading(false);
    }
  }
  useEffect(() => {
    if (!menuOpen) return;
    function handleClick(e: MouseEvent) {
      if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
        setMenuOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, [menuOpen]);

  const images = project.images;
  const hasImages = images.length > 0;
  const hasMultiple = images.length > 1;
  const safeLinks = project.links.filter((l) => isSafeLink(l.url));
  function prev() {
    setSlideIndex((i) => (i - 1 + images.length) % images.length);
  }
  function next() {
    setSlideIndex((i) => (i + 1) % images.length);
  }

  return (
    <>
      <GlassCard
        className={`flex flex-col gap-5 p-6 ${menuOpen ? "z-10" : ""}`}
      >
        {/* Slideshow */}
        {hasImages && (
          <div className="relative -mx-6 -mt-6 rounded-t-2xl overflow-hidden">
            <div className="relative h-48 w-full bg-secondary/5">
              <Image
                src={getProjectImgUrl(images[slideIndex].fileName)}
                alt={project.name}
                fill
                className="object-cover"
                unoptimized
              />
            </div>

            {hasMultiple && (
              <>
                <button
                  type="button"
                  onClick={prev}
                  aria-label="Previous image"
                  className="absolute left-2 top-1/2 -translate-y-1/2 flex h-7 w-7 items-center justify-center rounded-full bg-black/40 text-white/80 hover:bg-black/60 transition-colors"
                >
                  <FontAwesomeIcon icon={faChevronLeft} className="text-xs" />
                </button>
                <button
                  type="button"
                  onClick={next}
                  aria-label="Next image"
                  className="absolute right-2 top-1/2 -translate-y-1/2 flex h-7 w-7 items-center justify-center rounded-full bg-black/40 text-white/80 hover:bg-black/60 transition-colors"
                >
                  <FontAwesomeIcon icon={faChevronRight} className="text-xs" />
                </button>
                <div className="absolute bottom-2 left-1/2 -translate-x-1/2 flex gap-1.5">
                  {images.map((_, i) => (
                    <button
                      key={i}
                      type="button"
                      onClick={() => setSlideIndex(i)}
                      aria-label={`Go to image ${i + 1}`}
                      className={`h-1.5 rounded-full transition-all ${
                        i === slideIndex
                          ? "w-4 bg-white"
                          : "w-1.5 bg-white/40 hover:bg-white/70"
                      }`}
                    />
                  ))}
                </div>
              </>
            )}
          </div>
        )}

        {/* Header */}
        <div className="flex items-start justify-between gap-4">
          <div className="min-w-0">
            <h3 className="text-base font-semibold text-white truncate">
              {project.name}
            </h3>
            {project.description && (
              <p className="mt-1 text-sm leading-relaxed text-secondary/60 line-clamp-2">
                {project.description}
              </p>
            )}
          </div>
          <div className="flex shrink-0 items-center gap-2">
            {project.liveLink && isSafeLink(project.liveLink) && (
              <a
                href={project.liveLink}
                target="_blank"
                rel="noopener noreferrer"
                title="Live preview"
                className="inline-flex items-center gap-1.5 rounded-lg border border-secondary/20 bg-secondary/5 px-3 py-1.5 text-xs font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
              >
                <FontAwesomeIcon
                  icon={faArrowUpRightFromSquare}
                  className="text-xs"
                />
                Live
              </a>
            )}
            {canEdit && (
              <div className="relative" ref={menuRef}>
                <button
                  type="button"
                  onClick={() => setMenuOpen((o) => !o)}
                  aria-label="Project options"
                  aria-haspopup="menu"
                  aria-expanded={menuOpen}
                  className="flex cursor-pointer items-center justify-center h-8 w-8 rounded-lg border border-secondary/20 bg-secondary/5 text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
                >
                  <FontAwesomeIcon icon={faEllipsisVertical} />
                </button>

                {menuOpen && (
                  <div className="absolute right-0 top-full mt-1 z-50 min-w-[120px] rounded-lg border border-secondary/15 bg-primary/80 backdrop-blur-md py-1 shadow-lg">
                    <button
                      type="button"
                      onClick={() => {
                        setMenuOpen(false);
                        setEditOpen(true);
                      }}
                      className="w-full text-left px-4 py-2 text-xs text-secondary/70 hover:bg-secondary/10 hover:text-secondary transition-colors cursor-pointer"
                    >
                      Edit
                    </button>
                    <button
                      type="button"
                      onClick={() => {
                        setMenuOpen(false);
                        setDeleteProjectConfirm(true);
                      }}
                      className="w-full text-left px-4 py-2 text-xs text-red-400/70 hover:bg-secondary/10 hover:text-red-400 transition-colors cursor-pointer"
                    >
                      Delete
                    </button>
                  </div>
                )}

                {editOpen && (
                  <UpsertProjectModal
                    project={project}
                    onClose={() => setEditOpen(false)}
                  />
                )}
              </div>
            )}
          </div>
        </div>
        {/* Tools */}
        {project.tools.length > 0 && (
          <div>
            <p className="mb-2 text-xs font-semibold uppercase tracking-wider text-secondary/50">
              Tools
            </p>
            <div className="flex flex-wrap gap-2">
              {project.tools.map((tool, i) => (
                <span
                  key={i}
                  className="inline-flex items-center rounded-full border border-secondary/15 bg-secondary/5 px-3 py-1 text-xs font-medium text-secondary/80 transition-colors hover:border-secondary/30 hover:bg-secondary/10"
                >
                  {tool}
                </span>
              ))}
            </div>
          </div>
        )}

        {/* Links */}
        {safeLinks.length > 0 && (
          <div>
            <p className="mb-2 text-xs font-semibold uppercase tracking-wider text-secondary/50">
              Links
            </p>
            <div className="flex flex-wrap gap-2">
              {safeLinks.map((link, i) => (
                <a
                  key={i}
                  href={link.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  title={link.linkType}
                  aria-label={`Open ${link.linkType} link`}
                  className="inline-flex items-center gap-1.5 h-9 px-4 rounded-lg border border-secondary/15 bg-secondary/5 text-secondary/60 text-xs transition-all hover:border-secondary/30 hover:bg-secondary/10 hover:text-secondary"
                >
                  <FontAwesomeIcon icon={getLinkIcon(link.linkType)} />
                  {link.name ?? link.linkType}
                </a>
              ))}
            </div>
          </div>
        )}
      </GlassCard>
      {canEdit && deleteProjectConfirm && (
        <ConfirmModal
          title={`Delete Project ${project.name}`}
          description="Are you sure you want to delete this project?"
          confirmButtonClass="bg-danger text-light"
          confirmButtonText="delete"
          confirmFunction={deleteProject}
          onClose={() => { setDeleteProjectConfirm(false); setDeleteError(null); }}
          disableConfirm={deleteLoading}
          error={deleteError}
        />
      )}
    </>
  );
}
