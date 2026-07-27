use crate::error::{CommandError, Result};

pub const MAX_TITLE_LEN: usize = 200;
pub const MAX_DESCRIPTION_LEN: usize = 5000;

pub fn title(value: &str) -> Result<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(CommandError::Validation("title cannot be empty".into()));
    }
    if trimmed.chars().count() > MAX_TITLE_LEN {
        return Err(CommandError::Validation(format!(
            "title cannot exceed {MAX_TITLE_LEN} characters"
        )));
    }
    Ok(trimmed.to_string())
}

pub fn optional_description(value: Option<String>) -> Result<Option<String>> {
    match value {
        None => Ok(None),
        Some(v) => {
            let trimmed = v.trim();
            if trimmed.is_empty() {
                return Ok(None);
            }
            if trimmed.chars().count() > MAX_DESCRIPTION_LEN {
                return Err(CommandError::Validation(format!(
                    "description cannot exceed {MAX_DESCRIPTION_LEN} characters"
                )));
            }
            Ok(Some(trimmed.to_string()))
        }
    }
}

/// Expects YYYY-MM-DD, a local calendar date.
pub fn optional_date(value: Option<String>, field: &str) -> Result<Option<String>> {
    match value {
        None => Ok(None),
        Some(v) if v.trim().is_empty() => Ok(None),
        Some(v) => {
            let v = v.trim();
            let valid = v.len() == 10
                && v.as_bytes()[4] == b'-'
                && v.as_bytes()[7] == b'-'
                && v.chars().enumerate().all(|(i, c)| {
                    if i == 4 || i == 7 { c == '-' } else { c.is_ascii_digit() }
                });
            if !valid {
                return Err(CommandError::Validation(format!(
                    "{field} must be in YYYY-MM-DD format"
                )));
            }
            Ok(Some(v.to_string()))
        }
    }
}

pub fn date_order(start: &Option<String>, target: &Option<String>) -> Result<()> {
    if let (Some(s), Some(t)) = (start, target) {
        if s > t {
            return Err(CommandError::Validation(
                "start date cannot be after target date".into(),
            ));
        }
    }
    Ok(())
}