import type { Goal, NewGoal, Milestone, NewMilestone, Status } from "./types";

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

  createLogEntry: (input: NewLogEntry) =>
    invoke<LogEntry>("create_log_entry", { input }),
  listLogForDate: (date: string) =>
    invoke<LogEntry[]>("list_log_for_date", { date }),
  deleteLogEntry: (id: string) => invoke<void>("delete_log_entry", { id }),
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