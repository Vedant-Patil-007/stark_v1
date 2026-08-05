import { useEffect, useState } from "react";
import { api } from "./api";
import type { Goal, Milestone, Status } from "./types";

export default function GoalDetail({
  goal,
  onBack,
}: {
  goal: Goal;
  onBack: () => void;
}) {
  const [milestones, setMilestones] = useState<Milestone[]>([]);
  const [title, setTitle] = useState("");
  const [targetDate, setTargetDate] = useState("");
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    try {
      setMilestones(await api.listMilestones(goal.id));
      setError(null);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  useEffect(() => {
    refresh();
  }, [goal.id]);

  async function handleAdd() {
    try {
      await api.createMilestone({
        goal_id: goal.id,
        title,
        description: null,
        target_date: targetDate || null,
      });
      setTitle("");
      setTargetDate("");
      await refresh();
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  async function cycleStatus(m: Milestone) {
    const next: Record<Status, Status> = {
      NOT_STARTED: "IN_PROGRESS",
      IN_PROGRESS: "COMPLETED",
      COMPLETED: "NOT_STARTED",
      CANCELLED: "NOT_STARTED",
    };
    try {
      await api.setMilestoneStatus(m.id, next[m.status]);
      await refresh();
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  async function handleDelete(id: string) {
    try {
      await api.deleteMilestone(id);
      await refresh();
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  return (
    <div>
      <button onClick={onBack} style={{ marginBottom: 16 }}>
        ← Back
      </button>

      <h2 style={{ marginBottom: 4 }}>{goal.title}</h2>
      <div style={{ fontSize: 13, opacity: 0.7, marginBottom: 24 }}>
        {goal.priority} · {goal.status}
        {goal.target_date ? ` · due ${goal.target_date}` : ""}
      </div>

      <h3>Milestones</h3>

      <div style={{ display: "flex", gap: 8, marginBottom: 16 }}>
        <input
          placeholder="Milestone title"
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
        <button onClick={handleAdd} style={{ padding: "8px 16px" }}>
          Add
        </button>
      </div>

      {error && <p style={{ color: "#c00" }}>{error}</p>}

      {milestones.length === 0 ? (
        <p style={{ opacity: 0.6 }}>No milestones yet.</p>
      ) : (
        <ul style={{ listStyle: "none", padding: 0 }}>
          {milestones.map((m) => (
            <li
              key={m.id}
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
                <strong>{m.title}</strong>
                <div style={{ fontSize: 13, opacity: 0.7 }}>
                  {m.status}
                  {m.target_date ? ` · ${m.target_date}` : ""}
                </div>
              </div>
              <div style={{ display: "flex", gap: 8 }}>
                <button onClick={() => cycleStatus(m)}>Status</button>
                <button onClick={() => handleDelete(m.id)}>Delete</button>
              </div>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}