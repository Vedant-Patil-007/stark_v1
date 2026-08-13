import { invoke } from "@tauri-apps/api/core";
import type {
  Goal, NewGoal,
  Milestone, NewMilestone,
  Task, NewTask, TaskFilter,
  LogEntry, NewLogEntry,
  Status,AvailabilityWindow, NewAvailabilityWindow,
  AvailabilityException, NewAvailabilityException,
  DayCapacity,
} from "./types";

export const api = {
  createGoal: (input: NewGoal) => invoke<Goal>("create_goal", { input }),
  listGoals: () => invoke<Goal[]>("list_goals"),
  deleteGoal: (id: string) => invoke<void>("delete_goal", { id }),

  createMilestone: (input: NewMilestone) =>
    invoke<Milestone>("create_milestone", { input }),
  listMilestones: (goalId: string) =>
    invoke<Milestone[]>("list_milestones", { goalId }),
  setMilestoneStatus: (id: string, status: Status) =>
    invoke<void>("set_milestone_status", { id, status }),
  deleteMilestone: (id: string) => invoke<void>("delete_milestone", { id }),

  createTask: (input: NewTask) => invoke<Task>("create_task", { input }),
  listTasks: (filter: TaskFilter) => invoke<Task[]>("list_tasks", { filter }),
  setTaskStatus: (id: string, status: Status) =>
    invoke<void>("set_task_status", { id, status }),
  rescheduleTask: (id: string, scheduledDate: string | null) =>
    invoke<void>("reschedule_task", { id, scheduledDate }),
  deleteTask: (id: string) => invoke<void>("delete_task", { id }),
  createLogEntry: (input: NewLogEntry) =>
    invoke<LogEntry>("create_log_entry", { input }),
  listLogForDate: (date: string) =>
    invoke<LogEntry[]>("list_log_for_date", { date }),
  deleteLogEntry: (id: string) => invoke<void>("delete_log_entry", { id }),
  tasksInRange: (from: string, to: string) =>
    invoke<Task[]>("tasks_in_range", { from, to }),
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
};