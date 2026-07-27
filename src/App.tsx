import { useEffect, useState } from "react";
import { api } from "./api";
import type { Goal, Priority } from "./types";
import "./styles.css";

export default function App() {
  const [goals, setGoals] = useState<Goal[]>([]);
  const [title, setTitle] = useState("");
  const [targetDate, setTargetDate] = useState("");
  const [priority, setPriority] = useState<Priority>("MEDIUM");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  async function refresh() {
    try {
      setGoals(await api.listGoals());
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

  async function handleCreate() {
    try {
      await api.createGoal({
        title,
        description: null,
        start_date: null,
        target_date: targetDate || null,
        priority,
        estimated_effort_minutes: null,
        success_criteria: [],
      });
      setTitle("");
      setTargetDate("");
      await refresh();
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  async function handleDelete(id: string) {
    try {
      await api.deleteGoal(id);
      await refresh();
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  return (
    <main style={{ padding: 24, fontFamily: "system-ui", maxWidth: 720 }}>
      <h1>Stark</h1>

      <section style={{ display: "flex", gap: 8, marginBottom: 24 }}>
        <input
          placeholder="Goal title"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          style={{ flex: 1, padding: 8 }}
        />
        <input
          type="date"
          value={targetDate}
          onChange={(e) => setTargetDate(e.target.value)}
          style={{ padding: 8 }}
        />
        <select
          value={priority}
          onChange={(e) => setPriority(e.target.value as Priority)}
          style={{ padding: 8 }}
        >
          <option>LOW</option>
          <option>MEDIUM</option>
          <option>HIGH</option>
          <option>CRITICAL</option>
        </select>
        <button onClick={handleCreate} style={{ padding: "8px 16px" }}>
          Add
        </button>
      </section>

      {error && (
        <p style={{ color: "#c00", marginBottom: 16 }}>{error}</p>
      )}

      {loading ? (
        <p>Loading…</p>
      ) : goals.length === 0 ? (
        <p style={{ opacity: 0.6 }}>No goals yet.</p>
      ) : (
        <ul style={{ listStyle: "none", padding: 0 }}>
          {goals.map((g) => (
            <li
              key={g.id}
              style={{
                border: "1px solid #444",
                borderRadius: 6,
                padding: 12,
                marginBottom: 8,
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
              }}
            >
              <div>
                <strong>{g.title}</strong>
                <div style={{ fontSize: 13, opacity: 0.7 }}>
                  {g.priority} · {g.status}
                  {g.target_date ? ` · due ${g.target_date}` : ""}
                </div>
              </div>
              <button onClick={() => handleDelete(g.id)}>Delete</button>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}