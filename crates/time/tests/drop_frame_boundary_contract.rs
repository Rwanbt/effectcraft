//! Deterministic boundary oracles for SMPTE drop-frame rates, including 119.88 fps.
//! No generated snapshots: expected labels follow the SMPTE minute/ten-minute counting rule.

use effectcraft_time::{FrameRate, format_timecode_frames, parse_timecode};

#[test]
fn drop_frame_skips_correct_labels_at_the_first_minute() {
    for (rate, dropped) in [(FrameRate::FPS_29_97, 2), (FrameRate::FPS_59_94, 4), (FrameRate::FPS_119_88, 8)] {
        let base = rate.timecode_base();
        let first_minute = base * 60;
        let width = if base >= 100 { 3 } else { 2 };
        assert_eq!(format_timecode_frames(first_minute - 1, rate, true), format!("00;00;59;{:0width$}", base - 1), "{rate}: last label before first minute");
        let first_label = format!("00;01;00;{dropped:0width$}");
        assert_eq!(format_timecode_frames(first_minute, rate, true), first_label, "{rate}: skipped labels at first minute");
        assert_eq!(parse_timecode(&first_label, rate, true, 0), Ok(first_minute), "{rate}: parse first-minute boundary");
    }
}

#[test]
fn drop_frame_ten_minute_hour_and_negative_boundaries_round_trip() {
    for (rate, dropped) in [(FrameRate::FPS_29_97, 2_i64), (FrameRate::FPS_59_94, 4), (FrameRate::FPS_119_88, 8)] {
        let base = rate.timecode_base();
        let width = if base >= 100 { 3 } else { 2 };
        let ten_minutes = base * 600 - dropped * 9;
        let hour = base * 3600 - dropped * 54;
        assert_eq!(format_timecode_frames(ten_minutes, rate, true), format!("00;10;00;{:0width$}", 0));
        assert_eq!(format_timecode_frames(hour, rate, true), format!("01;00;00;{:0width$}", 0));
        for frame in [0, base * 60 - 1, base * 60, base * 60 + 1, ten_minutes - 1, ten_minutes, hour - 1, hour, -base * 60, -ten_minutes, -hour] {
            let label = format_timecode_frames(frame, rate, true);
            assert_eq!(parse_timecode(&label, rate, true, 0), Ok(frame), "{rate}: label {label} must round-trip");
            assert_eq!(rate.frame_at(rate.tick_of(frame)), frame, "{rate}: exact tick/frame boundary for {frame}");
        }
    }
}
