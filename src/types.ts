export type Priority = "LOW" | "MEDIUM" | "HIGH" | "CRITICAL";
export type Status = "NOT_STARTED" | "IN_PROGRESS" | "COMPLETED" | "CANCELLED";

export interface Goal {
  id: string;
  title: string;
  description: string | null;
  start_date: string | null;
  target_date: string | null;
  priority: Priority;
  status: Status;
  estimated_effort_minutes: number | null;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface NewGoal {
  title: string;
  description: string | null;
  start_date: string | null;
  target_date: string | null;
  priority: Priority;
  estimated_effort_minutes: number | null;
  success_criteria: string[];
}

export interface ErrorPayload {
  kind: "VALIDATION" | "NOT_FOUND" | "STORAGE";
  message: string;
}
export interface Task {
  id: string;
  goal_id: string | null;
  milestone_id: string | null;
  title: string;
  description: string | null;
  due_date: string | null;
  scheduled_date: string | null;
  estimated_minutes: number | null;
  priority: Priority;
  status: Status;
  created_at: string;
  updated_at: string;
  completed_at: string | null;
  deleted_at: string | null;
}

export interface NewTask {
  goal_id: string | null;
  milestone_id: string | null;
  title: string;
  description: string | null;
  due_date: string | null;
  scheduled_date: string | null;
  estimated_minutes: number | null;
  priority: Priority;
}

export interface TaskFilter {
  goal_id?: string | null;
  milestone_id?: string | null;
  scheduled_date?: string | null;
  include_completed: boolean;
}