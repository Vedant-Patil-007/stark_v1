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