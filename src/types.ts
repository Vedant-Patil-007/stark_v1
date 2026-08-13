export type Priority = "LOW" | "MEDIUM" | "HIGH" | "CRITICAL";
export type Status = "NOT_STARTED" | "IN_PROGRESS" | "COMPLETED" | "CANCELLED";

export interface ErrorPayload {
  kind: "VALIDATION" | "NOT_FOUND" | "STORAGE";
  message: string;
}

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

export interface Milestone {
  id: string;
  goal_id: string;
  title: string;
  description: string | null;
  target_date: string | null;
  status: Status;
  order_index: number;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}

export interface NewMilestone {
  goal_id: string;
  title: string;
  description: string | null;
  target_date: string | null;
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

export interface LogEntry {
  id: string;
  log_date: string;
  task_id: string | null;
  milestone_id: string | null;
  goal_id: string | null;
  activity: string;
  duration_minutes: number | null;
  category: string | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface NewLogEntry {
  log_date: string;
  task_id: string | null;
  milestone_id: string | null;
  goal_id: string | null;
  activity: string;
  duration_minutes: number | null;
  category: string | null;
  notes: string | null;
}

export interface AvailabilityWindow {
  id: string;
  weekday: number;
  start_minute: number;
  end_minute: number;
  label: string | null;
  created_at: string;
}

export interface NewAvailabilityWindow {
  weekday: number;
  start_minute: number;
  end_minute: number;
  label: string | null;
}

export interface AvailabilityException {
  id: string;
  date: string;
  start_minute: number;
  end_minute: number;
  is_available: boolean;
  note: string | null;
  created_at: string;
}

export interface NewAvailabilityException {
  date: string;
  start_minute: number;
  end_minute: number;
  is_available: boolean;
  note: string | null;
}

export interface Interval {
  start: number;
  end: number;
}

export interface DayCapacity {
  date: string;
  windows: Interval[];
  total_minutes: number;
}