use rusqlite::Connection;
use stark_ai::action::AiAction;
use stark_ai::resolver::{clarification_question, resolve, Candidate, Resolution};
use stark_domain::{
    GoalId, NewGoal, NewLogEntry, NewTask, Priority, Status, TaskFilter, TaskId,
};
use stark_storage::{goal_repo, task_repo};

use crate::error::{Result};
use crate::{daily_log, goal, task};

/// What happened when an AI action was applied.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApplyOutcome {
    /// The action was executed. `summary` is shown to the user.
    Executed { summary: String },
    /// A reference was ambiguous or missing. Ask before acting.
    NeedsClarification {
        question: String,
        candidates: Vec<String>,
    },
    /// Captured to the Inbox because there wasn't enough to act on.
    Captured { summary: String },
    /// A read-only query; nothing was changed.
    Answered { summary: String },
}

/// Build the candidate list the resolver matches against.
fn goal_candidates(conn: &Connection) -> Result<Vec<Candidate>> {
    Ok(goal_repo::list(conn)?
        .into_iter()
        .map(|g| Candidate {
            id: g.id.as_str().to_string(),
            name: g.title,
        })
        .collect())
}

fn task_candidates(conn: &Connection) -> Result<Vec<Candidate>> {
    Ok(task_repo::list(
        conn,
        &TaskFilter {
            goal_id: None,
            milestone_id: None,
            scheduled_date: None,
            include_completed: false,
        },
    )?
    .into_iter()
    .map(|t| Candidate {
        id: t.id.as_str().to_string(),
        name: t.title,
    })
    .collect())
}

/// Resolve an optional goal reference. None stays None; an unresolvable
/// reference is an error rather than a silent drop.
fn resolve_goal(conn: &Connection, r: &Option<String>) -> Result<ResolveStep<GoalId>> {
    let Some(reference) = r else {
        return Ok(ResolveStep::Absent);
    };
    let candidates = goal_candidates(conn)?;
    match resolve(reference, &candidates) {
        Resolution::Resolved(id) => Ok(ResolveStep::Found(GoalId::from(id))),
        Resolution::Ambiguous(c) => Ok(ResolveStep::Ambiguous {
            question: clarification_question(reference, &c),
            candidates: c.into_iter().map(|x| x.name).collect(),
        }),
        Resolution::NotFound => Ok(ResolveStep::NotFound(reference.clone())),
    }
}

fn resolve_task(conn: &Connection, reference: &str) -> Result<ResolveStep<TaskId>> {
    let candidates = task_candidates(conn)?;
    match resolve(reference, &candidates) {
        Resolution::Resolved(id) => Ok(ResolveStep::Found(TaskId::from(id))),
        Resolution::Ambiguous(c) => Ok(ResolveStep::Ambiguous {
            question: clarification_question(reference, &c),
            candidates: c.into_iter().map(|x| x.name).collect(),
        }),
        Resolution::NotFound => Ok(ResolveStep::NotFound(reference.to_string())),
    }
}

enum ResolveStep<T> {
    Found(T),
    Absent,
    Ambiguous {
        question: String,
        candidates: Vec<String>,
    },
    NotFound(String),
}

