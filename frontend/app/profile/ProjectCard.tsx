"use client";

import { useState } from "react";
import Image from "next/image";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faArrowUpRightFromSquare,
  faChevronLeft,
  faChevronRight,
} from "@fortawesome/free-solid-svg-icons";
import GlassCard from "../components/GlassCard";
import { getLinkIcon } from "../components/LinkIcon";
import { getProjectImgUrl, isSafeLink } from "../lib/helpers";
import UpsertProjectModal from "./UpsertProjectModal";
import type { Project } from "./page";

interface Props {
  project: Project;
  canEdit: boolean;
}

export default function ProjectCard({ project, canEdit }: Props) {
  const [editOpen, setEditOpen] = useState(false);
  const [slideIndex, setSlideIndex] = useState(0);

  const images = project.images;
  const hasImages = images.length > 0;
  const hasMultiple = images.length > 1;

  function prev() {
    setSlideIndex((i) => (i - 1 + images.length) % images.length);
  }
  function next() {
    setSlideIndex((i) => (i + 1) % images.length);
  }

  return (
    <GlassCard className="flex flex-col gap-5 p-6">
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
                onClick={prev}
                aria-label="Previous image"
                className="absolute left-2 top-1/2 -translate-y-1/2 flex h-7 w-7 items-center justify-center rounded-full bg-black/40 text-white/80 hover:bg-black/60 transition-colors"
              >
                <FontAwesomeIcon icon={faChevronLeft} className="text-xs" />
              </button>
              <button
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
            <>
              <button
                onClick={() => setEditOpen(true)}
                className="flex cursor-pointer items-center gap-1.5 rounded-lg border border-secondary/20 bg-secondary/5 px-3 py-1.5 text-xs font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary"
              >
                Edit
              </button>
              {editOpen && (
                <UpsertProjectModal
                  project={project}
                  onClose={() => setEditOpen(false)}
                />
              )}
            </>
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
      {project.links.length > 0 && (
        <div>
          <p className="mb-2 text-xs font-semibold uppercase tracking-wider text-secondary/50">
            Links
          </p>
          <div className="flex flex-wrap gap-2">
            {project.links
              .filter((l) => isSafeLink(l.url))
              .map((link, i) => (
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
  );
}
