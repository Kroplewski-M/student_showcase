"use client";

import { useEffect, useState } from "react";
import Image from "next/image";
import Link from "next/link";
import { motion } from "framer-motion";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faChevronLeft,
  faChevronRight,
  faArrowRight,
} from "@fortawesome/free-solid-svg-icons";
import { getProfileImgUrl, getProjectImgUrl } from "../lib/helpers";
import GlassCard from "./GlassCard";
import SearchStudents from "./SearchStudents";
import SearchStudentsLoading from "./SearchStudentsLoading";

interface FeaturedProject {
  name: string;
  description: string;
  tools: string[];
  images: string[];
}

interface StudentCard {
  id: string;
  firstName: string;
  lastName: string;
  profileImage: string | null;
  description: string;
  course: string;
  tools: string[];
  featuredProject: FeaturedProject;
}

interface Props {
  query: string;
}

function ProjectSlideshow({ images }: { images: string[] }) {
  const [index, setIndex] = useState(0);

  if (images.length === 0) return null;

  const prev = (e: React.MouseEvent) => {
    e.preventDefault();
    setIndex((i) => (i - 1 + images.length) % images.length);
  };
  const next = (e: React.MouseEvent) => {
    e.preventDefault();
    setIndex((i) => (i + 1) % images.length);
  };

  return (
    <div className="relative aspect-video w-full overflow-hidden rounded-xl bg-primary/60">
      <Image
        src={getProjectImgUrl(images[index])}
        alt="Project screenshot"
        fill
        className="object-cover transition-opacity duration-300"
        unoptimized
      />
      {images.length > 1 && (
        <>
          <button
            onClick={prev}
            className="absolute left-2 top-1/2 -translate-y-1/2 flex h-7 w-7 items-center justify-center rounded-full bg-primary/70 text-light/80 backdrop-blur-sm transition hover:bg-primary hover:text-secondary"
          >
            <FontAwesomeIcon icon={faChevronLeft} className="h-3 w-3" />
          </button>
          <button
            onClick={next}
            className="absolute right-2 top-1/2 -translate-y-1/2 flex h-7 w-7 items-center justify-center rounded-full bg-primary/70 text-light/80 backdrop-blur-sm transition hover:bg-primary hover:text-secondary"
          >
            <FontAwesomeIcon icon={faChevronRight} className="h-3 w-3" />
          </button>
          <div className="absolute bottom-2 left-1/2 flex -translate-x-1/2 gap-1.5">
            {images.map((_, i) => (
              <button
                key={i}
                onClick={(e) => {
                  e.preventDefault();
                  setIndex(i);
                }}
                className={`h-1.5 rounded-full transition-all ${
                  i === index
                    ? "w-4 bg-secondary"
                    : "w-1.5 bg-light/30 hover:bg-light/60"
                }`}
              />
            ))}
          </div>
        </>
      )}
    </div>
  );
}

