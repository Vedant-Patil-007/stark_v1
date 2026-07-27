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