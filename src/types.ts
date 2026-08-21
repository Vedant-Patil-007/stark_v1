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
export type GoalHealth =
  | "ON_TRACK"
  | "AT_RISK"
  | "BEHIND"
  | "CRITICAL"
  | "NOT_APPLICABLE";

export type Confidence = "HIGH" | "MEDIUM" | "LOW";

export interface GoalAnalysis {
  goal_id: string;
  title: string;
  progress: number;
  tasks_total: number;
  tasks_completed: number;
  workload_remaining_minutes: number;
  capacity_available_minutes: number;
  shortfall_minutes: number;
  days_remaining: number | null;
  health: GoalHealth;
  estimate_coverage: number;
  unestimated_task_count: number;
  confidence: Confidence;
  reason: string;
}

export interface Analysis {
  generated_for: string;
  goals: GoalAnalysis[];
  today_task_count: number;
  today_planned_minutes: number;
  today_capacity_minutes: number;
  overdue_task_count: number;
  upcoming: UpcomingItem[];
  capacity_next_7_days_minutes: number;
}
export type UpcomingKind = "TASK_DUE" | "MILESTONE_TARGET" | "GOAL_TARGET";

export interface UpcomingItem {
  date: string;
  label: string;
  kind: UpcomingKind;
  days_away: number;
}

export type ReminderStatus = "PENDING" | "FIRED" | "MISSED" | "DISMISSED";

export interface Reminder {
  id: string;
  task_id: string | null;
  goal_id: string | null;
  fire_at_utc: string;
  title: string;
  body: string | null;
  status: ReminderStatus;
  fired_at: string | null;
  created_at: string;
}

export type ApplyOutcome =
  | { kind: "EXECUTED"; summary: string }
  | { kind: "NEEDS_CLARIFICATION"; question: string; candidates: string[] }
  | { kind: "CAPTURED"; summary: string }
  | { kind: "ANSWERED"; summary: string };

export interface AiResult {
  outcome: ApplyOutcome;
  tier: "local" | "cloud";
  latency_ms: number;
  action_name: string;
}