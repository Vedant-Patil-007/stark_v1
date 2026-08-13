use rusqlite::{Connection, params};
use stark_domain::{
    AvailabilityException, AvailabilityId, AvailabilityWindow, DayCapacity, ExceptionId,
    Interval, NewAvailabilityException, NewAvailabilityWindow,
};
use stark_domain::capacity;
use crate::error::Result;
use crate::time_util::now_utc;

pub fn create_window(
    conn: &Connection,
    input: NewAvailabilityWindow,
) -> Result<AvailabilityWindow> {
    let window = AvailabilityWindow {
        id: AvailabilityId::new(),
        weekday: input.weekday,
        start_minute: input.start_minute,
        end_minute: input.end_minute,
        label: input.label,
        created_at: now_utc(),
    };

    conn.execute(
        "INSERT INTO availability_template
            (id, weekday, start_minute, end_minute, label, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![
            window.id.as_str(),
            window.weekday,
            window.start_minute,
            window.end_minute,
            window.label,
            window.created_at,
        ],
    )?;

    Ok(window)
}

pub fn list_windows(conn: &Connection) -> Result<Vec<AvailabilityWindow>> {
    let mut stmt = conn.prepare(
        "SELECT id, weekday, start_minute, end_minute, label, created_at
         FROM availability_template
         ORDER BY weekday, start_minute",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(AvailabilityWindow {
            id: AvailabilityId::from(row.get::<_, String>(0)?),
            weekday: row.get(1)?,
            start_minute: row.get(2)?,
            end_minute: row.get(3)?,
            label: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn delete_window(conn: &Connection, id: &AvailabilityId) -> Result<bool> {
    let n = conn.execute(
        "DELETE FROM availability_template WHERE id = ?1",
        params![id.as_str()],
    )?;
    Ok(n > 0)
}

pub fn create_exception(
    conn: &Connection,
    input: NewAvailabilityException,
) -> Result<AvailabilityException> {
    let exception = AvailabilityException {
        id: ExceptionId::new(),
        date: input.date,
        start_minute: input.start_minute,
        end_minute: input.end_minute,
        is_available: input.is_available,
        note: input.note,
        created_at: now_utc(),
    };

    conn.execute(
        "INSERT INTO availability_exception
            (id, date, start_minute, end_minute, is_available, note, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            exception.id.as_str(),
            exception.date,
            exception.start_minute,
            exception.end_minute,
            exception.is_available as i64,
            exception.note,
            exception.created_at,
        ],
    )?;

    Ok(exception)
}

pub fn list_exceptions_in_range(
    conn: &Connection,
    from: &str,
    to: &str,
) -> Result<Vec<AvailabilityException>> {
    let mut stmt = conn.prepare(
        "SELECT id, date, start_minute, end_minute, is_available, note, created_at
         FROM availability_exception
         WHERE date >= ?1 AND date <= ?2
         ORDER BY date, start_minute",
    )?;
    let rows = stmt.query_map(params![from, to], |row| {
        Ok(AvailabilityException {
            id: ExceptionId::from(row.get::<_, String>(0)?),
            date: row.get(1)?,
            start_minute: row.get(2)?,
            end_minute: row.get(3)?,
            is_available: row.get::<_, i64>(4)? != 0,
            note: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
}

pub fn delete_exception(conn: &Connection, id: &ExceptionId) -> Result<bool> {
    let n = conn.execute(
        "DELETE FROM availability_exception WHERE id = ?1",
        params![id.as_str()],
    )?;
    Ok(n > 0)
}

/// Capacity for one date: weekly template, plus additive exceptions,
/// minus blocking exceptions.
pub fn capacity_for_date(
    conn: &Connection,
    date: &str,
    weekday: i64,
) -> Result<DayCapacity> {
    let windows = list_windows(conn)?;
    let exceptions = list_exceptions_in_range(conn, date, date)?;

    let mut base: Vec<Interval> = windows
        .iter()
        .filter(|w| w.weekday == weekday)
        .map(|w| Interval { start: w.start_minute, end: w.end_minute })
        .collect();

    // Additive exceptions extend the base before anything is cut.
    base.extend(
        exceptions
            .iter()
            .filter(|e| e.is_available)
            .map(|e| Interval { start: e.start_minute, end: e.end_minute }),
    );

    let cuts: Vec<Interval> = exceptions
        .iter()
        .filter(|e| !e.is_available)
        .map(|e| Interval { start: e.start_minute, end: e.end_minute })
        .collect();

    let result = capacity::subtract(base, cuts);
    let total = capacity::total_minutes(&result);

    Ok(DayCapacity {
        date: date.to_string(),
        windows: result,
        total_minutes: total,
    })
}