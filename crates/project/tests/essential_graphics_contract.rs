//! Essential Graphics contract tests independent of the engine's UI state.
//! Fixed expected orders and values prevent snapshot-to-snapshot false positives.

use effectcraft_project::LayerId;
use effectcraft_project::Value;
use effectcraft_project::essential::{self, ControlType, EgControl, EgKind, EgLink, EssentialGraphics};
use effectcraft_project::props::{ParamUi, Property};

fn property_control(id: u64, layer: u64, prop: u64) -> EgControl {
    EgControl { id, name: format!("Control {id}"), kind: EgKind::Property { layer: LayerId(layer), prop, links: vec![] }, as_type: None }
}

fn mirror(id: u64, of: u64) -> EgControl {
    EgControl { id, name: format!("Mirror {id}"), kind: EgKind::Mirror { of }, as_type: None }
}

#[test]
fn nested_control_mutation_removal_and_order_follow_tree_structure() {
    let group = EgControl {
        id: 1,
        name: "Typography".into(),
        kind: EgKind::Group {
            children: vec![property_control(2, 7, 11), EgControl { id: 3, name: "Note".into(), kind: EgKind::Comment { text: "Keep".into() }, as_type: None }],
        },
        as_type: None,
    };
    let mut graphics = EssentialGraphics { name: "Lower Third".into(), controls: vec![group, property_control(4, 8, 12)] };
    assert_eq!(graphics.flat().iter().map(|c| c.id).collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    assert_eq!(graphics.find(2).unwrap().name, "Control 2");
    assert!(graphics.find(123).is_none());
    assert!(graphics.list_mut(Some(123)).is_none());
    assert!(graphics.list_mut(Some(2)).is_none(), "a property control cannot contain children");
    graphics.find_mut(2).unwrap().name = "Title".into();
    graphics.list_mut(Some(1)).unwrap().push(mirror(5, 2));
    assert_eq!(graphics.find(2).unwrap().name, "Title");
    assert_eq!(graphics.flat().iter().map(|c| c.id).collect::<Vec<_>>(), vec![1, 2, 3, 5, 4]);
    assert_eq!(graphics.remove(3).unwrap().name, "Note");
    assert!(graphics.remove(3).is_none());
    assert_eq!(graphics.flat().iter().map(|c| c.id).collect::<Vec<_>>(), vec![1, 2, 5, 4]);
    assert!(graphics.has_instance_controls());
    assert_eq!(graphics.remove(1).unwrap().name, "Typography");
    assert!(graphics.find(2).is_none());
}

#[test]
fn long_mirror_chains_resolve_and_cycle_detection_terminates() {
    let mut graphics = EssentialGraphics { name: "Chains".into(), controls: vec![property_control(1, 10, 100)] };
    for id in 2..=21 {
        graphics.controls.push(mirror(id, id - 1));
    }
    assert_eq!(graphics.resolve(21).unwrap().id, 1, "more than eight valid mirrors must resolve");
    assert_eq!(graphics.mirrors_of(1), (2..=21).collect::<Vec<_>>());
    graphics.controls.push(mirror(22, 23));
    graphics.controls.push(mirror(23, 22));
    assert!(graphics.resolve(22).is_none(), "mirror cycles must not hang");
    assert!(graphics.resolve(23).is_none());
    assert!(graphics.resolve(999).is_none());
    graphics.controls.push(mirror(24, 999));
    assert!(graphics.resolve(24).is_none());
}

#[test]
fn control_lookup_drivers_links_and_types_are_independent() {
    let master = EgControl {
        id: 9,
        name: "Opacity".into(),
        kind: EgKind::Property { layer: LayerId(2), prop: 50, links: vec![EgLink { layer: LayerId(3), prop: 60 }] },
        as_type: None,
    };
    let second = EgControl { as_type: Some(ControlType::Scale), ..property_control(10, 2, 50) };
    let graphics = EssentialGraphics { name: String::new(), controls: vec![master.clone(), second, mirror(11, 9)] };
    assert_eq!(graphics.control_for(LayerId(2), 50).unwrap().id, 9);
    assert_eq!(graphics.control_for_as(LayerId(2), 50, Some(ControlType::Scale)).unwrap().id, 10);
    assert_eq!(graphics.driver_of(LayerId(3), 60).unwrap().id, 9);
    assert_eq!(graphics.driver_of(LayerId(2), 50).unwrap().id, 9);
    assert!(graphics.driver_of(LayerId(4), 60).is_none());
    assert_eq!(master.kind.targets(), vec![(LayerId(2), 50), (LayerId(3), 60)]);
    assert!(mirror(11, 9).kind.targets().is_empty());
    assert_eq!(graphics.resolve(11).unwrap().id, 9);
}

#[test]
fn comments_and_groups_alone_are_not_instance_controls() {
    let graphics = EssentialGraphics {
        name: String::new(),
        controls: vec![EgControl {
            id: 1,
            name: "Section".into(),
            kind: EgKind::Group {
                children: vec![EgControl { id: 2, name: "Help".into(), kind: EgKind::Comment { text: "No property".into() }, as_type: None }],
            },
            as_type: None,
        }],
    };
    assert!(!graphics.has_instance_controls());
    assert_eq!(graphics.flat().len(), 2);
    assert!(graphics.control_for(LayerId(1), 1).is_none());
}

#[test]
fn control_type_parser_roundtrips_and_rejects_invalid_names() {
    for (kind, expected) in [
        (ControlType::Text, "Text"),
        (ControlType::Color, "Color"),
        (ControlType::Slider, "Slider"),
        (ControlType::Checkbox, "Checkbox"),
        (ControlType::Point, "Point"),
        (ControlType::Angle, "Angle"),
        (ControlType::Dropdown, "Dropdown"),
        (ControlType::Media, "Media"),
        (ControlType::Font, "Font"),
        (ControlType::Scale, "Scale"),
    ] {
        assert_eq!(kind.label(), expected);
        assert_eq!(ControlType::parse(expected), Some(kind));
        assert_eq!(ControlType::parse(&format!("  {}  ", expected.to_ascii_lowercase())), Some(kind));
    }
    assert_eq!(ControlType::parse("unknown"), None);
    assert_eq!(ControlType::parse(""), None);
}

#[test]
fn property_exposure_enforces_hidden_types_and_compatible_controls() {
    let scalar = Property::new(1, "opacity", "Opacity", Value::Scalar(75.0));
    let vector = Property::new(2, "scale", "Scale", Value::Vec3([100.0, 100.0, 100.0]));
    let angle = Property::new(3, "rotation", "Rotation", Value::Scalar(45.0)).with_ui(ParamUi::Angle);
    let hidden = Property::new(4, "internal", "Internal", Value::Scalar(0.0)).with_ui(ParamUi::Hidden);
    let boolean = Property::new(5, "enabled", "Enabled", Value::Bool(true));
    let enumeration = Property::new(6, "mode", "Mode", Value::Enum(0));
    assert_eq!(essential::control_type(&scalar), Some(ControlType::Slider));
    assert_eq!(essential::control_type(&vector), Some(ControlType::Point));
    assert_eq!(essential::control_type(&angle), Some(ControlType::Angle));
    assert_eq!(essential::control_type(&boolean), Some(ControlType::Checkbox));
    assert_eq!(essential::control_type(&enumeration), Some(ControlType::Dropdown));
    assert_eq!(essential::control_type(&hidden), None);
    assert!(essential::can_show_as(&vector, ControlType::Scale));
    assert!(!essential::can_show_as(&angle, ControlType::Scale));
    assert!(!essential::can_show_as(&hidden, ControlType::Slider));
    assert!(!essential::can_show_as(&vector, ControlType::Media));
    assert!(essential::linkable(&scalar, &Property::new(7, "other", "Other", Value::Scalar(50.0))));
    assert!(!essential::linkable(&scalar, &boolean));
    assert_eq!(essential::effective_type(&EgControl { as_type: Some(ControlType::Scale), ..property_control(8, 1, 2) }, &vector), Some(ControlType::Scale));
    assert_eq!(essential::effective_type(&EgControl { as_type: Some(ControlType::Scale), ..property_control(8, 1, 1) }, &scalar), Some(ControlType::Slider));
}

#[test]
fn instance_control_identifier_parsing_rejects_malformed_and_overflowing_ids() {
    for id in [0, 1, 99, 100_001, u64::MAX] {
        assert_eq!(essential::control_of(&essential::match_id(id)), Some(id));
    }
    for invalid in ["", "eg", "EG1", "eg-1", "egfoo", "eg18446744073709551616"] {
        assert_eq!(essential::control_of(invalid), None, "{invalid}");
    }
}
