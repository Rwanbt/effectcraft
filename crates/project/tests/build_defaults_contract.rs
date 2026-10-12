//! Fixed property-tree oracles for the default project layer builders.

use effectcraft_keyframe::{ShapePath, Value};
use effectcraft_project::build::{self, Ids};
use effectcraft_project::props::{GroupKind, MaskMode, ParamUi};

#[test]
fn transform_has_stable_defaults_and_distinct_monotone_uids() {
    let mut next = 100;
    let mut ids = Ids(&mut next);
    let transform = build::transform(&mut ids, [8.0, 9.0], [100.0, 200.0]);
    assert_eq!(transform.match_id, "transform");
    assert_eq!(transform.get("anchor").unwrap().value, Value::Vec3([8.0, 9.0, 0.0]));
    assert_eq!(transform.get("position").unwrap().value, Value::Vec3([100.0, 200.0, 0.0]));
    assert_eq!(transform.get("scale").unwrap().value, Value::Vec3([100.0, 100.0, 100.0]));
    assert_eq!(transform.get("opacity").unwrap().value, Value::Scalar(100.0));
    assert_eq!(transform.get("rotation").unwrap().value, Value::Scalar(0.0));
    assert_eq!(transform.get("position").unwrap().shown_dims, 2);
    assert!(transform.get("position").unwrap().spatial);
    assert!(transform.get("rotationX").unwrap().three_d_only);
    assert!(matches!(transform.get("opacity").unwrap().ui, ParamUi::Slider { min: 0.0, max: 100.0, .. }));
    let mut uids: Vec<_> = transform.children.iter().map(|node| node.uid()).collect();
    uids.push(transform.uid);
    uids.sort_unstable();
    uids.dedup();
    assert_eq!(uids.len(), 9, "all eight props and the group need distinct uids");
    assert_eq!(next, 109);
}

#[test]
fn mask_builder_preserves_mode_color_and_default_controls() {
    let mut next = 1;
    let mut ids = Ids(&mut next);
    let path = ShapePath::rect([0.0, 0.0], 8.0, 5.0);
    let mask = build::mask(&mut ids, "Cutout", path.clone(), MaskMode::Subtract, [7, 21, 35]);
    assert_eq!(mask.name, "Cutout");
    assert!(matches!(mask.kind, GroupKind::Mask { mode: MaskMode::Subtract, color: [7, 21, 35], inverted: false, locked: false, .. }));
    assert_eq!(mask.get("path").unwrap().value, Value::Path(path));
    assert_eq!(mask.get("feather").unwrap().value, Value::Vec2([0.0, 0.0]));
    assert_eq!(mask.get("opacity").unwrap().value, Value::Scalar(100.0));
    assert_eq!(mask.get("expansion").unwrap().value, Value::Scalar(0.0));
}

#[test]
fn text_selector_builders_enforce_valid_kinds_and_defaults() {
    let mut next = 10;
    let mut ids = Ids(&mut next);
    assert!(build::text_selector(&mut ids, "invalid", "Unknown").is_none());
    assert_eq!(next, 10, "unknown selector must not consume a UID");
    let range = build::text_selector(&mut ids, "range", "Range Selector 1").unwrap();
    assert_eq!(range.match_id, "rangeSelector");
    assert_eq!(range.get("start").unwrap().value, Value::Scalar(0.0));
    assert_eq!(range.get("end").unwrap().value, Value::Scalar(100.0));
    assert_eq!(range.sub("advanced").unwrap().get("basedOn").unwrap().value, Value::Enum(0));
    let wiggle = build::text_selector(&mut ids, "wiggly", "Wiggle").unwrap();
    assert_eq!(wiggle.match_id, "wigglySelector");
    assert_eq!(wiggle.get("minAmount").unwrap().value, Value::Scalar(-100.0));
    let expr = build::text_selector(&mut ids, "expression", "Expression").unwrap();
    assert_eq!(expr.match_id, "expressionSelector");
    assert_eq!(expr.get("amount").unwrap().expr.as_ref().unwrap().text, build::EXPRESSION_SELECTOR_DEFAULT);
    assert!(expr.get("amount").unwrap().has_expression());
}

#[test]
fn text_path_options_and_masks_defaults_are_unambiguous() {
    let mut next = 1;
    let mut ids = Ids(&mut next);
    let path = build::path_options(&mut ids);
    assert_eq!(path.get("path").unwrap().value, Value::Enum(0));
    assert!(path.get("path").unwrap().static_only);
    assert_eq!(path.get("perpendicular").unwrap().value, Value::Bool(true));
    assert_eq!(path.get("reversePath").unwrap().value, Value::Bool(false));
    let more = build::more_options(&mut ids);
    assert_eq!(more.get("anchorGrouping").unwrap().value, Value::Enum(0));
    assert!(more.get("interCharBlend").unwrap().static_only);
    assert_eq!(build::masks(&mut ids).match_id, "masks");
    assert_eq!(build::effects(&mut ids).match_id, "effects");
    assert_eq!(build::MASK_COLORS.len(), 8);
    assert_eq!(build::MASK_COLORS[0], [0xb1, 0xb4, 0x4f]);
}
