"use client";

import { useState, useEffect, useRef } from "react";
import { useRouter } from "next/navigation";
import { createPortal } from "react-dom";
import Image from "next/image";
import ErrorDisplay from "../components/ErrorDisplay";
import Loading from "../SVGS/Loading";
import Search from "../SVGS/Search";
import Close from "../SVGS/Close";
import {
  getProjectImgUrl,
  isSafeLink,
  validateLinkUrl,
  MAX_IMAGE_SIZE_BYTES,
  MAX_IMAGE_SIZE_MB,
  ALLOWED_IMAGE_TYPES,
} from "../lib/helpers";
import type { Project } from "./page";

const MAX_IMAGES = 5;

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
interface FormData {
  name: string;
  description: string;
  live_link: string;
  selectedTools: string[];
  links: LinkEntry[];
  existingImages: string[];
}
interface FormState {
  data: FormData;
  linkTypes: LinkType[];
  toolsList: SoftwareTool[];
}
interface FieldErrors {
  name?: string;
  description?: string;
  liveLink?: string;
  links: Record<string, { url?: string }>;
  images?: string;
}

interface Props {
  project?: Project;
  onClose: () => void;
}

export default function UpsertProjectModal({ project, onClose }: Props) {
  const router = useRouter();
  const isEdit = !!project;

  const [formState, setFormState] = useState<FormState | null>(null);
  const [fetchLoading, setFetchLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [fetchError, setFetchError] = useState<string | null>(null);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [fieldErrors, setFieldErrors] = useState<FieldErrors>({ links: {} });

  const [newImages, setNewImages] = useState<File[]>([]);
  const [previewUrls, setPreviewUrls] = useState<string[]>([]);
  const [toolSearch, setToolSearch] = useState("");
  const [toolDropdownOpen, setToolDropdownOpen] = useState(false);
  const toolsRef = useRef<HTMLDivElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    document.body.style.overflow = "hidden";
    return () => {
      document.body.style.overflow = "";
    };
  }, []);

  useEffect(() => {
    const url = isEdit
      ? `/api/user/upsert_project?project_id=${project.id}`
      : "/api/user/upsert_project";

    fetch(url, { credentials: "include" })
      .then((res) => {
        if (!res.ok) throw new Error();
        return res.json();
      })
      .then((data) => {
        setFormState({
          linkTypes: Array.isArray(data.linkTypes) ? data.linkTypes : [],
          toolsList: Array.isArray(data.toolsList) ? data.toolsList : [],
          data: {
            name: data.name ?? "",
            description: data.description ?? "",
            live_link: data.live_link ?? "",
            selectedTools: Array.isArray(data.selectedTools)
              ? data.selectedTools
              : [],
            existingImages: Array.isArray(data.existingImages)
              ? data.existingImages
              : [],
            links: (data.links ?? []).map(
              (l: { id: string; url: string; name: string | null }) => ({
                _key: crypto.randomUUID(),
                linkTypeId: l.id,
                url: l.url,
                name: l.name ?? "",
              }),
            ),
          },
        });
      })
      .catch(() => setFetchError("Failed to load project data."))
      .finally(() => setFetchLoading(false));
  }, [isEdit, project?.id]);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      if (toolsRef.current && !toolsRef.current.contains(e.target as Node)) {
        setToolDropdownOpen(false);
      }
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, []);

  useEffect(() => {
    const urls = newImages.map((file) => URL.createObjectURL(file));
    setPreviewUrls(urls);
    return () => urls.forEach((url) => URL.revokeObjectURL(url));
  }, [newImages]);

  const setField = <K extends keyof FormData>(key: K, value: FormData[K]) =>
    setFormState((prev) =>
      prev ? { ...prev, data: { ...prev.data, [key]: value } } : prev,
    );

  const addTool = (id: string) => {
    setFormState((prev) => {
      if (!prev || prev.data.selectedTools.includes(id)) return prev;
      return {
        ...prev,
        data: { ...prev.data, selectedTools: [...prev.data.selectedTools, id] },
      };
    });
    setToolSearch("");
  };

  const removeTool = (id: string) =>
    setFormState((prev) =>
      prev
        ? {
            ...prev,
            data: {
              ...prev.data,
              selectedTools: prev.data.selectedTools.filter((t) => t !== id),
            },
          }
        : prev,
    );

  const addLink = () => {
    setFormState((prev) => {
      if (!prev?.linkTypes.length) return prev;
      return {
        ...prev,
        data: {
          ...prev.data,
          links: [
            ...prev.data.links,
            {
              _key: crypto.randomUUID(),
              linkTypeId: prev.linkTypes[0].id,
              url: "",
              name: "",
            },
          ],
        },
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
            data: {
              ...prev.data,
              links: prev.data.links.map((l) =>
                l._key === key ? { ...l, [field]: value } : l,
              ),
            },
          }
        : prev,
    );

  const removeLink = (key: string) =>
    setFormState((prev) =>
      prev
        ? {
            ...prev,
            data: {
              ...prev.data,
              links: prev.data.links.filter((l) => l._key !== key),
            },
          }
        : prev,
    );

  const removeExistingImage = (filename: string) => {
    setFormState((prev) =>
      prev
        ? {
            ...prev,
            data: {
              ...prev.data,
              existingImages: prev.data.existingImages.filter(
                (img) => img !== filename,
              ),
            },
          }
        : prev,
    );
    setFieldErrors((prev) => ({ ...prev, images: undefined }));
  };

  const removeNewImage = (index: number) => {
    setNewImages((prev) => prev.filter((_, i) => i !== index));
    setFieldErrors((prev) => ({ ...prev, images: undefined }));
  };

  const totalImages =
    (formState?.data.existingImages.length ?? 0) + newImages.length;
  const remainingSlots = MAX_IMAGES - totalImages;

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(e.target.files ?? []);
    if (!files.length) return;
    const oversized = files.filter((f) => f.size > MAX_IMAGE_SIZE_BYTES);
    if (oversized.length > 0) {
      setFieldErrors((prev) => ({
        ...prev,
        images: `Each image must be under ${MAX_IMAGE_SIZE_MB}MB`,
      }));
      e.target.value = "";
      return;
    }
    const available = MAX_IMAGES - totalImages;
    const toAdd = files.slice(0, available);
    setNewImages((prev) => [...prev, ...toAdd]);
    setFieldErrors((prev) => ({ ...prev, images: undefined }));
    e.target.value = "";
  };

  const validate = (): FieldErrors => {
    const errors: FieldErrors = { links: {} };
    if (!formState) return errors;

    if (!formState.data.name.trim()) errors.name = "Required";
    else if (formState.data.name.length > 250)
      errors.name = "Max 250 characters";

    if (!formState.data.description.trim()) errors.description = "Required";

    if (
      formState.data.live_link.trim() &&
      !isSafeLink(formState.data.live_link)
    )
      errors.liveLink = "Must be a valid http/https URL";

    if (totalImages > MAX_IMAGES)
      errors.images = `Maximum ${MAX_IMAGES} images allowed`;

    const seenUrls = new Set<string>();
    for (const link of formState.data.links) {
      const linkErr: { url?: string } = {};
      if (!link.url.trim()) {
        linkErr.url = "URL is required";
      } else if (!isSafeLink(link.url)) {
        linkErr.url = "Must be a valid http/https URL";
      } else if (seenUrls.has(link.url)) {
        linkErr.url = "Duplicate URL";
      } else {
        seenUrls.add(link.url);
        const linkTypeName =
          formState.linkTypes.find((lt) => lt.id === link.linkTypeId)?.name ??
          "";
        const typeError = validateLinkUrl(linkTypeName, link.url);
        if (typeError) linkErr.url = typeError;
      }
      if (Object.keys(linkErr).length > 0) errors.links[link._key] = linkErr;
    }

    return errors;
  };

  const handleSubmit = async () => {
    const errors = validate();
    const hasErrors =
      errors.name ||
      errors.description ||
      errors.liveLink ||
      errors.images ||
      Object.keys(errors.links).length > 0;
    if (hasErrors) {
      setFieldErrors(errors);
      return;
    }

    setSaving(true);
    setSaveError(null);
    try {
      const { data } = formState!;
      const body = new FormData();

      const payload = JSON.stringify({
        id: project?.id ?? null,
        name: data.name,
        description: data.description,
        live_link: data.live_link || null,
        selectedTools: data.selectedTools,
        existingImages: data.existingImages,
        links: data.links.map(({ linkTypeId, url, name }) => ({
          linkTypeId,
          url,
          name,
        })),
      });
      body.append("data", payload);
      for (const file of newImages) {
        body.append("new_files", file);
      }

      const res = await fetch("/api/user/upsert_project", {
        method: isEdit ? "PATCH" : "POST",
        credentials: "include",
        body,
      });
      if (!res.ok) throw new Error();
      router.refresh();
      onClose();
    } catch {
      setSaveError("Failed to save project. Please try again.");
    } finally {
      setSaving(false);
    }
  };

  return createPortal(
    <div className="fixed z-[1000] h-screen w-screen left-0 top-0 bg-primary/85 backdrop-blur-[8px] p-5 animate-[fadeIn_0.3s_ease] flex items-center justify-center">
      <div className="w-full max-w-[560px] max-h-[85vh] flex flex-col rounded-2xl border border-secondary/12 bg-primary/35 backdrop-blur-[20px]">
        {/* Header */}
        <div className="flex items-center justify-between px-8 pt-7 pb-5 border-b border-secondary/10">
          <h2 className="text-[22px] font-bold text-white">
            {isEdit ? "Edit Project" : "Add Project"}
          </h2>
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
                <div className="flex flex-col gap-1.5">
                  <label className="text-xs text-secondary/60">
                    Project Name
                  </label>
                  <input
                    value={formState.data.name}
                    onChange={(e) => {
                      setField("name", e.target.value);
                      setFieldErrors((prev) => ({ ...prev, name: undefined }));
                    }}
                    placeholder="My Awesome Project"
                    className={`rounded-xl border px-4 py-2.5 text-sm text-secondary placeholder-secondary/30 outline-none transition-colors ${fieldErrors.name ? "border-danger/50 bg-danger/5 focus:border-danger/70" : "border-secondary/15 bg-secondary/5 focus:border-secondary/35 focus:bg-secondary/8"}`}
                  />
                  {fieldErrors.name && (
                    <p className="text-xs text-danger">{fieldErrors.name}</p>
                  )}
                </div>
                <div className="flex flex-col gap-1.5">
                  <label className="text-xs text-secondary/60">
                    Description
                  </label>
                  <textarea
                    value={formState.data.description}
                    onChange={(e) => {
                      setField("description", e.target.value);
                      setFieldErrors((prev) => ({
                        ...prev,
                        description: undefined,
                      }));
                    }}
                    placeholder="What does this project do?"
                    rows={3}
                    className={`resize-none rounded-xl border px-4 py-2.5 text-sm text-secondary placeholder-secondary/30 outline-none transition-colors ${fieldErrors.description ? "border-danger/50 bg-danger/5 focus:border-danger/70" : "border-secondary/15 bg-secondary/5 focus:border-secondary/35 focus:bg-secondary/8"}`}
                  />
                  {fieldErrors.description && (
                    <p className="text-xs text-danger">
                      {fieldErrors.description}
                    </p>
                  )}
                </div>
                <div className="flex flex-col gap-1.5">
                  <label className="text-xs text-secondary/60">Live Link</label>
                  <input
                    value={formState.data.live_link}
                    onChange={(e) => {
                      setField("live_link", e.target.value);
                      setFieldErrors((prev) => ({
                        ...prev,
                        liveLink: undefined,
                      }));
                    }}
                    placeholder="https://myproject.com"
                    className={`rounded-xl border px-4 py-2.5 text-sm text-secondary placeholder-secondary/30 outline-none transition-colors ${fieldErrors.liveLink ? "border-danger/50 bg-danger/5 focus:border-danger/70" : "border-secondary/15 bg-secondary/5 focus:border-secondary/35 focus:bg-secondary/8"}`}
                  />
                  {fieldErrors.liveLink && (
                    <p className="text-xs text-danger">
                      {fieldErrors.liveLink}
                    </p>
                  )}
                </div>
              </section>

              {/* Tools */}
              <section className="space-y-3">
                <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
                  Tools ({formState.data.selectedTools.length})
                </p>
                {formState.data.selectedTools.length > 0 && (
                  <div className="flex flex-wrap gap-2">
                    {formState.data.selectedTools.map((id) => {
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
                              !formState.data.selectedTools.includes(t.id) &&
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
                            !formState.data.selectedTools.includes(t.id) &&
                            t.name
                              .toLowerCase()
                              .includes(toolSearch.toLowerCase()),
                        ).length === 0 && (
                          <li className="px-4 py-3 text-xs text-secondary/35">
                            {formState.data.selectedTools.length ===
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

              {/* Images */}
              <section className="space-y-3">
                <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
                  Images ({totalImages}/{MAX_IMAGES})
                </p>
                {fieldErrors.images && (
                  <p className="text-xs text-danger">{fieldErrors.images}</p>
                )}
                {(formState.data.existingImages.length > 0 ||
                  newImages.length > 0) && (
                  <div className="flex flex-wrap gap-2">
                    {formState.data.existingImages.map((filename) => (
                      <div key={filename} className="relative group">
                        <div className="h-20 w-20 overflow-hidden rounded-xl border border-secondary/15 bg-secondary/5">
                          <Image
                            src={getProjectImgUrl(filename)}
                            alt="Project image"
                            width={80}
                            height={80}
                            className="h-full w-full object-cover"
                            unoptimized
                          />
                        </div>
                        <button
                          type="button"
                          onClick={() => removeExistingImage(filename)}
                          className="absolute -top-1.5 -right-1.5 flex h-5 w-5 items-center justify-center rounded-full bg-danger/80 text-white opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer"
                        >
                          <Close />
                        </button>
                      </div>
                    ))}
                    {newImages.map((file, i) => (
                      <div key={i} className="relative group">
                        <div className="h-20 w-20 overflow-hidden rounded-xl border border-secondary/30 bg-secondary/5">
                          <Image
                            src={previewUrls[i]}
                            alt="New image"
                            width={80}
                            height={80}
                            className="h-full w-full object-cover"
                            unoptimized
                          />
                        </div>
                        <button
                          type="button"
                          onClick={() => removeNewImage(i)}
                          className="absolute -top-1.5 -right-1.5 flex h-5 w-5 items-center justify-center rounded-full bg-danger/80 text-white opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer"
                        >
                          <Close />
                        </button>
                      </div>
                    ))}
                  </div>
                )}
                {remainingSlots > 0 && (
                  <>
                    <input
                      ref={fileInputRef}
                      type="file"
                      accept={`${ALLOWED_IMAGE_TYPES.join(",")}`}
                      multiple
                      className="hidden"
                      onChange={handleFileChange}
                    />
                    <button
                      type="button"
                      onClick={() => fileInputRef.current?.click()}
                      className="w-full rounded-xl border border-dashed border-secondary/20 py-3 text-xs font-medium text-secondary/50 transition-colors hover:border-secondary/35 hover:text-secondary/70 cursor-pointer"
                    >
                      + Add Images ({remainingSlots} slot
                      {remainingSlots !== 1 ? "s" : ""} remaining)
                    </button>
                  </>
                )}
              </section>

              {/* Links */}
              <section className="space-y-3">
                <p className="text-xs font-semibold uppercase tracking-wider text-secondary/50">
                  Links ({formState.data.links.length})
                </p>
                <div className="space-y-2">
                  {formState.data.links.map((link) => (
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
                      <input
                        value={link.name}
                        onChange={(e) =>
                          updateLink(link._key, "name", e.target.value)
                        }
                        placeholder="Display name (optional)"
                        className="rounded-lg border border-secondary/15 bg-secondary/5 px-3 py-2 text-xs text-secondary placeholder-secondary/30 outline-none transition-colors focus:border-secondary/35"
                      />
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
                ) : isEdit ? (
                  "Save Changes"
                ) : (
                  "Create Project"
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
