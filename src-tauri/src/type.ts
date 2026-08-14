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

