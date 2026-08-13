import { invoke } from "@tauri-apps/api/core";
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

  createTask: (input: NewTask) => invoke<Task>("create_task", { input }),
  listTasks: (filter: TaskFilter) => invoke<Task[]>("list_tasks", { filter }),
  setTaskStatus: (id: string, status: Status) =>
    invoke<void>("set_task_status", { id, status }),
  rescheduleTask: (id: string, scheduledDate: string | null) =>
    invoke<void>("reschedule_task", { id, scheduledDate }),
  deleteTask: (id: string) => invoke<void>("delete_task", { id }),
};