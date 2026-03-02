"use client";

import { useState, useEffect, useRef } from "react";
import { createPortal } from "react-dom";
import ErrorDisplay from "../components/ErrorDisplay";
import Loading from "../SVGS/Loading";
import Search from "../SVGS/Search";
import Close from "../SVGS/Close";
import * as helpers from "../lib/helpers";

interface Course {
  id: string;
  name: string;
}
interface LinkType {
  id: string;
  name: string;
}
interface SoftwareTool {
  id: string;
  name: string;
}
interface LinkEntry {
  _key: string;
  linkTypeId: string;
  url: string;
  name: string;
}

interface FormState {
  coursesList: Course[];
  linkTypes: LinkType[];
  toolsList: SoftwareTool[];
  firstName: string;
  lastName: string;
  personalEmail: string;
  description: string;
  selectedCourse: string;
  selectedTools: string[];
  certificates: string[];
  links: LinkEntry[];
}

interface FormErrors {
  firstName?: string;
  lastName?: string;
  personalEmail?: string;
  links: Record<string, { url?: string; name?: string }>;
}

interface Props {
  onClose: () => void;
}

export default function EditProfileForm({ onClose }: Props) {
  const [formState, setFormState] = useState<FormState | null>(null);
  const [fetchLoading, setFetchLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [fetchError, setFetchError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [fieldErrors, setFieldErrors] = useState<FormErrors>({ links: {} });

  const [newCert, setNewCert] = useState("");
  const [toolSearch, setToolSearch] = useState("");
  const [toolDropdownOpen, setToolDropdownOpen] = useState(false);
  const toolsRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    document.body.style.overflow = "hidden";
    return () => {
      document.body.style.overflow = "";
    };
  }, []);

  useEffect(() => {
    fetch("/api/user/profile_form", { credentials: "include" })
      .then((res) => {
        if (!res.ok) throw new Error();
        return res.json();
      })
      .then((data) => {
        setFormState({
          coursesList: Array.isArray(data.coursesList) ? data.coursesList : [],
          linkTypes: Array.isArray(data.linkTypes) ? data.linkTypes : [],
          toolsList: Array.isArray(data.toolsList) ? data.toolsList : [],
          firstName: data.firstName ?? "",
          lastName: data.lastName ?? "",
          personalEmail: data.personalEmail ?? "",
          description: data.description ?? "",
          selectedCourse: data.selectedCourse ?? "",
          selectedTools: Array.isArray(data.selectedTools)
            ? data.selectedTools
            : [],
          certificates: Array.isArray(data.certificates)
            ? data.certificates
            : [],
          links: (data.links ?? []).map(
            (l: { id: string; url: string; name: string | null }) => ({
              _key: crypto.randomUUID(),
              linkTypeId: l.id,
              url: l.url,
              name: l.name ?? "",
            }),
          ),
        });
      })
      .catch(() => setFetchError("Failed to load profile data."))
      .finally(() => setFetchLoading(false));
  }, []);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (toolsRef.current && !toolsRef.current.contains(e.target as Node)) {
        setToolDropdownOpen(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, []);

  const setField = <K extends keyof FormState>(key: K, value: FormState[K]) =>
    setFormState((prev) => (prev ? { ...prev, [key]: value } : prev));

  const addTool = (id: string) => {
    setFormState((prev) => {
      if (!prev) return prev;
      return prev.selectedTools.includes(id)
        ? prev
        : { ...prev, selectedTools: [...prev.selectedTools, id] };
    });
    setToolSearch("");
  };

  const removeTool = (id: string) =>
    setFormState((prev) =>
      prev
        ? { ...prev, selectedTools: prev.selectedTools.filter((t) => t !== id) }
        : prev,
    );

  const addCert = () => {
    const trimmed = newCert.trim();
    if (!trimmed || formState?.certificates.includes(trimmed)) return;
    setFormState((prev) =>
      prev ? { ...prev, certificates: [...prev.certificates, trimmed] } : prev,
    );
    setNewCert("");
  };

  const addLink = () => {
    setFormState((prev) => {
      if (!prev?.linkTypes.length) return prev;
      return {
        ...prev,
        links: [
          ...prev.links,
          {
            _key: crypto.randomUUID(),
            linkTypeId: prev.linkTypes[0].id,
            url: "",
            name: "",
          },
        ],
      };
    });
  };

  const updateLink = (
    key: string,
    field: keyof Omit<LinkEntry, "_key">,
    value: string,
  ) =>
    setFormState((prev) =>
      prev
        ? {
            ...prev,
            links: prev.links.map((l) =>
              l._key === key ? { ...l, [field]: value } : l,
            ),
          }
        : prev,
    );

  const removeLink = (key: string) =>
    setFormState((prev) =>
      prev
        ? { ...prev, links: prev.links.filter((l) => l._key !== key) }
        : prev,
    );

  const validate = (): FormErrors => {
    const errors: FormErrors = { links: {} };
    if (!formState) return errors;

    if (formState.firstName.length > 50) errors.firstName = "Max 50 characters";
    if (formState.lastName.length > 50) errors.lastName = "Max 50 characters";
    if (formState.personalEmail) {
      if (formState.personalEmail.length > 250)
        errors.personalEmail = "Max 250 characters";
      else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formState.personalEmail))
        errors.personalEmail = "Enter a valid email address";
    }

    const seenUrls = new Set<string>();
    for (const link of formState.links) {
      const linkErr: { url?: string; name?: string } = {};
      if (!link.url.trim()) {
        linkErr.url = "URL is required";
      } else if (!helpers.isSafeLink(link.url)) {
        linkErr.url = "Must be a valid http/https URL";
      } else if (seenUrls.has(link.url)) {
        linkErr.url = "Duplicate URL";
      } else {
        seenUrls.add(link.url);
        const linkTypeName =
          formState.linkTypes.find((lt) => lt.id === link.linkTypeId)?.name ??
          "";
        const typeError = helpers.validateLinkUrl(linkTypeName, link.url);
        if (typeError) linkErr.url = typeError;
      }
      if (link.name.length > 50) linkErr.name = "Max 50 characters";
      if (Object.keys(linkErr).length > 0) errors.links[link._key] = linkErr;
    }

    return errors;
  };

  const handleSubmit = async () => {
    const errors = validate();
    const hasErrors =
      errors.firstName ||
      errors.lastName ||
      errors.personalEmail ||
      Object.keys(errors.links).length > 0;
    if (hasErrors) {
      setFieldErrors(errors);
      return;
    }
    setSaving(true);
    setSaveError(null);
    console.log(formState);
    // TODO: call PATCH /api/user/profile_form when backend endpoint is implemented
    setSaving(false);
  };

  return createPortal(
    <div className="fixed z-[1000] h-screen w-screen left-0 top-0 bg-primary/85 backdrop-blur-[8px] p-5 animate-[fadeIn_0.3s_ease] flex items-center justify-center">
      <div className="w-full max-w-[560px] max-h-[85vh] flex flex-col rounded-2xl border border-secondary/12 bg-primary/35 backdrop-blur-[20px]">
        {/* Header */}
        <div className="flex items-center justify-between px-8 pt-7 pb-5 border-b border-secondary/10">
          <h2 className="text-[22px] font-bold text-white">Edit Profile</h2>
          <button
            onClick={onClose}
            disabled={saving}
            className="flex h-8 w-8 items-center justify-center rounded-lg text-secondary/50 transition-colors hover:bg-secondary/10 hover:text-secondary cursor-pointer"
          >
            <Close />
          </button>
        </div>

        {/* Scrollable body */}
        <div className="overflow-y-auto flex-1 px-8 py-6 space-y-7">
          {fetchLoading && (
            <div className="flex justify-center py-10 text-secondary/50">
              <Loading />
            </div>
          )}

          {fetchError && <ErrorDisplay text={fetchError} />}

          {formState && !fetchLoading && (
            <>
              {/* Basic info */}
              <section className="space-y-3">
                <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
                  Basic Info
                </p>
                <div className="grid grid-cols-2 gap-3">
                  <div className="flex flex-col gap-1.5">
                    <label className="text-xs text-secondary/60">
                      First Name
                    </label>
                    <input
                      value={formState.firstName}
                      onChange={(e) => {
                        setField("firstName", e.target.value);
                        setFieldErrors((prev) => ({
                          ...prev,
                          firstName: undefined,
                        }));
                      }}
                      placeholder="First name"
                      className={`rounded-xl border px-4 py-2.5 text-sm text-secondary placeholder-secondary/30 outline-none transition-colors ${fieldErrors.firstName ? "border-danger/50 bg-danger/5 focus:border-danger/70" : "border-secondary/15 bg-secondary/5 focus:border-secondary/35 focus:bg-secondary/8"}`}
                    />
                    {fieldErrors.firstName && (
                      <p className="text-xs text-danger">
                        {fieldErrors.firstName}
                      </p>
                    )}
                  </div>
                  <div className="flex flex-col gap-1.5">
                    <label className="text-xs text-secondary/60">
                      Last Name
                    </label>
                    <input
                      value={formState.lastName}
                      onChange={(e) => {
                        setField("lastName", e.target.value);
                        setFieldErrors((prev) => ({
                          ...prev,
                          lastName: undefined,
                        }));
                      }}
                      placeholder="Last name"
                      className={`rounded-xl border px-4 py-2.5 text-sm text-secondary placeholder-secondary/30 outline-none transition-colors ${fieldErrors.lastName ? "border-danger/50 bg-danger/5 focus:border-danger/70" : "border-secondary/15 bg-secondary/5 focus:border-secondary/35 focus:bg-secondary/8"}`}
                    />
                    {fieldErrors.lastName && (
                      <p className="text-xs text-danger">
                        {fieldErrors.lastName}
                      </p>
                    )}
                  </div>
                </div>
                <div className="flex flex-col gap-1.5">
                  <label className="text-xs text-secondary/60">
                    Personal Email
                  </label>
                  <input
                    value={formState.personalEmail}
                    onChange={(e) => {
                      setField("personalEmail", e.target.value);
                      setFieldErrors((prev) => ({
                        ...prev,
                        personalEmail: undefined,
                      }));
                    }}
                    placeholder="your@email.com"
                    type="email"
                    className={`rounded-xl border px-4 py-2.5 text-sm text-secondary placeholder-secondary/30 outline-none transition-colors ${fieldErrors.personalEmail ? "border-danger/50 bg-danger/5 focus:border-danger/70" : "border-secondary/15 bg-secondary/5 focus:border-secondary/35 focus:bg-secondary/8"}`}
                  />
                  {fieldErrors.personalEmail && (
                    <p className="text-xs text-danger">
                      {fieldErrors.personalEmail}
                    </p>
                  )}
                </div>
                <div className="flex flex-col gap-1.5">
                  <label className="text-xs text-secondary/60">
                    Description
                  </label>
                  <textarea
                    value={formState.description}
                    onChange={(e) => setField("description", e.target.value)}
                    placeholder="Tell us about yourself…"
                    rows={3}
                    className="resize-none rounded-xl border border-secondary/15 bg-secondary/5 px-4 py-2.5 text-sm text-secondary placeholder-secondary/30 outline-none transition-colors focus:border-secondary/35 focus:bg-secondary/8"
                  />
                </div>
              </section>

              {/* Course */}
              <section className="space-y-3">
                <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
                  Course
                </p>
                <select
                  value={formState.selectedCourse}
                  onChange={(e) => setField("selectedCourse", e.target.value)}
                  className="w-full rounded-xl border border-secondary/15 bg-primary/60 px-4 py-2.5 text-sm text-secondary outline-none transition-colors focus:border-secondary/35"
                >
                  <option value="">— No course selected —</option>
                  {formState.coursesList.map((c) => (
                    <option key={c.id} value={c.id}>
                      {c.name}
                    </option>
                  ))}
                </select>
              </section>

              {/* Tools */}
              <section className="space-y-3">
                <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
                  Tech Interests ({formState.selectedTools.length})
                </p>
                {formState.selectedTools.length > 0 && (
                  <div className="flex flex-wrap gap-2">
                    {formState.selectedTools.map((id) => {
                      const tool = formState.toolsList.find((t) => t.id === id);
                      if (!tool) return null;
                      return (
                        <span
                          key={id}
                          className="inline-flex items-center gap-1.5 rounded-full border border-secondary/40 bg-secondary/12 px-3 py-1 text-xs font-medium text-secondary"
                        >
                          {tool.name}
                          <button
                            type="button"
                            onClick={() => removeTool(id)}
                            className="text-secondary/50 transition-colors hover:text-secondary cursor-pointer"
                          >
                            <Close />
                          </button>
                        </span>
                      );
                    })}
                  </div>
                )}
                <div ref={toolsRef} className="relative">
                  <div className="flex items-center gap-2 rounded-xl border border-secondary/15 bg-secondary/5 px-4 py-2.5 transition-colors focus-within:border-secondary/35 focus-within:bg-secondary/8">
                    <Search />
                    <input
                      value={toolSearch}
                      onChange={(e) => setToolSearch(e.target.value)}
                      onFocus={() => setToolDropdownOpen(true)}
                      onKeyDown={(e) =>
                        e.key === "Escape" && setToolDropdownOpen(false)
                      }
                      placeholder="Search tools…"
                      className="flex-1 bg-transparent text-sm text-secondary placeholder-secondary/30 outline-none"
                    />
                  </div>
                  {toolDropdownOpen && (
                    <div className="absolute z-10 mt-1.5 w-full rounded-xl border border-secondary/15 bg-[#0d2426] shadow-[0_8px_32px_rgba(0,0,0,0.4)] overflow-hidden">
                      <ul className="max-h-48 overflow-y-auto py-1">
                        {formState.toolsList
                          .filter(
                            (t) =>
                              !formState.selectedTools.includes(t.id) &&
                              t.name
                                .toLowerCase()
                                .includes(toolSearch.toLowerCase()),
                          )
                          .map((tool) => (
                            <li key={tool.id}>
                              <button
                                type="button"
                                onMouseDown={(e) => e.preventDefault()}
                                onClick={() => addTool(tool.id)}
                                className="w-full px-4 py-2.5 text-left text-sm text-secondary/70 transition-colors hover:bg-secondary/8 hover:text-secondary"
                              >
                                {tool.name}
                              </button>
                            </li>
                          ))}
                        {formState.toolsList.filter(
                          (t) =>
                            !formState.selectedTools.includes(t.id) &&
                            t.name
                              .toLowerCase()
                              .includes(toolSearch.toLowerCase()),
                        ).length === 0 && (
                          <li className="px-4 py-3 text-xs text-secondary/35">
                            {formState.selectedTools.length ===
                            formState.toolsList.length
                              ? "All tools selected"
                              : "No tools match your search"}
                          </li>
                        )}
                      </ul>
                    </div>
                  )}
                </div>
              </section>

              {/* Certificates */}
              <section className="space-y-3">
                <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
                  Certificates ({formState.certificates.length})
                </p>
                {formState.certificates.length > 0 && (
                  <div className="flex flex-wrap gap-2">
                    {formState.certificates.map((cert) => (
                      <span
                        key={cert}
                        className="inline-flex items-center gap-1.5 rounded-lg border border-secondary/15 bg-secondary/5 px-3 py-1 text-xs font-medium text-secondary/80"
                      >
                        {cert}
                        <button
                          type="button"
                          onClick={() =>
                            setField(
                              "certificates",
                              formState.certificates.filter((c) => c !== cert),
                            )
                          }
                          className="text-secondary/40 hover:text-secondary/80 transition-colors"
                        >
                          <Close />
                        </button>
                      </span>
                    ))}
                  </div>
                )}
                <div className="flex gap-2">
                  <input
                    value={newCert}
                    onChange={(e) => setNewCert(e.target.value)}
                    onKeyDown={(e) =>
                      e.key === "Enter" && (e.preventDefault(), addCert())
                    }
                    placeholder="e.g. AWS Certified Developer"
                    className="flex-1 rounded-xl border border-secondary/15 bg-secondary/5 px-4 py-2.5 text-sm text-secondary placeholder-secondary/30 outline-none transition-colors focus:border-secondary/35 focus:bg-secondary/8"
                  />
                  <button
                    type="button"
                    onClick={addCert}
                    className="rounded-xl border border-secondary/20 bg-secondary/8 px-4 py-2.5 text-sm font-semibold text-secondary transition-colors hover:bg-secondary/15"
                  >
                    Add
                  </button>
                </div>
              </section>

              {/* Links */}
              <section className="space-y-3">
                <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
                  Links ({formState.links.length})
                </p>
                <div className="space-y-2">
                  {formState.links.map((link) => (
                    <div
                      key={link._key}
                      className="flex flex-col gap-2 rounded-xl border border-secondary/10 bg-secondary/3 p-3"
                    >
                      <div className="flex gap-2">
                        <select
                          value={link.linkTypeId}
                          onChange={(e) =>
                            updateLink(link._key, "linkTypeId", e.target.value)
                          }
                          className="rounded-lg border border-secondary/15 bg-primary/60 px-3 py-2 text-xs text-secondary outline-none focus:border-secondary/35"
                        >
                          {formState.linkTypes.map((lt) => (
                            <option key={lt.id} value={lt.id}>
                              {lt.name}
                            </option>
                          ))}
                        </select>
                        <button
                          type="button"
                          onClick={() => removeLink(link._key)}
                          className="ml-auto flex h-8 w-8 items-center justify-center rounded-lg text-secondary/30 transition-colors hover:bg-secondary/10 hover:text-secondary/70"
                        >
                          <svg
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            strokeWidth={2}
                            className="h-3.5 w-3.5"
                          >
                            <path
                              strokeLinecap="round"
                              d="M6 6l12 12M18 6L6 18"
                            />
                          </svg>
                        </button>
                      </div>
                      <div className="flex flex-col gap-1">
                        <input
                          value={link.url}
                          onChange={(e) => {
                            updateLink(link._key, "url", e.target.value);
                            setFieldErrors((prev) => ({
                              ...prev,
                              links: {
                                ...prev.links,
                                [link._key]: {
                                  ...prev.links[link._key],
                                  url: undefined,
                                },
                              },
                            }));
                          }}
                          placeholder="https://..."
                          className={`rounded-lg border px-3 py-2 text-xs text-secondary placeholder-secondary/30 outline-none transition-colors ${fieldErrors.links[link._key]?.url ? "border-danger/50 bg-danger/5 focus:border-danger/70" : "border-secondary/15 bg-secondary/5 focus:border-secondary/35"}`}
                        />
                        {fieldErrors.links[link._key]?.url && (
                          <p className="text-xs text-danger">
                            {fieldErrors.links[link._key].url}
                          </p>
                        )}
                      </div>
                      <div className="flex flex-col gap-1">
                        <input
                          value={link.name}
                          onChange={(e) => {
                            updateLink(link._key, "name", e.target.value);
                            setFieldErrors((prev) => ({
                              ...prev,
                              links: {
                                ...prev.links,
                                [link._key]: {
                                  ...prev.links[link._key],
                                  name: undefined,
                                },
                              },
                            }));
                          }}
                          placeholder="Display name (optional)"
                          className={`rounded-lg border px-3 py-2 text-xs text-secondary placeholder-secondary/30 outline-none transition-colors ${fieldErrors.links[link._key]?.name ? "border-danger/50 bg-danger/5 focus:border-danger/70" : "border-secondary/15 bg-secondary/5 focus:border-secondary/35"}`}
                        />
                        {fieldErrors.links[link._key]?.name && (
                          <p className="text-xs text-danger">
                            {fieldErrors.links[link._key].name}
                          </p>
                        )}
                      </div>
                    </div>
                  ))}
                </div>
                <button
                  type="button"
                  onClick={addLink}
                  className="w-full rounded-xl border border-dashed border-secondary/20 py-2 text-xs font-medium text-secondary/50 transition-colors hover:border-secondary/35 hover:text-secondary/70"
                >
                  + Add Link
                </button>
              </section>
            </>
          )}
        </div>

        {/* Footer */}
        {formState && !fetchLoading && (
          <div className="flex flex-col gap-3 border-t border-secondary/10 px-8 py-5">
            {saveError && <ErrorDisplay text={saveError} />}
            <div className="flex justify-end gap-3">
              <button
                onClick={onClose}
                disabled={saving}
                className="inline-flex cursor-pointer items-center gap-2 rounded-[10px] border border-secondary/20 bg-secondary/8 px-5 py-2.5 font-[Poppins] text-sm font-semibold text-secondary transition-all duration-250 ease-in-out hover:bg-secondary/15"
              >
                Cancel
              </button>
              <button
                onClick={handleSubmit}
                disabled={saving}
                className={`inline-flex items-center gap-2 rounded-[10px] border-none px-5 py-2.5 font-[Poppins] text-sm font-semibold text-primary transition-all duration-250 ease-in-out ${
                  saving
                    ? "cursor-not-allowed bg-secondary/30"
                    : "cursor-pointer bg-[linear-gradient(135deg,var(--color-secondary),var(--color-support))] shadow-[0_4px_20px_var(--color-secondary)/0.25] hover:shadow-[0_6px_24px_var(--color-secondary)/0.35]"
                }`}
              >
                {saving ? (
                  <>
                    <Loading />
                    Saving…
                  </>
                ) : (
                  <>Save Changes</>
                )}
              </button>
            </div>
          </div>
        )}
      </div>
    </div>,
    document.body,
  );
}