function StudentCardItem({ student }: { student: StudentCard }) {
  const initials =
    `${student.firstName[0] ?? ""}${student.lastName[0] ?? ""}`.toUpperCase();
  const imageUrl = student.profileImage
    ? getProfileImgUrl(student.profileImage)
    : null;

  return (
    <Link
      href={`/student/${student.id}`}
      className="block group"
      onClick={() =>
        sessionStorage.setItem("studentsScroll", String(window.scrollY))
      }
    >
      <GlassCard className="flex h-full flex-col overflow-hidden transition-all duration-300 group-hover:border-secondary/25 group-hover:shadow-[0_0_30px_rgba(161,233,240,0.06)]">
        {/* Header — avatar + name + course + tools */}
        <div className="flex items-start gap-4 p-5">
          {/* Avatar */}
          <div className="relative h-14 w-14 shrink-0 overflow-hidden rounded-full border-2 border-secondary/20 bg-[linear-gradient(135deg,var(--color-secondary)/0.2,var(--color-primary)/0.7)] shadow-[0_4px_16px_rgba(0,0,0,0.25)]">
            {imageUrl ? (
              <Image
                src={imageUrl}
                alt={`${student.firstName} ${student.lastName}`}
                fill
                className="object-cover"
                unoptimized
              />
            ) : (
              <span className="flex h-full w-full items-center justify-center text-sm font-bold text-secondary/70">
                {initials}
              </span>
            )}
          </div>

          {/* Name + course */}
          <div className="min-w-0 flex-1">
            <h3 className="truncate text-base font-bold text-light group-hover:text-secondary transition-colors duration-200">
              {student.firstName} {student.lastName}
            </h3>
            <p className="mt-0.5 truncate text-xs text-secondary/50">
              {student.course}
            </p>

            {/* Tools */}
            {student.tools.length > 0 && (
              <div className="mt-2.5 flex flex-wrap gap-1.5">
                {student.tools.slice(0, 5).map((tool) => (
                  <span
                    key={tool}
                    className="inline-flex items-center rounded-full border border-secondary/15 bg-secondary/5 px-2 py-0.5 text-[10px] font-medium text-secondary/70"
                  >
                    {tool}
                  </span>
                ))}
                {student.tools.length > 5 && (
                  <span className="inline-flex items-center rounded-full border border-third/20 bg-third/10 px-2 py-0.5 text-[10px] text-support/50">
                    +{student.tools.length - 5}
                  </span>
                )}
              </div>
            )}
          </div>

          {/* Arrow */}
          <FontAwesomeIcon
            icon={faArrowRight}
            className="mt-1 h-3.5 w-3.5 shrink-0 text-support/20 transition-all duration-200 group-hover:translate-x-0.5 group-hover:text-secondary/60"
          />
        </div>

        {/* Description */}
        {student.description && (
          <div className="mx-5 mb-4 rounded-lg border border-secondary/10 bg-secondary/5 px-3 py-2.5">
            <p className="line-clamp-2 text-xs leading-relaxed text-secondary/60">
              {student.description}
            </p>
          </div>
        )}

        {/* Divider */}
        <div className="mx-5 h-px bg-gradient-to-r from-transparent via-third/30 to-transparent" />

        {/* Featured Project */}
        <div className="flex flex-1 flex-col gap-3 p-5">
          <p className="text-[10px] font-semibold uppercase tracking-[0.15em] text-secondary/40">
            Featured Project
          </p>

          {student.featuredProject.images.length > 0 && (
            <ProjectSlideshow images={student.featuredProject.images} />
          )}

          <div>
            <h4 className="text-sm font-semibold text-light/90">
              {student.featuredProject.name}
            </h4>
            <p className="mt-1 line-clamp-2 text-xs leading-relaxed text-support/50">
              {student.featuredProject.description}
            </p>
          </div>

          {student.featuredProject.tools.length > 0 && (
            <div className="flex flex-wrap gap-1.5">
              {student.featuredProject.tools.map((tool) => (
                <span
                  key={tool}
                  className="inline-flex items-center rounded-full border border-third/25 bg-third/10 px-2 py-0.5 text-[10px] font-medium text-support/60"
                >
                  {tool}
                </span>
              ))}
            </div>
          )}
        </div>
      </GlassCard>
    </Link>
  );
}

export default function StudentsResult({ query }: Props) {
  const [students, setStudents] = useState<StudentCard[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(false);

  useEffect(() => {
    if (!query) return;
    let cancelled = false;

    async function fetchStudents() {
      setLoading(true);
      setError(false);
      try {
        const res = await fetch(
          `/api/user/search?query=${encodeURIComponent(query)}`,
          {
            next: {
              revalidate: 60, //one minute
            },
          },
        );

        if (!res.ok) throw new Error();
        const data = await res.json();
        if (!cancelled) setStudents(data.students ?? []);
      } catch {
        if (!cancelled) setError(true);
      } finally {
        if (!cancelled) setLoading(false);
      }
    }

    fetchStudents();
    return () => {
      cancelled = true;
    };
  }, [query]);

  if (loading) {
    return <SearchStudentsLoading />;
  }

  if (error) {
    return (
      <p className="py-8 text-center text-sm text-support/40">
        Something went wrong. Please try again.
      </p>
    );
  }

  if (students.length === 0) {
    return (
      <p className="py-8 text-center text-sm text-support/40">
        No students found for &ldquo;{query}&rdquo;
      </p>
    );
  }

  return (
    <div className="columns-1 gap-5 sm:columns-2 lg:columns-3">
      {students.map((student, i) => (
        <motion.div
          key={student.id}
          className="break-inside-avoid mb-5"
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{
            duration: 0.4,
            delay: i * 0.07,
            ease: [0.16, 1, 0.3, 1],
          }}
        >
          <StudentCardItem student={student} />
        </motion.div>
      ))}
    </div>
  );
}
