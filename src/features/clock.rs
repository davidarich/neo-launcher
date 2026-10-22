use chrono::{DateTime, Local, Timelike};
use eframe::egui::Color32;

use crate::config::ClockConfig;

// Helper function to determine clock color based on time of day
pub fn get_clock_color(hour: u32, clock_config: &ClockConfig) -> Result<Color32, String> {
    for phase in &clock_config.phases {
        let in_phase = if phase.hour_start == phase.hour_end {
            // same start and end spans the full day
            true
        } else if phase.hour_start < phase.hour_end {
            hour >= phase.hour_start && hour <= phase.hour_end
        } else {
            // range wraps past midnight
            hour >= phase.hour_start || hour <= phase.hour_end
        };
        if in_phase {
            return Color32::from_hex(&phase.color)
                .map_err(|_| format!("invalid hex color: {}", phase.color));
        }
    }
    Color32::from_hex(&clock_config.default_color)
        .map_err(|_| format!("invalid hex color: {}", clock_config.default_color))
}

/// Returns the countdown to the start of the next `TimePhase` after `now`.
///
/// Phases are defined by their `hour_start` on a 24h clock. Phases that start
/// later today take precedence over tomorrow's earliest phase, so the result
/// is the closest start moment in the future.
///
/// Returns `None` when the clock has no phases.
///
/// Format ref - https://docs.rs/chrono/latest/chrono/format/strftime/index.html
pub fn next_phase_start(
    now: &DateTime<Local>,
    clock_config: &ClockConfig,
) -> Option<(u32, u32, String)> {
    if clock_config.phases.is_empty() {
        return None;
    }
    let current_hour = now.hour();
    let mut candidates: Vec<(u32, String)> = clock_config
        .phases
        .iter()
        .map(|p| (p.hour_start, p.name.clone()))
        .collect();

    // Sort so today's phases come first, then earliest hour wins.
    candidates.sort_by_key(|(h, _)| (h <= &current_hour, *h));

    let (next_hour, name) = match candidates.first() {
        Some((h, n)) => (*h, n.clone()),
        None => return None,
    };
    let add_days = next_hour <= current_hour;

    // Compute the whole minutes until the next start in pure arithmetic: the
    // start is at `next_hour:00` today, or `next_hour:00` tomorrow when
    // `add_days`.
    let hours_until = if add_days {
        24 - current_hour + next_hour
    } else {
        next_hour - current_hour
    };
    let total_minutes = hours_until * 60 - now.minute();
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    Some((hours, minutes, name))
}

#[cfg(test)]
mod tests {
    use super::{get_clock_color, next_phase_start};
    use crate::config::{ClockConfig, TimePhase};
    use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
    use eframe::egui::Color32;

    fn phase(start: u32, end: u32, color: &str) -> TimePhase {
        TimePhase {
            name: "test".to_string(),
            hour_start: start,
            hour_end: end,
            color: color.to_string(),
        }
    }

    fn config(default_color: &str, phases: Vec<TimePhase>) -> ClockConfig {
        ClockConfig {
            default_color: default_color.to_string(),
            phases,
        }
    }

    // hour_start == hour_end spans the full day, matching any hour.
    #[test]
    fn full_day_phase_matches_any_hour() {
        let config = config("#cccccc", vec![phase(5, 5, "#111111")]);
        assert_eq!(
            get_clock_color(0, &config).unwrap(),
            Color32::from_hex("#111111").unwrap()
        );
        assert_eq!(
            get_clock_color(13, &config).unwrap(),
            Color32::from_hex("#111111").unwrap()
        );
        assert_eq!(
            get_clock_color(23, &config).unwrap(),
            Color32::from_hex("#111111").unwrap()
        );
    }

    // hour inside [hour_start, hour_end] of a normal range matches.
    #[test]
    fn normal_range_inside_matches() {
        let config = config("#cccccc", vec![phase(8, 15, "#3333cc")]);
        assert_eq!(
            get_clock_color(8, &config).unwrap(),
            Color32::from_hex("#3333cc").unwrap()
        );
        assert_eq!(
            get_clock_color(12, &config).unwrap(),
            Color32::from_hex("#3333cc").unwrap()
        );
        assert_eq!(
            get_clock_color(15, &config).unwrap(),
            Color32::from_hex("#3333cc").unwrap()
        );
    }

