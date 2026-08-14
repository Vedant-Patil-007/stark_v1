import type { GoalHealth } from "./types";

export const healthColor: Record<GoalHealth, string> = {
  ON_TRACK: "#2d6a4f",
  AT_RISK: "#8a6d1f",
  BEHIND: "#8a4a1f",
  CRITICAL: "#8a2a2a",
  NOT_APPLICABLE: "#444",
};

export const healthLabel: Record<GoalHealth, string> = {
  ON_TRACK: "ON TRACK",
  AT_RISK: "AT RISK",
  BEHIND: "BEHIND",
  CRITICAL: "CRITICAL",
  NOT_APPLICABLE: "—",
};

export function fmtMinutes(m: number): string {
  const sign = m < 0 ? "-" : "";
  const abs = Math.abs(m);
  const h = Math.floor(abs / 60);
  const min = abs % 60;
  if (h === 0) return `${sign}${min}m`;
  if (min === 0) return `${sign}${h}h`;
  return `${sign}${h}h ${min}m`;
}

export function todayIso(): string {
  const d = new Date();
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}