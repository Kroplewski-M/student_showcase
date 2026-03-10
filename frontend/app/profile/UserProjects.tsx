"use client";
import type { Project } from "./page";
import ProjectCard from "./ProjectCard";

const MAX_PROJECTS = 5;

interface UserProjectProps {
  projects: Project[];
  canEdit: boolean;
}

export default function UserProjects({ projects, canEdit }: UserProjectProps) {
  return (
    <>
      <div className="flex flex-wrap items-start justify-between">
        <h1 className="mb-5 text-2xl font-bold text-white">
          Projects{" "}
          <span className="text-secondary/50 text-sm">
            ({projects.length}/{MAX_PROJECTS})
          </span>
        </h1>
        {canEdit && projects.length < MAX_PROJECTS && (
          <button className="flex cursor-pointer items-center gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary">
            Add Project
          </button>
        )}
      </div>
      {projects.length > 0 ? (
        <div className="flex flex-col gap-4">
          {projects.map((project) => (
            <ProjectCard key={project.id} project={project} canEdit={canEdit} />
          ))}
        </div>
      ) : (
        <p className="text-sm italic text-secondary/40">
          No projects added yet
        </p>
      )}
    </>
  );
}
