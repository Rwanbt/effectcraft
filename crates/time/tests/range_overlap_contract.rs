//! Regression contracts for half-open time ranges used by composition and layer timelines.
//! Empty and reversed intervals represent no instants, regardless of their apparent endpoints.

use effectcraft_time::{Tick, TimeRange};

#[test]
fn empty_and_reversed_ranges_never_overlap_a_nonempty_range() {
    let filled = TimeRange::from_bounds(Tick(0), Tick(20));
    for empty in [TimeRange::new(Tick(10), Tick(0)), TimeRange::new(Tick(10), Tick(-5))] {
        assert!(empty.is_empty(), "zero or negative duration must be empty");
        assert!(!empty.contains(Tick(10)));
        assert!(!empty.overlaps(&filled), "an empty left operand cannot overlap");
        assert!(!filled.overlaps(&empty), "an empty right operand cannot overlap");
        assert_eq!(empty.intersect(&filled), None);
        assert_eq!(filled.intersect(&empty), None);
    }
}

#[test]
fn time_ranges_are_half_open_and_overlap_is_symmetric() {
    let a = TimeRange::from_bounds(Tick(1), Tick(8));
    let b = TimeRange::from_bounds(Tick(5), Tick(10));
    let touching = TimeRange::from_bounds(Tick(8), Tick(12));
    assert!(a.contains(Tick(1)));
    assert!(a.contains(Tick(7)));
    assert!(!a.contains(Tick(8)));
    assert!(a.overlaps(&b));
    assert!(b.overlaps(&a));
    assert_eq!(a.intersect(&b), Some(TimeRange::from_bounds(Tick(5), Tick(8))));
    assert!(!a.overlaps(&touching));
    assert!(!touching.overlaps(&a));
    assert_eq!(a.intersect(&touching), None);
}
