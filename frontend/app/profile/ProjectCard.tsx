"use client";

import { useState } from "react";
import Image from "next/image";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faArrowUpRightFromSquare } from "@fortawesome/free-solid-svg-icons";
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
  const featuredImage = project.featuredImgId
    ? project.images.find(([id]) => id === project.featuredImgId)
    : project.images[0];

  return (
    <GlassCard className="flex flex-col gap-5 p-6">
      {/* Header */}
      <div className="flex items-start justify-between gap-4">
        <div className="flex items-start gap-4 min-w-0">
          {featuredImage && (
            <div className="shrink-0 h-14 w-14 overflow-hidden rounded-xl border border-secondary/15 bg-secondary/5">
              <Image
                src={getProjectImgUrl(featuredImage[1])}
                alt={project.name}
                width={56}
                height={56}
                className="h-full w-full object-cover"
                unoptimized
              />
            </div>
          )}
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
      )}

      {/* Extra images (excluding featured) */}
      {project.images.length > 1 && (
        <div className="flex flex-wrap gap-2">
          {project.images
            .filter(([id]) => id !== featuredImage?.[0])
            .map(([id, name]) => (
              <div
                key={id}
                className="h-16 w-16 overflow-hidden rounded-lg border border-secondary/15 bg-secondary/5"
              >
                <Image
                  src={getProjectImgUrl(name)}
                  alt={project.name}
                  width={64}
                  height={64}
                  className="h-full w-full object-cover"
                  unoptimized
                />
              </div>
            ))}
        </div>
      )}
    </GlassCard>
  );
}
