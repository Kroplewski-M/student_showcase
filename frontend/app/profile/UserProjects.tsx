"use client";

export default function UserProjects() {
  return (
    <>
      <div className="flex flex-wrap items-start justify-between ">
        <h1 className="mb-5 text-2xl font-bold text-white">
          Projects <span className="text-secondary/50 text-sm">(0/5)</span>
        </h1>
        <button className="flex cursor-pointer items-center gap-2 rounded-lg border border-secondary/20 bg-secondary/6 px-4 py-2 text-sm font-medium text-secondary/70 transition-all hover:border-secondary/35 hover:bg-secondary/10 hover:text-secondary">
          Add Project
        </button>
      </div>
    </>
  );
}