    // hour outside a normal range does not match, so the default color is returned.
    #[test]
    fn normal_range_outside_falls_back_to_default() {
        let config = config("#cccccc", vec![phase(8, 15, "#3333cc")]);
        assert_eq!(
            get_clock_color(7, &config).unwrap(),
            Color32::from_hex("#cccccc").unwrap()
        );
        assert_eq!(
            get_clock_color(16, &config).unwrap(),
            Color32::from_hex("#cccccc").unwrap()
        );
        assert_eq!(
            get_clock_color(0, &config).unwrap(),
            Color32::from_hex("#cccccc").unwrap()
        );
    }

    // Wrapping range: hour >= hour_start (top of the range) matches.
    #[test]
    fn wrap_range_high_side_matches() {
        let config = config("#cccccc", vec![phase(22, 5, "#00cc00")]);
        assert_eq!(
            get_clock_color(22, &config).unwrap(),
            Color32::from_hex("#00cc00").unwrap()
        );
        assert_eq!(
            get_clock_color(23, &config).unwrap(),
            Color32::from_hex("#00cc00").unwrap()
        );
    }

    // Wrapping range: hour <= hour_end (bottom of the range) matches.
    #[test]
    fn wrap_range_low_side_matches() {
        let config = config("#cccccc", vec![phase(22, 5, "#00cc00")]);
        assert_eq!(
            get_clock_color(0, &config).unwrap(),
            Color32::from_hex("#00cc00").unwrap()
        );
        assert_eq!(
            get_clock_color(5, &config).unwrap(),
            Color32::from_hex("#00cc00").unwrap()
        );
    }

    // Wrapping range: hour strictly between hour_end and hour_start does not match.
    #[test]
    fn wrap_range_middle_falls_back_to_default() {
        let config = config("#cccccc", vec![phase(22, 5, "#00cc00")]);
        assert_eq!(
            get_clock_color(6, &config).unwrap(),
            Color32::from_hex("#cccccc").unwrap()
        );
        assert_eq!(
            get_clock_color(12, &config).unwrap(),
            Color32::from_hex("#cccccc").unwrap()
        );
        assert_eq!(
            get_clock_color(21, &config).unwrap(),
            Color32::from_hex("#cccccc").unwrap()
        );
    }

    // When no phase matches, the default color is returned.
    #[test]
    fn no_matching_phase_returns_default() {
        let config = config("#cccccc", vec![phase(8, 15, "#3333cc")]);
        assert_eq!(
            get_clock_color(0, &config).unwrap(),
            Color32::from_hex("#cccccc").unwrap()
        );
    }

    // An empty phase list always returns the default color.
    #[test]
    fn empty_phases_returns_default() {
        let cfg = config("#cccccc", vec![]);
        assert_eq!(
            get_clock_color(12, &cfg).unwrap(),
            Color32::from_hex("#cccccc").unwrap()
        );

        // Invalid colors return an Err instead of panicking.
        let bad_default = config("not-a-color", vec![]);
        assert!(get_clock_color(12, &bad_default).is_err());
        let bad_phase = config("#cccccc", vec![phase(0, 23, "bad")]);
        assert!(get_clock_color(12, &bad_phase).is_err());
    }

    // The first matching phase wins; a non-matching earlier phase is skipped.
    #[test]
    fn later_phase_matches_when_earlier_phase_skipped() {
        let config = config(
            "#cccccc",
            vec![phase(0, 7, "#cc0000"), phase(8, 15, "#3333cc")],
        );
        // 12 is not in phase 1 (0..=7), so phase 2 (8..=15) matches.
        assert_eq!(
            get_clock_color(12, &config).unwrap(),
            Color32::from_hex("#3333cc").unwrap()
        );
        // 3 is in phase 1 (0..=7), so phase 1 wins.
        assert_eq!(
            get_clock_color(3, &config).unwrap(),
            Color32::from_hex("#cc0000").unwrap()
        );
    }

