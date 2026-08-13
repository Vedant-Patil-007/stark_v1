# Stark

A goal-driven personal planning and execution system for Windows.

Stark is not a to-do list. It is a system that takes a long-term goal, breaks it
into milestones and tasks, schedules that work against the hours you actually
have, records what you actually did, and then tells you whether you are on track
or falling behind — with a number, not a feeling.

> **Status: pre-release, in active development.** The core planner works and
> persists data reliably. The planning engine, reminders, and AI layer are not
> built yet. See [Roadmap](#roadmap) for exactly what exists today.

---

## The problem

Most planning tools store intentions. Very few of them can answer the question
that actually matters:

> *"I have 46 hours of work left on this project, 20 days until the deadline,
> and I'm available 31 hours in that window. Am I going to make it?"*

A calendar can't answer that — it doesn't know your workload. A to-do app can't
answer it — it doesn't know your capacity. Stark holds both, and does the
arithmetic.

---

## Core principle

**AI is not the database. AI is not the planner. AI does not directly control
application state.**

Every mutation flows through the same path, whether it originated from a button
click, a typed command, or speech:

```
        Manual UI ─┐
      Text command ─┼──▶ Interpreter ──▶ Validator ──▶ Command Layer ──▶ SQLite
     Voice command ─┘                                        │
                                                             ▼
                                                    Planning Engine
                                                             │
                                                             ▼
                                           Dashboard · Calendar · Notifications
```

This is enforced by the compiler, not by convention. The React frontend has no
dependency on the storage crate; it *cannot* issue SQL. A language model's only
possible output is a typed `Command` value that must survive validation before
anything is written.

The reliability guarantee that falls out of this:

| If this fails | This still works |
|---|---|
| AI | Calendar, tasks, planning |
| Internet | Everything except cloud AI |
| Voice | Reminders, manual entry |
| The cloud provider disappears | The entire core planner |

---

## Architecture

A Rust workspace with strictly one-way dependencies:

```
stark/
├─ crates/
│  ├─ domain/       Pure types and logic. No I/O, no DB, no async.
│  ├─ storage/      SQLite repositories + migrations. The ONLY place with SQL.
│  ├─ commands/     Validation + command execution. The ONLY writer.
│  └─ planning/     The deterministic engine (planned)
├─ src-tauri/       Thin Tauri layer: #[tauri::command] wrappers
└─ src/             React frontend
```

```
src-tauri → commands → { storage, planning } → domain
```

`domain` depends on nothing. `planning` depends only on `domain` — never on
`storage`. That constraint is what makes the engine testable: it operates on an
in-memory snapshot struct, so its tests need no database, no fixtures, and no
async runtime.

### Design decisions worth calling out

**Due date ≠ scheduled date.** An assignment due Friday that you plan to write
on Wednesday has two distinct dates. Conflating them makes workload planning
impossible, so they are separate columns and separate UI fields.

**Manual priority ≠ calculated urgency.** What you decide is HIGH stays HIGH.
What the engine computes from deadline pressure and capacity shortfall is a
separate value. The engine never overwrites your judgement.

**Times are stored as minutes from local midnight.** "Available Monday 09:00–17:00"
is a recurring pattern, not an instant. Storing it as an integer (540 to 1020)
sidesteps DST entirely. Instants use ISO-8601 UTC; calendar dates use bare
`YYYY-MM-DD` with no timezone.

**Unknown stays unknown.** If there isn't enough information to assign a
deadline, duration, or goal, the system creates an Inbox item rather than
inventing plausible values. This applies to the deterministic engine as much as
to AI — a confident "you're 15 hours short" derived from incomplete estimates is
worse than no estimate, so analysis output carries an explicit coverage and
confidence figure.

**Backups are not optional.** A timestamped `VACUUM INTO` snapshot is taken
daily on startup, and a pre-migration backup is mandatory before any schema
change — if the backup fails, the migration does not run.

---

## Technology

| Layer | Choice | Why |
|---|---|---|
| Desktop shell | Tauri 2 | Native Windows binary, small footprint, Rust core |
| Frontend | React + TypeScript + Vite | Familiar, fast iteration |
| Core logic | Rust | Type safety for the command/validation boundary |
| Database | SQLite via `rusqlite` (bundled) | Zero-install, embedded, no external dependency |
| Migrations | Custom runner on `user_version` | Transactional, idempotent, backup-gated |
| Planning engine | Custom deterministic Rust | Not an LLM. Must be reproducible and testable |
| AI | Provider abstraction (planned) | No vendor lock-in |
| Speech | Replaceable STT/TTS traits (planned) | Local or cloud, swappable |

Deliberately **not** used: `tauri-plugin-sql` — it exposes raw SQL execution to
JavaScript, which would place database access in the frontend and reduce the
architecture's central rule to a naming convention.

---

## Constraints

Built and tested on modest hardware, which shaped several decisions:

- Intel i5-1135G7, **8 GB RAM**, integrated graphics, Windows 11
- No large local LLMs. Cloud AI for complex reasoning; a deterministic parser
  handles the common commands with zero latency and zero cost
- Local speech-to-text, when added, targets `whisper.cpp base.en` quantized
  (~57 MB on disk, ~400 MB RAM) rather than anything larger
- Target recurring infrastructure cost: **$0/month**. Cloud AI is optional

---

## Roadmap

### Built

- [x] Cargo workspace with enforced layer separation
- [x] SQLite embedded, WAL mode, foreign keys enforced
- [x] Migration runner — transactional, idempotent, version-tracked
- [x] Automatic backups: daily + mandatory pre-migration
- [x] Goals with structured success criteria
- [x] Milestones, ordered per goal
- [x] Tasks with independent due and scheduled dates, estimates, priorities
- [x] Daily log — the single source of truth for time actually spent
- [x] Calendar month view distinguishing scheduled work from deadlines
- [x] Availability: recurring weekly windows + date-specific exceptions
- [x] Capacity calculation via interval arithmetic (pure functions, unit tested)
- [x] Windows toast notifications verified from a background thread

### V1 — in progress

- [ ] **Planning engine** — urgency, deadline risk, workload vs capacity,
      `ON_TRACK` / `AT_RISK` / `BEHIND` / `CRITICAL`, capacity shortfall
- [ ] **Dashboard** — today's plan, goal status, upcoming deadlines, recommendations
- [ ] **Reminders** — background scheduler, system tray, autostart,
      catch-up for reminders missed while closed
- [ ] Packaging, installer, first-run experience

### V1.5

- [ ] Natural-language commands — closed action enum, ID resolution by lookup
      (the model never emits raw IDs), confirmation gates, action history, undo
- [ ] Deterministic fast-path parser for common commands — no network, no cost
- [ ] Voice — push-to-talk, local STT, Windows SAPI for speech output
- [ ] Replanning proposals: detect drift → analyse → propose → approve → apply

### Later

Recurring tasks · Inbox · Notes and resources · Analytics · Goal insights ·
External calendar integration · Private sync server · Mobile companion ·
Always-listening wake word

Explicitly **not** planned for V1: multi-user support, web hosting, mobile apps.
This is a single-user, local-first desktop system.

---

## Running it

Requires Rust (stable, MSVC toolchain), Node.js LTS, and the Visual Studio
Build Tools with the *Desktop development with C++* workload.

```bash
git clone https://github.com/<your-username>/stark.git
cd stark
npm install
npm run tauri dev
```

Tests:

```bash
cargo test
```

The database lives at `%APPDATA%\com.vspat.stark\stark.db`, with backups
alongside it. Nothing is stored in the repository.

**Note:** Windows toast notifications require an installed application, so
reminder behaviour must be verified against a packaged build (`npm run tauri build`),
not the dev server.

---

## Why the constraints are the interesting part

The hard problem here isn't storing tasks. It's building something that can be
trusted enough to actually rely on — where a schema change can't silently
corrupt six months of history, where a language model can't reach the database,
where a wrong estimate produces a low-confidence marker rather than a
confidently wrong number, and where every component keeps working when the ones
above it fail.

That is why the migration runner, the backup routine, and the layer boundaries
were built before a single planning feature.

---

## License

Personal project. Not currently licensed for redistribution.
