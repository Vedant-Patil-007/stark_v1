import { useEffect, useState } from "react";
import { api } from "./api";
import type { Priority, Task } from "./types";

export default function TaskList({
  goalId,
  milestoneId,
}: {
  goalId?: string | null;
  milestoneId?: string | null;
}) {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [title, setTitle] = useState("");
  const [dueDate, setDueDate] = useState("");
  const [scheduledDate, setScheduledDate] = useState("");
  const [minutes, setMinutes] = useState("");
  const [priority, setPriority] = useState<Priority>("MEDIUM");
  const [showDone, setShowDone] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    try {
      setTasks(
        await api.listTasks({
          goal_id: goalId ?? null,
          milestone_id: milestoneId ?? null,
          scheduled_date: null,
          include_completed: showDone,
        }),
      );
      setError(null);
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  useEffect(() => {
    refresh();
  }, [goalId, milestoneId, showDone]);

  async function handleAdd() {
    try {
      await api.createTask({
        goal_id: goalId ?? null,
        milestone_id: milestoneId ?? null,
        title,
        description: null,
        due_date: dueDate || null,
        scheduled_date: scheduledDate || null,
        estimated_minutes: minutes ? parseInt(minutes, 10) : null,
        priority,
      });
      setTitle("");
      setDueDate("");
      setScheduledDate("");
      setMinutes("");
      await refresh();
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  async function toggleDone(t: Task) {
    try {
      await api.setTaskStatus(
        t.id,
        t.status === "COMPLETED" ? "NOT_STARTED" : "COMPLETED",
      );
      await refresh();
    } catch (e: any) {
      setError(e?.message ?? String(e));
    }
  }

  return (
    <div>
      <div style={{ display: "flex", gap: 8, marginBottom: 8, flexWrap: "wrap" }}>
        <input
          placeholder="Task title"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          style={{ flex: "1 1 200px", padding: 8 }}
        />
        <label style={{ fontSize: 12, opacity: 0.7 }}>
          due
          <input
            type="date"
            value={dueDate}
            onChange={(e) => setDueDate(e.target.value)}
            style={{ padding: 6, marginLeft: 4 }}
          />
        </label>
        <label style={{ fontSize: 12, opacity: 0.7 }}>
          do on
          <input
            type="date"
            value={scheduledDate}
            onChange={(e) => setScheduledDate(e.target.value)}
            style={{ padding: 6, marginLeft: 4 }}
          />
        </label>
        <input
          placeholder="min"
          value={minutes}
          onChange={(e) => setMinutes(e.target.value)}
          style={{ width: 60, padding: 8 }}
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
        <button onClick={handleAdd} style={{ padding: "8px 16px" }}>
          Add
        </button>
      </div>

      <label style={{ fontSize: 12, opacity: 0.7 }}>
        <input
          type="checkbox"
          checked={showDone}
          onChange={(e) => setShowDone(e.target.checked)}
        />{" "}
        show completed
      </label>

      {error && <p style={{ color: "#c00" }}>{error}</p>}

      {tasks.length === 0 ? (
        <p style={{ opacity: 0.6 }}>No tasks.</p>
      ) : (
        <ul style={{ listStyle: "none", padding: 0, marginTop: 12 }}>
          {tasks.map((t) => (
            <li
              key={t.id}
              style={{
                border: "1px solid #444",
                borderRadius: 6,
                padding: 10,
                marginBottom: 6,
                display: "flex",
                alignItems: "center",
                gap: 10,
                opacity: t.status === "COMPLETED" ? 0.5 : 1,
              }}
            >
              <input
                type="checkbox"
                checked={t.status === "COMPLETED"}
                onChange={() => toggleDone(t)}
              />
              <div style={{ flex: 1 }}>
                <div
                  style={{
                    textDecoration:
                      t.status === "COMPLETED" ? "line-through" : "none",
                  }}
                >
                  {t.title}
                </div>
                <div style={{ fontSize: 12, opacity: 0.7 }}>
                  {t.priority}
                  {t.scheduled_date ? ` · do ${t.scheduled_date}` : ""}
                  {t.due_date ? ` · due ${t.due_date}` : ""}
                  {t.estimated_minutes ? ` · ${t.estimated_minutes}m` : ""}
                </div>
              </div>
              <button
                onClick={async () => {
                  await api.deleteTask(t.id);
                  await refresh();
                }}
              >
                Delete
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}