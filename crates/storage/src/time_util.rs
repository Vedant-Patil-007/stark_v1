use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

/// Current instant as ISO-8601 UTC. The only place we generate timestamps.
pub fn now_utc() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string())
}