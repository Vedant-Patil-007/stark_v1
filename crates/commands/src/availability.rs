use rusqlite::Connection;
use stark_domain::{
    AvailabilityException, AvailabilityId, AvailabilityWindow, DayCapacity, ExceptionId,
    NewAvailabilityException, NewAvailabilityWindow,
};
use stark_storage::availability_repo;
use crate::error::{CommandError, Result};
use crate::validate;

fn check_minutes(start: i64, end: i64) -> Result<()> {
    if !(0..=1440).contains(&start) || !(0..=1440).contains(&end) {
        return Err(CommandError::Validation(
            "times must be between 00:00 and 24:00".into(),
        ));
    }
    if end <= start {
        return Err(CommandError::Validation(
            "end time must be after start time".into(),
        ));
    }
    Ok(())
}

pub fn create_availability_window(
    conn: &Connection,
    input: NewAvailabilityWindow,
) -> Result<AvailabilityWindow> {
    if !(0..=6).contains(&input.weekday) {
        return Err(CommandError::Validation(
            "weekday must be 0 (Sunday) through 6 (Saturday)".into(),
        ));
    }
    check_minutes(input.start_minute, input.end_minute)?;
    Ok(availability_repo::create_window(conn, input)?)
}

pub fn list_availability_windows(conn: &Connection) -> Result<Vec<AvailabilityWindow>> {
    Ok(availability_repo::list_windows(conn)?)
}

pub fn delete_availability_window(conn: &Connection, id: &AvailabilityId) -> Result<()> {
    if availability_repo::delete_window(conn, id)? {
        Ok(())
    } else {
        Err(CommandError::NotFound(format!("availability window {id}")))
    }
}

pub fn create_availability_exception(
    conn: &Connection,
    mut input: NewAvailabilityException,
) -> Result<AvailabilityException> {
    let date = validate::optional_date(Some(input.date.clone()), "date")?
        .ok_or_else(|| CommandError::Validation("date is required".into()))?;
    input.date = date;
    check_minutes(input.start_minute, input.end_minute)?;
    Ok(availability_repo::create_exception(conn, input)?)
}

pub fn list_availability_exceptions(
    conn: &Connection,
    from: &str,
    to: &str,
) -> Result<Vec<AvailabilityException>> {
    if from > to {
        return Err(CommandError::Validation(
            "range start cannot be after range end".into(),
        ));
    }
    Ok(availability_repo::list_exceptions_in_range(conn, from, to)?)
}

pub fn delete_availability_exception(conn: &Connection, id: &ExceptionId) -> Result<()> {
    if availability_repo::delete_exception(conn, id)? {
        Ok(())
    } else {
        Err(CommandError::NotFound(format!("exception {id}")))
    }
}

pub fn capacity_for_date(
    conn: &Connection,
    date: &str,
    weekday: i64,
) -> Result<DayCapacity> {
    Ok(availability_repo::capacity_for_date(conn, date, weekday)?)
}