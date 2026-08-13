import { useEffect, useState } from "react";
import { api } from "./api";
import type { Task } from "./types";

function pad(n: number) {
  return String(n).padStart(2, "0");
}

function iso(y: number, m: number, d: number) {
  return `${y}-${pad(m + 1)}-${pad(d)}`;
}

function todayIso() {
  const d = new Date();
  return iso(d.getFullYear(), d.getMonth(), d.getDate());
}

export default function Calendar() {
  const now = new Date();
  const [year, setYear] = useState(now.getFullYear());
  const [month, setMonth] = useState(now.getMonth()); // 0-indexed
  const [tasks, setTasks] = useState<Task[]>([]);
  const [error, setError] = useState<string | null>(null);

  const firstOfMonth = new Date(year, month, 1);
  const daysInMonth = new Date(year, month + 1, 0).getDate();
  const startWeekday = firstOfMonth.getDay(); // 0 = Sunday

  useEffect(() => {
    (async () => {
      try {
        const from = iso(year, month, 1);
        const to = iso(year, month, daysInMonth);
        setTasks(await api.tasksInRange(from, to));
        setError(null);
      } catch (e: any) {
        setError(e?.message ?? String(e));
      }
    })();
  }, [year, month]);

  function shiftMonth(delta: number) {
    const d = new Date(year, month + delta, 1);
    setYear(d.getFullYear());
    setMonth(d.getMonth());
  }

  // Build the grid: leading blanks, then each day.
  const cells: (number | null)[] = [];
  for (let i = 0; i < startWeekday; i++) cells.push(null);
  for (let d = 1; d <= daysInMonth; d++) cells.push(d);

  const monthName = firstOfMonth.toLocaleString(undefined, {
    month: "long",
    year: "numeric",
  });

  const today = todayIso();

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
        <button onClick={() => shiftMonth(-1)}>←</button>
        <strong style={{ minWidth: 180, textAlign: "center" }}>
          {monthName}
        </strong>
        <button onClick={() => shiftMonth(1)}>→</button>
        <button
          onClick={() => {
            const d = new Date();
            setYear(d.getFullYear());
            setMonth(d.getMonth());
          }}
        >
          Today
        </button>
      </div>

      {error && <p style={{ color: "#c00" }}>{error}</p>}

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(7, 1fr)",
          gap: 4,
        }}
      >
        {["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"].map((d) => (
          <div
            key={d}
            style={{
              fontSize: 11,
              opacity: 0.6,
              textAlign: "center",
              padding: 4,
            }}
          >
            {d}
          </div>
        ))}

        {cells.map((day, i) => {
          if (day === null) {
            return <div key={`blank-${i}`} />;
          }

          const date = iso(year, month, day);
          const scheduled = tasks.filter((t) => t.scheduled_date === date);
          const due = tasks.filter(
            (t) => t.due_date === date && t.scheduled_date !== date,
          );
          const isToday = date === today;

          return (
            <div
              key={date}
              style={{
                border: isToday ? "2px solid #6af" : "1px solid #444",
                borderRadius: 4,
                minHeight: 84,
                padding: 4,
                fontSize: 11,
                overflow: "hidden",
              }}
            >
              <div style={{ opacity: 0.7, marginBottom: 2 }}>{day}</div>

              {scheduled.map((t) => (
                <div
                  key={t.id}
                  title={`Scheduled: ${t.title}`}
                  style={{
                    background: "#2a4a6a",
                    borderRadius: 3,
                    padding: "1px 3px",
                    marginBottom: 2,
                    whiteSpace: "nowrap",
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    textDecoration:
                      t.status === "COMPLETED" ? "line-through" : "none",
                    opacity: t.status === "COMPLETED" ? 0.5 : 1,
                  }}
                >
                  {t.title}
                </div>
              ))}

              {due.map((t) => (
                <div
                  key={t.id}
                  title={`Due: ${t.title}`}
                  style={{
                    background: "#6a2a2a",
                    borderRadius: 3,
                    padding: "1px 3px",
                    marginBottom: 2,
                    whiteSpace: "nowrap",
                    overflow: "hidden",
                    textOverflow: "ellipsis",
                    opacity: t.status === "COMPLETED" ? 0.5 : 1,
                  }}
                >
                  ! {t.title}
                </div>
              ))}
            </div>
          );
        })}
      </div>

      <div style={{ marginTop: 12, fontSize: 12, opacity: 0.7 }}>
        <span
          style={{ background: "#2a4a6a", padding: "1px 6px", borderRadius: 3 }}
        >
          scheduled
        </span>{" "}
        <span
          style={{ background: "#6a2a2a", padding: "1px 6px", borderRadius: 3 }}
        >
          due
        </span>
      </div>
    </div>
  );
}