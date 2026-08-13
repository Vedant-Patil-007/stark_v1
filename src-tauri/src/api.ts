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
};