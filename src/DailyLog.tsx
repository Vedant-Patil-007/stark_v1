import { useEffect, useState } from "react";
import { api } from "./api";
import type { Goal, LogEntry } from "./types";

function today(): string {
  const d = new Date();
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}

function shiftDate(date: string, days: number): string {
  const d = new Date(date + "T12:00:00");
  d.setDate(d.getDate() + days);
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}

export default function DailyLog({ goals }: { goals: Goal[] }) {
  const [date, setDate] = useState(today());
  const [entries, setEntries] = useState<LogEntry[]>([]);
  const [activity, setActivity] = useState("");
  const [minutes, setMinutes] = useState("");
  const [goalId, setGoalId] = useState("");
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    try {
      setEntries(await api.listLogForDate(date));
      setError(null);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  useEffect(() => {
    refresh();
  }, [date]);

  async function handleAdd() {
    try {
      await api.createLogEntry({
        log_date: date,
        task_id: null,
        milestone_id: null,
        goal_id: goalId || null,
        activity,
        duration_minutes: minutes ? parseInt(minutes, 10) : null,
        category: null,
        notes: null,
      });
      setActivity("");
      setMinutes("");
      await refresh();
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  const totalMinutes = entries.reduce(
    (sum, e) => sum + (e.duration_minutes ?? 0),
    0,
  );
  const hours = Math.floor(totalMinutes / 60);
  const mins = totalMinutes % 60;

  return (
    <div>
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 12,
          marginBottom: 16,
        }}
      >
        <button onClick={() => setDate(shiftDate(date, -1))}>←</button>
        <input
          type="date"
          value={date}
          onChange={(e) => setDate(e.target.value)}
          style={{ padding: 8 }}
        />
        <button onClick={() => setDate(shiftDate(date, 1))}>→</button>
        <button onClick={() => setDate(today())}>Today</button>
        <span style={{ marginLeft: "auto", opacity: 0.8 }}>
          {hours}h {mins}m logged
        </span>
      </div>

      <div style={{ display: "flex", gap: 8, marginBottom: 16, flexWrap: "wrap" }}>
        <input
          placeholder="What did you do?"
          value={activity}
          onChange={(e) => setActivity(e.target.value)}
          style={{ flex: "1 1 240px", padding: 8 }}
        />
        <input
          placeholder="min"
          value={minutes}
          onChange={(e) => setMinutes(e.target.value)}
          style={{ width: 70, padding: 8 }}
        />
        <select
          value={goalId}
          onChange={(e) => setGoalId(e.target.value)}
          style={{ padding: 8 }}
        >
          <option value="">No goal</option>
          {goals.map((g) => (
            <option key={g.id} value={g.id}>
              {g.title}
            </option>
          ))}
        </select>
        <button onClick={handleAdd} style={{ padding: "8px 16px" }}>
          Log
        </button>
      </div>

      {error && <p style={{ color: "#c00" }}>{error}</p>}

      {entries.length === 0 ? (
        <p style={{ opacity: 0.6 }}>Nothing logged for this day.</p>
      ) : (
        <ul style={{ listStyle: "none", padding: 0 }}>
          {entries.map((e) => {
            const goal = goals.find((g) => g.id === e.goal_id);
            return (
              <li
                key={e.id}
                style={{
                  border: "1px solid #444",
                  borderRadius: 6,
                  padding: 10,
                  marginBottom: 6,
                  display: "flex",
                  alignItems: "center",
                  gap: 10,
                }}
              >
                <div style={{ flex: 1 }}>
                  <div>{e.activity}</div>
                  <div style={{ fontSize: 12, opacity: 0.7 }}>
                    {e.duration_minutes ? `${e.duration_minutes}m` : "no duration"}
                    {goal ? ` · ${goal.title}` : ""}
                  </div>
                </div>
                <button
                  onClick={async () => {
                    await api.deleteLogEntry(e.id);
                    await refresh();
                  }}
                >
                  Delete
                </button>
              </li>
            );
          })}
        </ul>
      )}
    </div>
  );
}