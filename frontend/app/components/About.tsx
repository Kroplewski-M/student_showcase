"use client";

import { motion } from "framer-motion";
import Link from "next/link";
import { useRef } from "react";
import Calendar from "../SVGS/Calendar";

const schedule = [
  { time: "12:00 – 13:00", activity: "Setup" },
  { time: "13:00 – 13:30", activity: "Guests arrive" },
  { time: "13:30 – 13:45", activity: "Welcome address" },
  { time: "13:45 – 14:00", activity: "Keynote" },
  { time: "14:00 – 16:00", activity: "Poster presentations & networking" },
  { time: "16:00 – 16:30", activity: "Awards & closing address" },
  { time: "16:30 – 17:00", activity: "Networking" },
];

const highlights = [
  {
    title: "Meet Innovators",
    description:
      "Engage with talented final-year students from Computing and Engineering as they present live project demos and prototypes.",
  },
  {
    title: "Spot Emerging Talent",
    description:
      "Discover students with the skills, creativity, and drive to make an immediate impact in your organisation.",
  },
  {
    title: "Experience R&D",
    description:
      "Leverage the University of Huddersfield's expertise in Engineering, AI, Cyber Security, and Creative Computing.",
  },
  {
    title: "Tour Facilities",
    description:
      "Explore our state-of-the-art labs and workspaces powering the next generation of digital solutions and engineering design.",
  },
];

