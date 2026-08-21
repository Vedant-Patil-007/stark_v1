import { invoke } from "@tauri-apps/api/core";
import type {
  Goal, NewGoal,
  Milestone, NewMilestone,
  Task, NewTask, TaskFilter,
  LogEntry, NewLogEntry,
  AvailabilityWindow, NewAvailabilityWindow,
  AvailabilityException, NewAvailabilityException,
  DayCapacity,
  Analysis,
  Reminder,
  AiResult,
  Status,
} from "./types";

export const api = {
  // ---- goals ----
  createGoal: (input: NewGoal) => invoke<Goal>("create_goal", { input }),
  listGoals: () => invoke<Goal[]>("list_goals"),
  deleteGoal: (id: string) => invoke<void>("delete_goal", { id }),

  // ---- milestones ----
  createMilestone: (input: NewMilestone) =>
    invoke<Milestone>("create_milestone", { input }),
  listMilestones: (goalId: string) =>
    invoke<Milestone[]>("list_milestones", { goalId }),
  setMilestoneStatus: (id: string, status: Status) =>
    invoke<void>("set_milestone_status", { id, status }),
  deleteMilestone: (id: string) => invoke<void>("delete_milestone", { id }),

  // ---- tasks ----
  createTask: (input: NewTask) => invoke<Task>("create_task", { input }),
  listTasks: (filter: TaskFilter) => invoke<Task[]>("list_tasks", { filter }),
  tasksInRange: (from: string, to: string) =>
    invoke<Task[]>("tasks_in_range", { from, to }),
  setTaskStatus: (id: string, status: Status) =>
    invoke<void>("set_task_status", { id, status }),
  rescheduleTask: (id: string, scheduledDate: string | null) =>
    invoke<void>("reschedule_task", { id, scheduledDate }),
  deleteTask: (id: string) => invoke<void>("delete_task", { id }),

  // ---- daily log ----
  createLogEntry: (input: NewLogEntry) =>
    invoke<LogEntry>("create_log_entry", { input }),
  listLogForDate: (date: string) =>
    invoke<LogEntry[]>("list_log_for_date", { date }),
  deleteLogEntry: (id: string) => invoke<void>("delete_log_entry", { id }),

  // ---- availability ----
  createAvailabilityWindow: (input: NewAvailabilityWindow) =>
    invoke<AvailabilityWindow>("create_availability_window", { input }),
  listAvailabilityWindows: () =>
    invoke<AvailabilityWindow[]>("list_availability_windows"),
  deleteAvailabilityWindow: (id: string) =>
    invoke<void>("delete_availability_window", { id }),
  createAvailabilityException: (input: NewAvailabilityException) =>
    invoke<AvailabilityException>("create_availability_exception", { input }),
  listAvailabilityExceptions: (from: string, to: string) =>
    invoke<AvailabilityException[]>("list_availability_exceptions", { from, to }),
  deleteAvailabilityException: (id: string) =>
    invoke<void>("delete_availability_exception", { id }),
  capacityForDate: (date: string, weekday: number) =>
    invoke<DayCapacity>("capacity_for_date", { date, weekday }),

  // ---- planning ----
  analyzePlan: (today: string) => invoke<Analysis>("analyze_plan", { today }),
  todayTasks: (today: string) => invoke<Task[]>("today_tasks", { today }),
  overdueTasks: (today: string) => invoke<Task[]>("overdue_tasks", { today }),

  // ---- reminders ----
  syncReminders: (date: string, offsetMinutes: number) =>
    invoke<number>("sync_reminders", { date, offsetMinutes }),
  listMissedReminders: () => invoke<Reminder[]>("list_missed_reminders"),
  dismissReminder: (id: string) => invoke<void>("dismiss_reminder", { id }),

  // ---- ai ----
  runAiCommand: (instruction: string, today: string) =>
    invoke<AiResult>("run_ai_command", { instruction, today }),
  setAiKey: (key: string) => invoke<void>("set_ai_key", { key }),
    hasAiKey: () => invoke<string>("has_ai_key"),
  clearAiKey: () => invoke<void>("clear_ai_key"),
};