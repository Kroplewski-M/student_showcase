"use client";

import { useState } from "react";
import Image from "next/image";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faSpinner,
  faCircleUser,
  faBan,
  faCircleCheck,
} from "@fortawesome/free-solid-svg-icons";
import GlassCard from "../components/GlassCard";
import ErrorDisplay from "../components/ErrorDisplay";
import validateStudentId, { getProfileImgUrl } from "../lib/helpers";

interface FindStudent {
  id: string;
  image_name: string | null;
  suspended: boolean;
}

export default function StudentSearch() {
  const [query, setQuery] = useState("");
  const [queryError, setQueryError] = useState<string | null>(null);
  const [searching, setSearching] = useState(false);
  const [searchError, setSearchError] = useState<string | null>(null);
  const [student, setStudent] = useState<FindStudent | null>(null);
  const [notFound, setNotFound] = useState(false);
  const [actionLoading, setActionLoading] = useState(false);
  const [actionError, setActionError] = useState<string | null>(null);

  async function handleSearch(e: React.FormEvent) {
    e.preventDefault();
    const idError = validateStudentId(query);
    if (idError) {
      setQueryError(idError);
      return;
    }
    setQueryError(null);
    setSearchError(null);
    setNotFound(false);
    setStudent(null);
    setActionError(null);
    setSearching(true);
    try {
      const res = await fetch(
        `/api/admin/search_student/${encodeURIComponent(query.trim())}`,
        { credentials: "include" },
      );
      if (!res.ok) throw new Error();
      const data: FindStudent | null = await res.json();
      if (!data) {
        setNotFound(true);
      } else {
        setStudent(data);
      }
    } catch {
      setSearchError("Failed to search for student. Please try again.");
    } finally {
      setSearching(false);
    }
  }

  async function handleSuspend() {
    if (!student) return;
    setActionError(null);
    setActionLoading(true);
    const endpoint = student.suspended
      ? "unsuspend_student"
      : "suspend_student";
    try {
      const res = await fetch(
        `/api/admin/${endpoint}/${encodeURIComponent(student.id)}`,
        { method: "POST", credentials: "include" },
      );
      if (!res.ok) throw new Error();
      setStudent((prev) => prev && { ...prev, suspended: !prev.suspended });
    } catch {
      setActionError(
        `Failed to ${student.suspended ? "unsuspend" : "suspend"} student. Please try again.`,
      );
    } finally {
      setActionLoading(false);
    }
  }

  return (
    <div className="flex flex-col gap-6">
      <form onSubmit={handleSearch} className="flex flex-col gap-3">
        <div className="flex gap-3">
          <div className="relative flex-1">
            <span className="absolute left-4 top-[15px] -translate-y-1/2 h-3.5 w-3.5 font-medium text-support/50 select-none">
              U
            </span>
            <input
              type="text"
              value={query}
              onChange={(e) => {
                setQuery(e.target.value.replace(/\D/g, ""));
                if (queryError) setQueryError(null);
              }}
              placeholder="Enter 7-digit student ID"
              maxLength={7}
              inputMode="numeric"
              pattern="\d*"
              className={`w-full rounded-xl border pl-10 pr-4 py-2.5 text-sm text-secondary placeholder-secondary/30 outline-none transition-colors ${
                queryError
                  ? "border-danger/50 bg-danger/5 focus:border-danger/70"
                  : "border-secondary/15 bg-secondary/5 focus:border-secondary/35 focus:bg-secondary/8"
              }`}
            />
          </div>
          <button
            type="submit"
            disabled={searching}
            className="inline-flex cursor-pointer items-center gap-2 rounded-[10px] bg-secondary px-5 py-2.5 text-sm font-semibold text-primary transition-all duration-250 hover:bg-secondary/85 disabled:opacity-60 disabled:cursor-not-allowed"
          >
            {searching ? (
              <FontAwesomeIcon
                icon={faSpinner}
                className="animate-spin h-4 w-4"
              />
            ) : (
              "Search"
            )}
          </button>
        </div>
        {queryError && (
          <p className="text-xs text-danger/80 pl-1">{queryError}</p>
        )}
        <ErrorDisplay text={searchError} />
      </form>

      {notFound && (
        <p className="text-sm text-secondary/50 text-center py-4">
          No student found with ID{" "}
          <span className="text-secondary font-semibold">{query.trim()}</span>.
        </p>
      )}

      {student && (
        <GlassCard className="p-6">
          <div className="flex items-center gap-5">
            <div className="relative h-16 w-16 shrink-0 overflow-hidden rounded-full border border-secondary/15">
              {student.image_name ? (
                <Image
                  src={getProfileImgUrl(student.image_name)}
                  alt={`${student.id} profile`}
                  fill
                  unoptimized
                  className="object-cover"
                />
              ) : (
                <div className="flex h-full w-full items-center justify-center bg-secondary/5">
                  <FontAwesomeIcon
                    icon={faCircleUser}
                    className="h-8 w-8 text-secondary/30"
                  />
                </div>
              )}
            </div>

            <div className="flex flex-1 flex-col gap-1 min-w-0">
              <span className="text-base font-semibold text-light truncate">
                {student.id}
              </span>
              <div className="flex items-center gap-1.5">
                <FontAwesomeIcon
                  icon={student.suspended ? faBan : faCircleCheck}
                  className={`h-3 w-3 ${student.suspended ? "text-danger" : "text-emerald-400"}`}
                />
                <span
                  className={`text-xs font-medium ${student.suspended ? "text-danger" : "text-emerald-400"}`}
                >
                  {student.suspended ? "Suspended" : "Active"}
                </span>
              </div>
            </div>

            <button
              type="button"
              onClick={handleSuspend}
              disabled={actionLoading}
              className={`inline-flex shrink-0 cursor-pointer items-center gap-2 rounded-[10px] px-5 py-2.5 text-sm font-semibold transition-all duration-250 disabled:opacity-60 disabled:cursor-not-allowed ${
                student.suspended
                  ? "bg-secondary/10 border border-secondary/20 text-secondary hover:bg-secondary/15"
                  : "bg-danger/15 border border-danger/30 text-danger hover:bg-danger/25"
              }`}
            >
              {actionLoading ? (
                <FontAwesomeIcon
                  icon={faSpinner}
                  className="animate-spin h-4 w-4"
                />
              ) : student.suspended ? (
                "Unsuspend"
              ) : (
                "Suspend"
              )}
            </button>
          </div>

          {actionError && (
            <div className="mt-4">
              <ErrorDisplay text={actionError} />
            </div>
          )}
        </GlassCard>
      )}
    </div>
  );
}
