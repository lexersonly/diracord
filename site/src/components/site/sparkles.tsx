"use client";

import { motion, useReducedMotion } from "motion/react";
import { useId } from "react";
import { cn } from "@/lib/utils";

/** Four-point star path in a 100×100 box. */
const STAR = "M50 0 C53 34 66 47 100 50 C66 53 53 66 50 100 C47 66 34 53 0 50 C34 47 47 34 50 0 Z";

/** One chrome star: dark-to-white gradient body, bright rim and a soft glow. */
function ChromeStar({ size, className, delay = 0 }: { size: number; className?: string; delay?: number }) {
  const id = useId().replace(/:/g, "");
  const reduce = useReducedMotion();
  return (
    <motion.svg
      viewBox="-10 -10 120 120"
      width={size}
      height={size}
      className={cn("absolute drop-shadow-[0_0_40px_rgba(255,255,255,0.18)]", className)}
      initial={{ opacity: 0, scale: 0.6, rotate: -12 }}
      animate={
        reduce
          ? { opacity: 1, scale: 1, rotate: 0 }
          : { opacity: 1, scale: 1, rotate: 0, y: [0, -8, 0] }
      }
      transition={{
        opacity: { duration: 0.8, delay },
        scale: { type: "spring", stiffness: 120, damping: 14, delay },
        rotate: { type: "spring", stiffness: 80, damping: 12, delay },
        y: { duration: 6, repeat: Infinity, ease: "easeInOut", delay: delay + 0.8 },
      }}
      aria-hidden
    >
      <defs>
        <linearGradient id={`body-${id}`} x1="0.15" y1="0" x2="0.85" y2="1">
          <stop offset="0%" stopColor="#ffffff" />
          <stop offset="28%" stopColor="#bdbdbd" />
          <stop offset="52%" stopColor="#3a3a3a" />
          <stop offset="70%" stopColor="#e6e6e6" />
          <stop offset="100%" stopColor="#262626" />
        </linearGradient>
        <linearGradient id={`rim-${id}`} x1="0" y1="0" x2="1" y2="1">
          <stop offset="0%" stopColor="#ffffff" stopOpacity="0.95" />
          <stop offset="50%" stopColor="#ffffff" stopOpacity="0.15" />
          <stop offset="100%" stopColor="#ffffff" stopOpacity="0.7" />
        </linearGradient>
        <radialGradient id={`shine-${id}`} cx="0.38" cy="0.32" r="0.5">
          <stop offset="0%" stopColor="#ffffff" stopOpacity="0.9" />
          <stop offset="100%" stopColor="#ffffff" stopOpacity="0" />
        </radialGradient>
      </defs>
      <path d={STAR} fill={`url(#body-${id})`} />
      <path d={STAR} fill={`url(#shine-${id})`} opacity="0.55" />
      <path d={STAR} fill="none" stroke={`url(#rim-${id})`} strokeWidth="1.6" />
    </motion.svg>
  );
}

/** The three stars of the hero, like the reference design. */
export function Sparkles({ className }: { className?: string }) {
  return (
    <div className={cn("pointer-events-none relative h-[260px] w-[320px]", className)}>
      <ChromeStar size={200} className="left-[70px] top-[30px]" />
      <ChromeStar size={110} className="left-0 top-0" delay={0.2} />
      <ChromeStar size={64} className="right-0 bottom-0" delay={0.4} />
    </div>
  );
}