/// Apply a validated AI action. Every path either changes state deliberately
/// or explains why it didn't. Nothing is guessed.
pub fn apply(conn: &mut Connection, action: AiAction, today: &str) -> Result<ApplyOutcome> {
    match action {
        AiAction::CreateTask {
            title,
            goal_ref,
            due_date,
            scheduled_date,
            estimated_minutes,
            priority,
        } => {
            let goal_id = match resolve_goal(conn, &goal_ref)? {
                ResolveStep::Found(id) => Some(id),
                ResolveStep::Absent => None,
                ResolveStep::Ambiguous { question, candidates } => {
                    return Ok(ApplyOutcome::NeedsClarification { question, candidates })
                }
                ResolveStep::NotFound(r) => {
                    return Ok(ApplyOutcome::NeedsClarification {
                        question: format!("I couldn't find a goal called \"{r}\"."),
                        candidates: goal_candidates(conn)?
                            .into_iter()
                            .map(|c| c.name)
                            .collect(),
                    })
                }
            };

            let t = task::create_task(
                conn,
                NewTask {
                    goal_id,
                    milestone_id: None,
                    title: title.clone(),
                    description: None,
                    due_date,
                    scheduled_date,
                    estimated_minutes,
                    priority: priority.unwrap_or(Priority::Medium),
                },
            )?;

            Ok(ApplyOutcome::Executed {
                summary: format!("Created task \"{}\".", t.title),
            })
        }

        AiAction::CompleteTask { task_ref } => match resolve_task(conn, &task_ref)? {
            ResolveStep::Found(id) => {
                task::set_task_status(conn, &id, Status::Completed)?;
                Ok(ApplyOutcome::Executed {
                    summary: format!("Marked \"{task_ref}\" complete."),
                })
            }
            ResolveStep::Ambiguous { question, candidates } => {
                Ok(ApplyOutcome::NeedsClarification { question, candidates })
            }
            ResolveStep::NotFound(r) => Ok(ApplyOutcome::NeedsClarification {
                question: format!("I couldn't find an open task called \"{r}\"."),
                candidates: task_candidates(conn)?.into_iter().map(|c| c.name).collect(),
            }),
            ResolveStep::Absent => unreachable!("task_ref is required"),
        },

        AiAction::RescheduleTask { task_ref, scheduled_date } => {
            match resolve_task(conn, &task_ref)? {
                ResolveStep::Found(id) => {
                    task::reschedule_task(conn, &id, Some(scheduled_date.clone()))?;
                    Ok(ApplyOutcome::Executed {
                        summary: format!("Moved \"{task_ref}\" to {scheduled_date}."),
                    })
                }
                ResolveStep::Ambiguous { question, candidates } => {
                    Ok(ApplyOutcome::NeedsClarification { question, candidates })
                }
                ResolveStep::NotFound(r) => Ok(ApplyOutcome::NeedsClarification {
                    question: format!("I couldn't find an open task called \"{r}\"."),
                    candidates: task_candidates(conn)?.into_iter().map(|c| c.name).collect(),
                }),
                ResolveStep::Absent => unreachable!("task_ref is required"),
            }
        }

        AiAction::CreateGoal { title, target_date, priority } => {
            let g = goal::create_goal(
                conn,
                NewGoal {
                    title: title.clone(),
                    description: None,
                    start_date: None,
                    target_date,
                    priority: priority.unwrap_or(Priority::Medium),
                    estimated_effort_minutes: None,
                    success_criteria: Vec::new(),
                },
            )?;
            Ok(ApplyOutcome::Executed {
                summary: format!("Created goal \"{}\".", g.title),
            })
        }

        AiAction::LogWork { activity, duration_minutes, goal_ref, log_date } => {
            let goal_id = match resolve_goal(conn, &goal_ref)? {
                ResolveStep::Found(id) => Some(id),
                ResolveStep::Absent | ResolveStep::NotFound(_) => None,
                ResolveStep::Ambiguous { question, candidates } => {
                    return Ok(ApplyOutcome::NeedsClarification { question, candidates })
                }
            };

            daily_log::create_log_entry(
                conn,
                NewLogEntry {
                    log_date: log_date.unwrap_or_else(|| today.to_string()),
                    task_id: None,
                    milestone_id: None,
                    goal_id,
                    activity: activity.clone(),
                    duration_minutes,
                    category: None,
                    notes: None,
                },
            )?;

            let time = match duration_minutes {
                Some(m) => format!(" ({m} min)"),
                None => String::new(),
            };
            Ok(ApplyOutcome::Executed {
                summary: format!("Logged \"{activity}\"{time}."),
            })
        }

        AiAction::SetAvailability { date, start_minute, end_minute, is_available } => {
            crate::availability::create_availability_exception(
                conn,
                stark_domain::NewAvailabilityException {
                    date: date.clone(),
                    start_minute,
                    end_minute,
                    is_available,
                    note: None,
                },
            )?;
            let word = if is_available { "available" } else { "unavailable" };
            Ok(ApplyOutcome::Executed {
                summary: format!("Marked you {word} on {date}."),
            })
        }

        AiAction::QueryProgress { goal_ref } => {
            let analysis = crate::planning::analyze_plan(conn, today)?;
            let summary = match goal_ref {
                Some(r) => {
                    let candidates = goal_candidates(conn)?;
                    match resolve(&r, &candidates) {
                        Resolution::Resolved(id) => analysis
                            .goals
                            .iter()
                            .find(|g| g.goal_id == id)
                            .map(|g| {
                                format!(
                                    "{}: {}% complete, {}. {}",
                                    g.title,
                                    (g.progress * 100.0).round(),
                                    format!("{:?}", g.health),
                                    g.reason
                                )
                            })
                            .unwrap_or_else(|| "No analysis available.".into()),
                        _ => format!("I couldn't find a goal matching \"{r}\"."),
                    }
                }
                None => format!(
                    "{} task(s) today, {} overdue. {} goal(s) tracked.",
                    analysis.today_task_count,
                    analysis.overdue_task_count,
                    analysis.goals.len()
                ),
            };
            Ok(ApplyOutcome::Answered { summary })
        }

        AiAction::CreateInboxItem { raw_text } => {
            // Inbox table doesn't exist yet; capture as an unscheduled task
            // with no invented fields. Replace when Inbox lands in Phase 5.
            task::create_task(
                conn,
                NewTask {
                    goal_id: None,
                    milestone_id: None,
                    title: raw_text.clone(),
                    description: Some("Captured from AI — needs detail".into()),
                    due_date: None,
                    scheduled_date: None,
                    estimated_minutes: None,
                    priority: Priority::Low,
                },
            )?;
            Ok(ApplyOutcome::Captured {
                summary: format!("Captured \"{raw_text}\" — add detail when you can."),
            })
        }

        AiAction::NeedsClarification { question, candidates } => {
            Ok(ApplyOutcome::NeedsClarification { question, candidates })
        }
    }
}