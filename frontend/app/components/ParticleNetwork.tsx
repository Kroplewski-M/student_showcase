"use client";

import { useEffect, useRef } from "react";
import { motion, useScroll, useTransform } from "framer-motion";

// ── Particle Network Canvas ──
function ParticleNetwork() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    let animationId: number;
    let particles: {
      x: number;
      y: number;
      vx: number;
      vy: number;
      r: number;
    }[] = [];

    const PARTICLE_COUNT = 60;
    const CONNECTION_DIST = 150;
    const ACCENT = { r: 161, g: 233, b: 240 }; // secondary colour

    function resize() {
      if (!canvas) return;
      canvas.width = canvas.offsetWidth * window.devicePixelRatio;
      canvas.height = canvas.offsetHeight * window.devicePixelRatio;
      ctx!.scale(window.devicePixelRatio, window.devicePixelRatio);
    }

    function init() {
      resize();
      particles = Array.from({ length: PARTICLE_COUNT }, () => ({
        x: Math.random() * canvas!.offsetWidth,
        y: Math.random() * canvas!.offsetHeight,
        vx: (Math.random() - 0.5) * 0.4,
        vy: (Math.random() - 0.5) * 0.4,
        r: Math.random() * 1.5 + 0.5,
      }));
    }

    function draw() {
      if (!canvas || !ctx) return;
      const w = canvas.offsetWidth;
      const h = canvas.offsetHeight;

      ctx.clearRect(0, 0, w, h);

      // Update positions
      for (const p of particles) {
        p.x += p.vx;
        p.y += p.vy;
        if (p.x < 0 || p.x > w) p.vx *= -1;
        if (p.y < 0 || p.y > h) p.vy *= -1;
      }

      // Draw connections
      for (let i = 0; i < particles.length; i++) {
        for (let j = i + 1; j < particles.length; j++) {
          const dx = particles[i].x - particles[j].x;
          const dy = particles[i].y - particles[j].y;
          const dist = Math.sqrt(dx * dx + dy * dy);
          if (dist < CONNECTION_DIST) {
            const alpha = (1 - dist / CONNECTION_DIST) * 0.15;
            ctx.beginPath();
            ctx.moveTo(particles[i].x, particles[i].y);
            ctx.lineTo(particles[j].x, particles[j].y);
            ctx.strokeStyle = `rgba(${ACCENT.r},${ACCENT.g},${ACCENT.b},${alpha})`;
            ctx.lineWidth = 0.5;
            ctx.stroke();
          }
        }
      }

      // Draw particles
      for (const p of particles) {
        ctx.beginPath();
        ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
        ctx.fillStyle = `rgba(${ACCENT.r},${ACCENT.g},${ACCENT.b},0.4)`;
        ctx.fill();
      }

      animationId = requestAnimationFrame(draw);
    }

    init();
    draw();
    window.addEventListener("resize", resize);

    return () => {
      cancelAnimationFrame(animationId);
      window.removeEventListener("resize", resize);
    };
  }, []);

  return <canvas ref={canvasRef} className="absolute inset-0 h-full w-full" />;
}

