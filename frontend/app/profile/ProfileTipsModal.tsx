"use client";

import { useEffect } from "react";
import { createPortal } from "react-dom";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faXmark } from "@fortawesome/free-solid-svg-icons";

const TIPS = [
  {
    number: "Tip 1",
    title: "Name your project — don't label it",
    body: "A project title should tell someone what you built and why it matters. The form of the thing ('Website', 'App', 'System') tells them nothing. The subject and the problem it solves tells them everything.",
    bad: '"A Website"',
    good: "International Motor Museum Digital Archive — searchable catalogue for 200+ historic vehicles",
  },
  {
    number: "Tip 2",
    title:
      "Write your bio for the person reading it, not the person writing it",
    body: 'Every student on every course in every university has "strong communication skills developed through team projects." That sentence does not distinguish you — it buries you. Write one or two sentences about what you actually built, what you find genuinely interesting, or what kind of problem you want to work on next.',
    bad: "I have developed strong communication skills through participating in team projects and understanding different points of view.",
    good: "I spent my final year applying six optimisations to Grover's search algorithm and building a live archive for a museum. I like problems where the answer isn't obvious yet.",
  },
  {
    number: "Tip 3",
    title: "Say what it does, not just what you used to build it",
    body: "Your tools list (Python, React, MySQL) tells a technical person you know the tools. Your project description tells everyone what you did with them. Lead with the outcome, then mention the stack.",
    bad: "Built using Python, TensorFlow, and Jupyter Notebook.",
    good: "A classifier that identifies plant diseases from smartphone photos with 91% accuracy — built in Python using TensorFlow.",
  },
  {
    number: "Tip 4",
    title: "Label your links so they land",
    body: 'If you\'re linking to something other than GitHub or LinkedIn, tell the reader what they\'ll find there. "Website" could be anything. "Live demo — trolleybus museum archive" tells them exactly what to click and why.',
    bad: null,
    good: null,
  },
];

interface Props {
  onClose: () => void;
}

export default function ProfileTipsModal({ onClose }: Props) {
  useEffect(() => {
    const prev = document.body.style.overflow;
    document.body.style.overflow = "hidden";
    return () => {
      document.body.style.overflow = prev;
    };
  }, []);

  return createPortal(
    <div
      className="fixed z-[1000] h-screen w-screen left-0 top-0 flex items-center justify-center bg-primary/85 backdrop-blur-[8px] p-5 animate-[fadeIn_0.2s_ease]"
      role="dialog"
      aria-modal="true"
      aria-labelledby="tips-modal-title"
      onClick={onClose}
    >
      <div
        className="w-full max-w-[600px] max-h-[85vh] flex flex-col rounded-2xl bg-[#0d2426] border border-secondary/15"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-start justify-between px-5 pt-5 pb-4 border-b border-secondary/10 flex-shrink-0">
          <div>
            <p className="text-[10px] font-semibold text-third uppercase tracking-widest mb-1">
              Quick tips
            </p>
            <h2
              id="tips-modal-title"
              className="text-lg font-bold text-white leading-snug"
            >
              You&apos;re in. Now make them stop scrolling.
            </h2>
          </div>
          <button
            type="button"
            onClick={onClose}
            aria-label="Close tips"
            className="ml-3 mt-0.5 flex-shrink-0 flex h-8 w-8 items-center justify-center rounded-lg text-secondary/40 transition-colors hover:bg-secondary/10 hover:text-secondary cursor-pointer"
          >
            <FontAwesomeIcon icon={faXmark} className="h-4 w-4" />
          </button>
        </div>

        {/* Scrollable tips */}
        <div className="overflow-y-auto flex-1 overscroll-contain flex flex-col gap-4 px-5 py-4">
          {TIPS.map((tip) => (
            <div
              key={tip.number}
              className="rounded-xl border border-secondary/10"
            >
              <div className="bg-[#204346] px-4 py-3">
                <span className="text-[10px] font-semibold text-third uppercase tracking-widest">
                  {tip.number}
                </span>
                <p className="mt-1 text-sm font-semibold text-white leading-snug">
                  {tip.title}
                </p>
              </div>

              <div className="bg-third/5 px-4 py-3 flex flex-col gap-3">
                <p className="text-[13px] leading-relaxed text-secondary/75">
                  {tip.body}
                </p>
                {(tip.bad || tip.good) && (
                  <div className="flex flex-col sm:grid sm:grid-cols-2 gap-2">
                    {tip.bad && (
                      <div className="rounded-lg bg-red-500/10 border border-red-500/20 p-3">
                        <p className="text-[10px] font-semibold text-red-400 uppercase tracking-wider mb-1.5">
                          Instead of this
                        </p>
                        <p className="text-[13px] text-secondary/65 italic leading-relaxed">
                          {tip.bad}
                        </p>
                      </div>
                    )}
                    {tip.good && (
                      <div className="rounded-lg bg-green-500/10 border border-green-500/20 p-3">
                        <p className="text-[10px] font-semibold text-green-400 uppercase tracking-wider mb-1.5">
                          Try this
                        </p>
                        <p className="text-[13px] text-secondary/80 leading-relaxed">
                          {tip.good}
                        </p>
                      </div>
                    )}
                  </div>
                )}
              </div>
            </div>
          ))}

          <p className="text-[12px] text-secondary/50 leading-relaxed pb-1">
            The employers coming on 19th June are not marking your degree. They
            are looking for someone they want to work with. Give your profile
            fifteen minutes — it will be worth it.
          </p>
        </div>

        {/* Footer */}
        <div className="flex justify-end px-5 py-4 border-t border-secondary/10 flex-shrink-0">
          <button
            type="button"
            onClick={onClose}
            className="w-full sm:w-auto inline-flex justify-center cursor-pointer items-center rounded-xl border border-third/40 bg-third/10 px-6 py-3 text-sm font-semibold text-white transition-all duration-200 hover:bg-third/20 hover:border-third/60 active:scale-[0.98]"
          >
            Got it
          </button>
        </div>
      </div>
    </div>,
    document.body,
  );
}
