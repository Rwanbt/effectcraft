//! Project-file integrity contracts with fixed, independent error and graph oracles.
//! In particular, user-supplied JSON must not overflow identifiers or recurse forever.

use effectcraft_color::Label;
use effectcraft_project::{Comp, ItemId, ItemKind, LayerSource, Project, ProjectError, build};
use effectcraft_time::{FrameRate, Tick};

fn comp() -> Comp {
    Comp::new(64, 48, FrameRate::FPS_24, Tick::from_seconds_f64(2.0))
}

#[test]
fn cyclic_precomps_do_not_overflow_and_reachability_is_symmetric() {
    let mut project = Project::default();
    let a = project.add_item("A", Label::Red, None, ItemKind::Comp(comp().into()));
    let b = project.add_item("B", Label::Blue, None, ItemKind::Comp(comp().into()));
    let c = project.add_item("Isolated", Label::Green, None, ItemKind::Comp(comp().into()));
    let a_to_b = build::layer(&mut project, &comp(), "B", LayerSource::Comp { item: b }, (64, 48), None);
    let b_to_a = build::layer(&mut project, &comp(), "A", LayerSource::Comp { item: a }, (64, 48), None);
    project.comp_mut(a).unwrap().layers.push(a_to_b);
    project.comp_mut(b).unwrap().layers.push(b_to_a);

    assert!(project.comp_contains(a, b));
    assert!(project.comp_contains(b, a));
    assert!(project.comp_contains(a, a));
    assert!(!project.comp_contains(a, c), "cycle must terminate when the target is absent");
    assert!(!project.comp_contains(b, c));
    assert!(!project.comp_contains(c, a));
    assert!(!project.comp_contains(a, ItemId(9_999)));
}

#[test]
fn project_json_roundtrips_and_repairs_the_id_allocator() {
    let mut project = Project::default();
    let folder = project.add_item("Artwork", Label::Red, None, ItemKind::Folder);
    let child = project.add_item("Comp", Label::Green, Some(folder), ItemKind::Comp(comp().into()));
    let before = project.to_file_json().unwrap();
    assert_eq!(effectcraft_project::saved_by(&before).as_deref(), Some(effectcraft_project::APP_VERSION));
    let read = Project::from_json(&before).unwrap();
    assert_eq!(project, read);
    assert!(read.item(child).is_some());

    project.next_id = 1; // stale counter from an old or damaged project.
    let repaired = Project::from_json(&project.to_json()).unwrap();
    assert!(repaired.next_id > child.0);
    let mut edited = repaired.clone();
    let new_id = edited.add_item("New", Label::Blue, None, ItemKind::Folder);
    assert_ne!(new_id, child);
    assert!(edited.item(child).is_some());
    assert!(edited.item(new_id).is_some());
}

#[test]
fn malformed_and_truncated_project_files_return_errors() {
    let mut project = Project::default();
    project.add_item("Dessin", Label::Green, None, ItemKind::Folder);
    let json = project.to_file_json().unwrap();
    for (cut, _) in json.char_indices().skip(1) {
        assert!(Project::from_json(&json[..cut]).is_err(), "truncation at offset {cut} must fail");
    }
    for malformed in ["", "null", "[]", "{", "{\"schema\":-1}", "{\"schema\":9999999}"] {
        assert!(Project::from_json(malformed).is_err(), "invalid JSON must be refused: {malformed}");
    }
}

#[test]
fn exhausted_ids_fail_with_an_actionable_error_not_a_panic() {
    let project = Project { next_id: u64::MAX, ..Project::default() };
    let err = Project::from_json(&project.to_json()).unwrap_err();
    assert!(matches!(err, ProjectError::Invalid(message) if message.contains("id space exhausted")));

    let mut project = Project::default();
    let original = project.add_item("MaxId", Label::Blue, None, ItemKind::Folder);
    let mut item = project.items.remove(&original).unwrap();
    item.id = ItemId(u64::MAX);
    project.items.insert(item.id, item);
    let err = Project::from_json(&project.to_json()).unwrap_err();
    assert!(matches!(err, ProjectError::Invalid(message) if message.contains("id space exhausted")));

    // Public normalization itself must never panic on untrusted values.
    project.fix_next_id();
    assert_eq!(project.next_id, u64::MAX);
}

#[test]
fn unknown_schema_is_not_silently_downgraded() {
    let project = Project { schema: effectcraft_project::SCHEMA_VERSION + 1, ..Project::default() };
    let err = Project::from_json(&project.to_json()).unwrap_err();
    assert!(matches!(err, ProjectError::Invalid(message) if message.contains("newer")));
}