    fn named_phase(name: &str, start: u32, end: u32, color: &str) -> TimePhase {
        TimePhase {
            name: name.to_string(),
            hour_start: start,
            hour_end: end,
            color: color.to_string(),
        }
    }

    fn local_time(hour: u32, minute: u32) -> DateTime<Local> {
        // Interpret the naive datetime as a local wall-clock time, not UTC.
        // (January 15 is unambiguous in every timezone, no DST edge cases.)
        let date = NaiveDate::from_ymd_opt(2026, 1, 15).unwrap();
        let naive = NaiveDateTime::new(date, NaiveTime::from_hms_opt(hour, minute, 0).unwrap());
        Local.from_local_datetime(&naive).single().unwrap()
    }

    // An empty phase list has no next start moment.
    #[test]
    fn next_phase_empty_phases_returns_none() {
        assert!(next_phase_start(&local_time(12, 0), &config("#cccccc", vec![])).is_none());
    }

    // A phase later today is the closest start, even if a past phase is listed first.
    #[test]
    fn next_phase_later_today() {
        let config = config(
            "#cccccc",
            vec![
                named_phase("morning", 9, 14, "#3333cc"),
                named_phase("evening", 20, 23, "#cc0000"),
            ],
        );
        // 12:00 -> 20:00 is 8h away.
        assert_eq!(
            next_phase_start(&local_time(12, 0), &config),
            Some((8, 0, "evening".to_string()))
        );
    }

    // The earliest phase starting later today wins, even when the config is unsorted.
    #[test]
    fn next_phase_earliest_today_wins() {
        let config = config(
            "#cccccc",
            vec![
                named_phase("late", 20, 23, "#cc0000"),
                named_phase("mid", 14, 18, "#3333cc"),
            ],
        );
        // 12:30 -> 14:00 is 1h30m away.
        assert_eq!(
            next_phase_start(&local_time(12, 30), &config),
            Some((1, 30, "mid".to_string()))
        );
    }

    // A phase starting at the current hour is already started, so the next start is tomorrow.
    #[test]
    fn next_phase_current_hour_wraps_to_tomorrow() {
        let config = config("#cccccc", vec![named_phase("morning", 8, 14, "#3333cc")]);
        // 8:45 -> tomorrow 08:00 is 23h15m away.
        assert_eq!(
            next_phase_start(&local_time(8, 45), &config),
            Some((23, 15, "morning".to_string()))
        );
    }

    // All phases in the past wraps to tomorrow's earliest phase.
    #[test]
    fn next_phase_all_past_wraps_to_tomorrow() {
        let config = config(
            "#cccccc",
            vec![
                named_phase("dawn", 5, 9, "#3333cc"),
                named_phase("night", 22, 2, "#cc0000"),
            ],
        );
        // 23:15 -> 05:00 tomorrow is 5h45m away.
        assert_eq!(
            next_phase_start(&local_time(23, 15), &config),
            Some((5, 45, "dawn".to_string()))
        );
    }

    // When multiple phases share the same start hour, config order is preserved.
    #[test]
    fn next_phase_same_hour_keeps_config_order() {
        let config = config(
            "#cccccc",
            vec![
                named_phase("a", 15, 16, "#111111"),
                named_phase("b", 15, 16, "#222222"),
            ],
        );
        // 12:00 -> 15:00 is 3h away, and phase "a" wins the tie.
        assert_eq!(
            next_phase_start(&local_time(12, 0), &config),
            Some((3, 0, "a".to_string()))
        );
    }

    // Minutes count down from the current minute.
    #[test]
    fn next_phase_minutes_countdown() {
        let config = config("#cccccc", vec![named_phase("noon", 10, 14, "#3333cc")]);
        // 9:50 -> 10:00 is 10m away.
        assert_eq!(
            next_phase_start(&local_time(9, 50), &config),
            Some((0, 10, "noon".to_string()))
        );
    }
}
