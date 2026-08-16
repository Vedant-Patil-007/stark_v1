import { useEffect, useState } from "react";
import { api } from "./api";
import { healthColor, healthLabel, fmtMinutes, todayIso } from "./health";
import type { CSSProperties } from "react";
import type { Analysis, Reminder, Task, UpcomingKind } from "./types";const kindLabel: Record<UpcomingKind, string> = {
  TASK_DUE: "task",
  MILESTONE_TARGET: "milestone",
  GOAL_TARGET: "goal",
};

const card: CSSProperties = {
  border: "1px solid #444",
  borderRadius: 8,
  padding: 16,
  marginBottom: 16,
};

export default function Dashboard() {
  const [analysis, setAnalysis] = useState<Analysis | null>(null);
  const [today, setToday] = useState<Task[]>([]);
  const [overdue, setOverdue] = useState<Task[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [missed, setMissed] = useState<Reminder[]>([]);
 

  async function refresh() {
    const date = todayIso();
    // Minutes to add to local time to get UTC. JS returns the inverse sign.
    const offset = new Date().getTimezoneOffset() * -1;
    try {
      await api.syncReminders(date, offset);
      const [a, t, o, m] = await Promise.all([
        api.analyzePlan(date),
        api.todayTasks(date),
        api.overdueTasks(date),
        api.listMissedReminders(),
      ]);
      setAnalysis(a);
      setToday(t);
      setOverdue(o);
      setMissed(m);
      setError(null);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    refresh();
  }, []);

  async function toggle(t: Task) {
    await api.setTaskStatus(
      t.id,
      t.status === "COMPLETED" ? "NOT_STARTED" : "COMPLETED",
    );
    await refresh();
  }

  if (loading) return <p>Loading…</p>;
  if (error) return <p style={{ color: "#c00" }}>{error}</p>;
  if (!analysis) return null;

  const planned = analysis.today_planned_minutes;
  const capacity = analysis.today_capacity_minutes;
  const over = planned > capacity && capacity > 0;

  const atRisk = analysis.goals.filter(
    (g) => g.health === "AT_RISK" || g.health === "BEHIND" || g.health === "CRITICAL",
  );

  return (
    <div>
    {missed.length > 0 && (
        <div style={{ ...card, borderColor: "#8a6d1f" }}>
          <h3 style={{ margin: "0 0 12px" }}>
            Missed while Stark was closed ({missed.length})
          </h3>
          {missed.map((m) => (
            <div
              key={m.id}
              style={{
                display: "flex",
                alignItems: "center",
                gap: 10,
                padding: "4px 0",
                fontSize: 13,
              }}
            >
              <span style={{ flex: 1 }}>{m.title}</span>
              <button
                onClick={async () => {
                  await api.dismissReminder(m.id);
                  await refresh();
                }}
              >
                Dismiss
              </button>
            </div>
          ))}
        </div>
      )}
      {/* ---------- Today ---------- */}
      <div style={card}>
        <div
          style={{
            display: "flex",
            justifyContent: "space-between",
            alignItems: "baseline",
            marginBottom: 12,
          }}
        >
          <h3 style={{ margin: 0 }}>Today</h3>
          <span style={{ fontSize: 13, opacity: 0.8 }}>
            {analysis.today_task_count} task
            {analysis.today_task_count === 1 ? "" : "s"} ·{" "}
            <span style={{ color: over ? "#e88" : "inherit" }}>
              {fmtMinutes(planned)} planned
            </span>{" "}
            / {fmtMinutes(capacity)} available
          </span>
        </div>

        {over && (
          <p style={{ color: "#e88", fontSize: 13, marginTop: 0 }}>
            Today is overcommitted by {fmtMinutes(planned - capacity)}.
          </p>
        )}

        {today.length === 0 ? (
          <p style={{ opacity: 0.6, margin: 0 }}>Nothing scheduled today.</p>
        ) : (
          <ul style={{ listStyle: "none", padding: 0, margin: 0 }}>
            {today.map((t) => (
              <li
                key={t.id}
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 10,
                  padding: "6px 0",
                  borderBottom: "1px solid #333",
                  opacity: t.status === "COMPLETED" ? 0.5 : 1,
                }}
              >
                <input
                  type="checkbox"
                  checked={t.status === "COMPLETED"}
                  onChange={() => toggle(t)}
                />
                <span
                  style={{
                    flex: 1,
                    textDecoration:
                      t.status === "COMPLETED" ? "line-through" : "none",
                  }}
                >
                  {t.title}
                </span>
                <span style={{ fontSize: 12, opacity: 0.7 }}>
                  {t.priority}
                  {t.estimated_minutes ? ` · ${fmtMinutes(t.estimated_minutes)}` : ""}
                </span>
              </li>
            ))}
          </ul>
        )}
      </div>

      {/* ---------- Overdue ---------- */}
      {overdue.length > 0 && (
        <div style={{ ...card, borderColor: "#8a2a2a" }}>
          <h3 style={{ margin: "0 0 12px" }}>
            Overdue ({overdue.length})
          </h3>
          <ul style={{ listStyle: "none", padding: 0, margin: 0 }}>
            {overdue.map((t) => (
              <li
                key={t.id}
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 10,
                  padding: "6px 0",
                  borderBottom: "1px solid #333",
                }}
              >
                <input
                  type="checkbox"
                  checked={false}
                  onChange={() => toggle(t)}
                />
                <span style={{ flex: 1 }}>{t.title}</span>
                <span style={{ fontSize: 12, color: "#e88" }}>
                  due {t.due_date}
                </span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* ---------- Goals ---------- */}
      <div style={card}>
        <h3 style={{ margin: "0 0 12px" }}>Goals</h3>
        {analysis.goals.length === 0 ? (
          <p style={{ opacity: 0.6, margin: 0 }}>No goals yet.</p>
        ) : (
          analysis.goals.map((g) => (
            <div
              key={g.goal_id}
              style={{ padding: "8px 0", borderBottom: "1px solid #333" }}
            >
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 10,
                  marginBottom: 4,
                }}
              >
                <span style={{ flex: 1 }}>{g.title}</span>
                <span style={{ fontSize: 12, opacity: 0.8 }}>
                  {Math.round(g.progress * 100)}%
                </span>
                <span
                  style={{
                    background: healthColor[g.health],
                    borderRadius: 4,
                    padding: "1px 8px",
                    fontSize: 11,
                  }}
                >
                  {healthLabel[g.health]}
                </span>
              </div>

              <div
                style={{
                  height: 4,
                  background: "#333",
                  borderRadius: 2,
                  overflow: "hidden",
                }}
              >
                <div
                  style={{
                    height: "100%",
                    width: `${Math.min(100, Math.round(g.progress * 100))}%`,
                    background: healthColor[g.health],
                  }}
                />
              </div>

              {g.days_remaining !== null && g.health !== "NOT_APPLICABLE" && (
                <div style={{ fontSize: 11, opacity: 0.6, marginTop: 4 }}>
                  {g.days_remaining >= 0
                    ? `${g.days_remaining} days left`
                    : `${-g.days_remaining} days overdue`}
                  {g.confidence === "LOW" &&
                    ` · ⚠ ${g.unestimated_task_count} task(s) unestimated`}
                </div>
              )}
            </div>
          ))
        )}
      </div>

      {/* ---------- Upcoming ---------- */}
      <div style={card}>
        <h3 style={{ margin: "0 0 12px" }}>Upcoming</h3>
        {analysis.upcoming.length === 0 ? (
          <p style={{ opacity: 0.6, margin: 0 }}>
            Nothing due in the next 30 days.
          </p>
        ) : (
          <ul style={{ listStyle: "none", padding: 0, margin: 0 }}>
            {analysis.upcoming.map((u, i) => (
              <li
                key={`${u.date}-${i}`}
                style={{
                  display: "flex",
                  gap: 10,
                  padding: "4px 0",
                  fontSize: 13,
                }}
              >
                <span style={{ width: 90, opacity: 0.7 }}>{u.date}</span>
                <span style={{ flex: 1 }}>{u.label}</span>
                <span style={{ fontSize: 11, opacity: 0.5 }}>
                  {kindLabel[u.kind]}
                </span>
                <span style={{ fontSize: 11, opacity: 0.7, width: 60, textAlign: "right" }}>
                  {u.days_away === 0
                    ? "today"
                    : u.days_away === 1
                      ? "tomorrow"
                      : `${u.days_away}d`}
                </span>
              </li>
            ))}
          </ul>
        )}
      </div>

      {/* ---------- Insight ---------- */}
      {atRisk.length > 0 && (
        <div style={{ ...card, borderColor: "#8a6d1f" }}>
          <h3 style={{ margin: "0 0 12px" }}>Needs attention</h3>
          {atRisk.map((g) => (
            <p key={g.goal_id} style={{ fontSize: 13, margin: "0 0 8px" }}>
              <strong>{g.title}</strong> — {g.reason}
            </p>
          ))}
          <p style={{ fontSize: 12, opacity: 0.6, margin: 0 }}>
            Capacity over the next 7 days:{" "}
            {fmtMinutes(analysis.capacity_next_7_days_minutes)}
          </p>
        </div>
      )}
    </div>
  );
}