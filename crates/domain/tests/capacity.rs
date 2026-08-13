use stark_domain::capacity::{normalize, subtract, total_minutes};
use stark_domain::Interval;

fn iv(start: i64, end: i64) -> Interval {
    Interval { start, end }
}

#[test]
fn normalize_merges_overlaps() {
    let out = normalize(vec![iv(540, 720), iv(700, 900)]);
    assert_eq!(out, vec![iv(540, 900)]);
}

#[test]
fn normalize_merges_adjacent() {
    let out = normalize(vec![iv(540, 720), iv(720, 900)]);
    assert_eq!(out, vec![iv(540, 900)]);
}

#[test]
fn normalize_keeps_disjoint() {
    let out = normalize(vec![iv(540, 720), iv(800, 900)]);
    assert_eq!(out, vec![iv(540, 720), iv(800, 900)]);
}

#[test]
fn subtract_carves_middle() {
    // 9am-5pm minus 2pm-3pm = 9am-2pm plus 3pm-5pm
    let out = subtract(vec![iv(540, 1020)], vec![iv(840, 900)]);
    assert_eq!(out, vec![iv(540, 840), iv(900, 1020)]);
}

#[test]
fn subtract_full_cover_leaves_nothing() {
    let out = subtract(vec![iv(540, 1020)], vec![iv(500, 1100)]);
    assert!(out.is_empty());
}

#[test]
fn subtract_non_overlapping_is_noop() {
    let out = subtract(vec![iv(540, 1020)], vec![iv(1100, 1200)]);
    assert_eq!(out, vec![iv(540, 1020)]);
}

#[test]
fn total_minutes_sums_correctly() {
    // 9am-5pm minus a one-hour lunch = 7 hours = 420 minutes
    let out = subtract(vec![iv(540, 1020)], vec![iv(720, 780)]);
    assert_eq!(total_minutes(&out), 420);
}