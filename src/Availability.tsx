import { useEffect, useState } from "react";
import { api } from "./api";
import type { AvailabilityWindow, DayCapacity } from "./types";

const DAYS = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

function toMinutes(hhmm: string): number | null {
  const m = /^(\d{1,2}):(\d{2})$/.exec(hhmm);
  if (!m) return null;
  const h = parseInt(m[1], 10);
  const min = parseInt(m[2], 10);
  if (h > 24 || min > 59) return null;
  return h * 60 + min;
}

function fromMinutes(mins: number): string {
  const h = Math.floor(mins / 60);
  const m = mins % 60;
  return `${String(h).padStart(2, "0")}:${String(m).padStart(2, "0")}`;
}

function todayIso() {
  const d = new Date();
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}

export default function Availability() {
  const [windows, setWindows] = useState<AvailabilityWindow[]>([]);
  const [weekday, setWeekday] = useState(1);
  const [start, setStart] = useState("09:00");
  const [end, setEnd] = useState("17:00");
  const [error, setError] = useState<string | null>(null);

  const [checkDate, setCheckDate] = useState(todayIso());
  const [capacity, setCapacity] = useState<DayCapacity | null>(null);

  async function refresh() {
    try {
      setWindows(await api.listAvailabilityWindows());
      setError(null);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  useEffect(() => {
    refresh();
  }, []);

  async function checkCapacity() {
    try {
      const wd = new Date(checkDate + "T12:00:00").getDay();
      setCapacity(await api.capacityForDate(checkDate, wd));
      setError(null);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  useEffect(() => {
    checkCapacity();
  }, [checkDate, windows]);

  async function handleAdd() {
    const s = toMinutes(start);
    const e = toMinutes(end);
    if (s === null || e === null) {
      setError("Times must be in HH:MM format");
      return;
    }
    try {
      await api.createAvailabilityWindow({
        weekday,
        start_minute: s,
        end_minute: e,
        label: null,
      });
      await refresh();
    } catch (err: any) {
      setError(err?.message ?? String(err));
    }
  }

  const byDay = DAYS.map((name, i) => ({
    name,
    index: i,
    windows: windows.filter((w) => w.weekday === i),
  }));

  const weeklyTotal = windows.reduce(
    (sum, w) => sum + (w.end_minute - w.start_minute),
    0,
  );

  return (
    <div>
      <h3>Working hours</h3>
      <p style={{ fontSize: 13, opacity: 0.7 }}>
        When are you available to work? The planner uses this to calculate
        capacity.
      </p>

      <div style={{ display: "flex", gap: 8, marginBottom: 16, flexWrap: "wrap" }}>
        <select
          value={weekday}
          onChange={(e) => setWeekday(parseInt(e.target.value, 10))}
          style={{ padding: 8 }}
        >
          {DAYS.map((d, i) => (
            <option key={d} value={i}>
              {d}
            </option>
          ))}
        </select>
        <input
          value={start}
          onChange={(e) => setStart(e.target.value)}
          placeholder="09:00"
          style={{ width: 80, padding: 8 }}
        />
        <span style={{ alignSelf: "center" }}>to</span>
        <input
          value={end}
          onChange={(e) => setEnd(e.target.value)}
          placeholder="17:00"
          style={{ width: 80, padding: 8 }}
        />
        <button onClick={handleAdd} style={{ padding: "8px 16px" }}>
          Add
        </button>
      </div>

      {error && <p style={{ color: "#c00" }}>{error}</p>}

      <div style={{ marginBottom: 24 }}>
        {byDay.map((d) => (
          <div
            key={d.index}
            style={{
              display: "flex",
              alignItems: "center",
              gap: 12,
              padding: "6px 0",
              borderBottom: "1px solid #333",
            }}
          >
            <div style={{ width: 100, opacity: d.windows.length ? 1 : 0.4 }}>
              {d.name}
            </div>
            <div style={{ flex: 1, display: "flex", gap: 6, flexWrap: "wrap" }}>
              {d.windows.length === 0 ? (
                <span style={{ opacity: 0.4, fontSize: 13 }}>unavailable</span>
              ) : (
                d.windows.map((w) => (
                  <span
                    key={w.id}
                    style={{
                      background: "#2a4a6a",
                      borderRadius: 4,
                      padding: "2px 8px",
                      fontSize: 13,
                      display: "flex",
                      gap: 6,
                      alignItems: "center",
                    }}
                  >
                    {fromMinutes(w.start_minute)}–{fromMinutes(w.end_minute)}
                    <button
                      onClick={async () => {
                        await api.deleteAvailabilityWindow(w.id);
                        await refresh();
                      }}
                      style={{
                        background: "none",
                        border: "none",
                        color: "inherit",
                        cursor: "pointer",
                        padding: 0,
                      }}
                    >
                      ×
                    </button>
                  </span>
                ))
              )}
            </div>
          </div>
        ))}
      </div>

      <div style={{ fontSize: 13, opacity: 0.8, marginBottom: 24 }}>
        Weekly capacity: {Math.floor(weeklyTotal / 60)}h {weeklyTotal % 60}m
      </div>

      <h3>Check a day</h3>
      <div style={{ display: "flex", gap: 8, alignItems: "center" }}>
        <input
          type="date"
          value={checkDate}
          onChange={(e) => setCheckDate(e.target.value)}
          style={{ padding: 8 }}
        />
        {capacity && (
          <span style={{ opacity: 0.8 }}>
            {Math.floor(capacity.total_minutes / 60)}h{" "}
            {capacity.total_minutes % 60}m available
            {capacity.windows.length > 0 &&
              " · " +
                capacity.windows
                  .map((w) => `${fromMinutes(w.start)}–${fromMinutes(w.end)}`)
                  .join(", ")}
          </span>
        )}
      </div>
    </div>
  );
}