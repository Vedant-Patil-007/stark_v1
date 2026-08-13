use crate::availability::Interval;

/// Merge overlapping and adjacent intervals into a normalised, sorted set.
pub fn normalize(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return Vec::new();
    }
    intervals.sort_by_key(|i| i.start);

    let mut out: Vec<Interval> = Vec::with_capacity(intervals.len());
    for iv in intervals {
        if iv.end <= iv.start {
            continue; // discard degenerate intervals
        }
        match out.last_mut() {
            Some(last) if iv.start <= last.end => {
                if iv.end > last.end {
                    last.end = iv.end;
                }
            }
            _ => out.push(iv),
        }
    }
    out
}

/// Subtract `cuts` from `base`, returning the remaining intervals.
pub fn subtract(base: Vec<Interval>, cuts: Vec<Interval>) -> Vec<Interval> {
    let base = normalize(base);
    let cuts = normalize(cuts);

    let mut out = Vec::new();
    for b in base {
        let mut segments = vec![b];
        for c in &cuts {
            let mut next = Vec::new();
            for s in segments {
                // No overlap: keep the segment whole.
                if c.end <= s.start || c.start >= s.end {
                    next.push(s);
                    continue;
                }
                // Left remainder.
                if c.start > s.start {
                    next.push(Interval { start: s.start, end: c.start });
                }
                // Right remainder.
                if c.end < s.end {
                    next.push(Interval { start: c.end, end: s.end });
                }
                // Fully covered: contributes nothing.
            }
            segments = next;
        }
        out.extend(segments);
    }
    normalize(out)
}

pub fn total_minutes(intervals: &[Interval]) -> i64 {
    intervals.iter().map(|i| i.minutes()).sum()
}