export default function About() {
  const sectionRef = useRef<HTMLDivElement>(null);

  return (
    <motion.section
      ref={sectionRef}
      id="about"
      className="relative z-20  bg-primary shadow-[0_-40px_80px_rgba(0,0,0,0.5)] min-h-screen"
    >
      <div className="absolute inset-x-0 top-0 h-px bg-gradient-to-r from-transparent via-secondary/40 to-transparent" />

      <div className="pointer-events-none absolute inset-0 overflow-hidden">
        {/* Glow orbs */}
        <div className="absolute -right-[15%] top-[10%] h-[40vw] w-[40vw] rounded-full bg-secondary/[0.03] blur-3xl" />
        <div className="absolute -left-[10%] bottom-[10%] h-[30vw] w-[30vw] rounded-full bg-third/[0.05] blur-3xl" />
      </div>

      <div className="relative z-10 mx-auto w-full max-w-7xl px-4 py-24 sm:px-8 sm:py-32">
        {/* ── Section header ── */}
        <motion.div
          initial={{ opacity: 0, y: 30 }}
          whileInView={{ opacity: 1, y: 0 }}
          viewport={{ once: true, margin: "-100px" }}
          transition={{ duration: 0.8, ease: [0.16, 1, 0.3, 1] }}
        >
          <span className="inline-flex items-center gap-2 rounded-full border border-secondary/20 bg-secondary/5 px-4 py-1.5 text-[11px] font-semibold uppercase tracking-[0.2em] text-secondary backdrop-blur-sm">
            About the Event
          </span>

          <div className="mt-6 flex flex-col gap-6 sm:flex-row sm:items-end sm:gap-10">
            <h2 className="text-[clamp(2rem,5vw,4.5rem)] font-extrabold leading-[0.9] tracking-tighter text-light">
              Where talent
              <br />
              meets <span className="text-secondary">opportunity</span>
            </h2>

            {/* Calendar date card */}
            <div className="flex shrink-0 items-center gap-3 rounded-2xl border border-third/20 bg-third/[0.04] px-5 py-4 backdrop-blur-sm">
              <Calendar date={19} />
              <div className="flex flex-col">
                <span className="text-[10px] font-semibold uppercase tracking-[0.15em] text-secondary">
                  Friday
                </span>
                <span className="text-lg font-extrabold leading-tight tracking-tight text-light">
                  19th June
                </span>
                <span className="text-[11px] font-medium text-support/40">
                  2026
                </span>
              </div>
            </div>
          </div>

          <p className="mt-6 max-w-2xl text-sm leading-relaxed text-support/60 sm:text-base">
            The University of Huddersfield&rsquo;s School of Computing &amp;
            Engineering invites you to SCE Futures 2026 — a student showcase
            connecting the next generation of innovators with industry leaders.
          </p>
        </motion.div>

        {/* ── Highlights grid ── */}
        <div className="mt-16 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
          {highlights.map((item, i) => (
            <motion.div
              key={item.title}
              initial={{ opacity: 0, y: 30 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true, margin: "-60px" }}
              transition={{
                duration: 0.7,
                delay: i * 0.1,
                ease: [0.16, 1, 0.3, 1],
              }}
              className="group relative overflow-hidden rounded-2xl border border-third/20 bg-third/[0.04] p-6 backdrop-blur-sm transition-colors duration-300 hover:border-secondary/30 hover:bg-secondary/[0.04]"
            >
              {/* Card number accent */}
              <span className="text-[10px] font-bold uppercase tracking-[0.2em] text-secondary/50">
                0{i + 1}
              </span>
              <h3 className="mt-3 text-lg font-bold tracking-tight text-light">
                {item.title}
              </h3>
              <p className="mt-2 text-sm leading-relaxed text-support/50">
                {item.description}
              </p>

              {/* Hover glow */}
              <div className="pointer-events-none absolute -right-8 -top-8 h-24 w-24 rounded-full bg-secondary/10 opacity-0 blur-2xl transition-opacity duration-500 group-hover:opacity-100" />
            </motion.div>
          ))}
        </div>

        {/* ── Why attend + Schedule ── */}
        <div className="mt-20 grid gap-12 lg:grid-cols-[1fr_1fr] lg:gap-16">
          {/* Why attend */}
          <motion.div
            initial={{ opacity: 0, y: 30 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true, margin: "-80px" }}
            transition={{ duration: 0.8, ease: [0.16, 1, 0.3, 1] }}
          >
            <h3 className="text-2xl font-extrabold tracking-tight text-light sm:text-3xl">
              Why attend<span className="text-secondary">?</span>
            </h3>
            <div className="mt-6 space-y-4 text-sm leading-relaxed text-support/60 sm:text-base">
              <p>
                Attending our annual Showcase is more than just an opportunity
                to meet talented students — it&rsquo;s your gateway to
                leveraging the full R&amp;D potential of the University of
                Huddersfield.
              </p>
              <p>
                As a leading institution with established expertise in
                Engineering, AI, Cyber Security, and Creative Computing, we are
                uniquely positioned to help commercial partners accelerate
                innovation and solve real-world challenges.
              </p>
            </div>

            {/* Decorative separator */}
            <div className="mt-8 flex items-center gap-3">
              <div className="h-px flex-1 bg-gradient-to-r from-secondary/30 to-transparent" />
              <span className="text-[10px] font-semibold uppercase tracking-[0.2em] text-secondary/40">
                Hands-on demos &bull; Live prototypes &bull; Real-world research
              </span>
            </div>
          </motion.div>

          {/* Schedule */}
          <motion.div
            initial={{ opacity: 0, y: 30 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true, margin: "-80px" }}
            transition={{
              duration: 0.8,
              delay: 0.15,
              ease: [0.16, 1, 0.3, 1],
            }}
          >
            <h3 className="text-2xl font-extrabold tracking-tight text-light sm:text-3xl">
              Schedule
            </h3>

            <div className="mt-6 overflow-hidden rounded-2xl border border-third/20 bg-third/[0.04] backdrop-blur-sm">
              {schedule.map((slot, i) => (
                <div
                  key={slot.time}
                  className={`flex items-center gap-4 px-5 py-3.5 transition-colors hover:bg-secondary/[0.04] ${
                    i !== schedule.length - 1 ? "border-b border-third/10" : ""
                  }`}
                >
                  <span className="w-[7.5rem] shrink-0 text-xs font-semibold tracking-wide text-secondary/70 sm:text-sm">
                    {slot.time}
                  </span>
                  <span className="text-sm text-support/70">
                    {slot.activity}
                  </span>
                </div>
              ))}
            </div>
          </motion.div>
        </div>
      </div>
      <div className="w-[200px] mx-auto">
        <Link
          href="https://forms.office.com/pages/responsepage.aspx?id=2p8utZEGhUW9_FzK4c4YkG46iA9GSCpPssptatZjoWZUNlhVWTJMNDlJWjlKT09CNlhMMTJMUjJRVC4u&route=shorturl"
          target="_blank"
          rel="noopener noreferrer"
          className="rounded-xl border border-third/40 bg-third/10 px-8 py-3.5 text-center text-sm font-semibold text-light backdrop-blur-sm transition-all hover:border-third/60 hover:bg-third/20 active:scale-[0.985] cursor-pointer"
        >
          Register Interest
        </Link>
      </div>

      {/* Bottom corner accents */}
      <div className="pointer-events-none">
        <div className="absolute bottom-8 right-4 h-16 w-16 border-b border-r border-third/15 sm:right-8 sm:h-20 sm:w-20" />
      </div>
    </motion.section>
  );
}