// ── Floating Card Data ──
const CARDS = [
  {
    title: "AI Image Classifier",
    tags: ["Python", "TensorFlow"],
    color: "from-secondary/20 to-secondary/5",
    borderColor: "border-secondary/20",
    x: "8%",
    y: "15%",
    rotation: -3,
    delay: 0.2,
  },
  {
    title: "E-Commerce Platform",
    tags: ["React", "Node.js"],
    color: "from-support/15 to-support/5",
    borderColor: "border-support/20",
    x: "62%",
    y: "8%",
    rotation: 2,
    delay: 0.5,
  },
  {
    title: "Portfolio Redesign",
    tags: ["Figma", "Next.js"],
    color: "from-secondary/15 to-third/10",
    borderColor: "border-third/25",
    x: "35%",
    y: "55%",
    rotation: -1.5,
    delay: 0.8,
  },
  {
    title: "IoT Dashboard",
    tags: ["Rust", "WebSocket"],
    color: "from-third/20 to-third/5",
    borderColor: "border-third/20",
    x: "78%",
    y: "48%",
    rotation: 3,
    delay: 0.4,
  },
  {
    title: "Mobile Fitness App",
    tags: ["Swift", "HealthKit"],
    color: "from-support/10 to-secondary/5",
    borderColor: "border-secondary/15",
    x: "18%",
    y: "65%",
    rotation: 1.5,
    delay: 0.7,
  },
  {
    title: "Cloud DevOps Pipeline",
    tags: ["Docker", "Kubernetes"],
    color: "from-third/15 to-secondary/5",
    borderColor: "border-secondary/20",
    x: "88%",
    y: "25%",
    rotation: -2,
    delay: 0.3,
  },
  {
    title: "Blockchain Wallet",
    tags: ["Solidity", "Ethers.js"],
    color: "from-secondary/15 to-support/5",
    borderColor: "border-support/15",
    x: "48%",
    y: "30%",
    rotation: 1,
    delay: 0.6,
  },
  {
    title: "Game Physics Engine",
    tags: ["C++", "OpenGL"],
    color: "from-support/15 to-third/10",
    borderColor: "border-third/20",
    x: "3%",
    y: "42%",
    rotation: -2.5,
    delay: 0.9,
  },
  {
    title: "Chat App",
    tags: ["Go", "gRPC"],
    color: "from-third/20 to-support/5",
    borderColor: "border-support/20",
    x: "55%",
    y: "68%",
    rotation: 2.5,
    delay: 0.35,
  },
  {
    title: "Compiler Design",
    tags: ["Rust", "LLVM"],
    color: "from-secondary/20 to-third/5",
    borderColor: "border-third/25",
    x: "82%",
    y: "70%",
    rotation: -1,
    delay: 0.55,
  },
];

// ── Main Component ──
export default function HeroVisuals() {
  const { scrollY } = useScroll();
  const yCards = useTransform(scrollY, [0, 600], [0, -60]);
  const yNetwork = useTransform(scrollY, [0, 600], [0, -30]);
  const opacity = useTransform(scrollY, [100, 500], [1, 0]);

  return (
    <motion.div
      style={{ opacity }}
      className="relative  h-[50vh] w-full sm:-mt-24 sm:h-[55vh]"
    >
      {/* Particle network background */}
      <motion.div style={{ y: yNetwork }} className="absolute inset-0">
        <ParticleNetwork />
      </motion.div>

      {/* Floating cards */}
      <motion.div style={{ y: yCards }} className="absolute inset-0">
        {CARDS.map((card, i) => (
          <motion.div
            key={card.title}
            initial={{ opacity: 0, y: 30, scale: 0.9 }}
            animate={{ opacity: 1, y: 0, scale: 1.3 }}
            transition={{
              duration: 0.8,
              delay: 0.8 + card.delay,
              ease: [0.16, 1, 0.3, 1],
            }}
            className="absolute"
            style={{
              left: card.x,
              top: card.y,
            }}
          >
            <motion.div
              animate={{
                y: [0, i % 2 === 0 ? -8 : 8, 0],
                rotate: [
                  card.rotation,
                  card.rotation + (i % 2 === 0 ? 1 : -1),
                  card.rotation,
                ],
              }}
              transition={{
                duration: 4 + i * 0.5,
                repeat: Infinity,
                ease: "easeInOut",
              }}
              className={`rounded-xl border  backdrop-blur-md ${card.color} ${card.borderColor} px-4 py-3 shadow-lg shadow-primary/20 sm:px-5 sm:py-4`}
              style={{ rotate: card.rotation }}
            >
              <p className="text-xs font-bold text-light sm:text-sm">
                {card.title}
              </p>
              <div className="mt-1.5 flex gap-1.5">
                {card.tags.map((tag) => (
                  <span
                    key={tag}
                    className="rounded-full bg-primary/40 px-2 py-0.5 text-[9px] font-semibold text-support/70 sm:text-[10px]"
                  >
                    {tag}
                  </span>
                ))}
              </div>
            </motion.div>
          </motion.div>
        ))}
      </motion.div>
    </motion.div>
  );
}
