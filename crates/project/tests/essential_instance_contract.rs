//! Cross-layer Essential Graphics contracts: live instance propagation, immutable render
//! overrides, and template ID remapping. No screenshots or GPU are required.

use effectcraft_color::Label;
use effectcraft_project::essential::{self, EgControl, EgKind, EssentialGraphics, Override};
use effectcraft_project::{Comp, ItemId, ItemKind, LayerId, LayerSource, Project, Value, build};
use effectcraft_time::{FrameRate, Tick};

struct Fixture {
    project: Project,
    source: ItemId,
    parent: ItemId,
    source_layer: LayerId,
    instance_layer: LayerId,
    source_property: u64,
    master: u64,
    mirror: u64,
}

fn fixture() -> Fixture {
    let mut project = Project::default();
    let comp = Comp::new(64, 48, FrameRate::FPS_24, Tick::from_seconds_f64(2.0));
    let source = project.add_item("Nested", Label::Red, None, ItemKind::Comp(comp.clone().into()));
    let parent = project.add_item("Parent", Label::Blue, None, ItemKind::Comp(comp.clone().into()));
    let shape = build::layer(&mut project, &comp, "Shape", LayerSource::Shape, (64, 48), None);
    let source_layer = shape.id;
    let source_property = shape.props.prop("transform/opacity").unwrap().uid;
    project.comp_mut(source).unwrap().layers.push(shape);
    let master = project.alloc();
    let mirror = project.alloc();
    project.comp_mut(source).unwrap().essential = Some(EssentialGraphics {
        name: "Nested Controls".into(),
        controls: vec![
            EgControl {
                id: master,
                name: "Opacity".into(),
                kind: EgKind::Property { layer: source_layer, prop: source_property, links: vec![] },
                as_type: None,
            },
            EgControl { id: mirror, name: "Alias".into(), kind: EgKind::Mirror { of: master }, as_type: None },
        ],
    });
    let precomp = build::layer(&mut project, &comp, "Nested", LayerSource::Comp { item: source }, (64, 48), None);
    let instance_layer = precomp.id;
    project.comp_mut(parent).unwrap().layers.push(precomp);
    Fixture { project, source, parent, source_layer, instance_layer, source_property, master, mirror }
}

fn instance<'a>(p: &'a Project, f: &Fixture) -> &'a effectcraft_project::props::PropGroup {
    essential::group(p.comp(f.parent).unwrap().layer(f.instance_layer).unwrap()).unwrap()
}

#[test]
fn instance_sync_preserves_overrides_and_tracks_aliases() {
    let f = fixture();
    let mut p = f.project.clone();
    let before = p.clone();
    essential::sync_project(&before, &mut p);
    let master = essential::match_id(f.master);
    let mirror = essential::match_id(f.mirror);
    assert_eq!(instance(&p, &f).get(&master).unwrap().value, Value::Scalar(100.0));
    assert_eq!(instance(&p, &f).get(&mirror).unwrap().value, Value::Scalar(100.0));
    assert!(essential::overridden(p.comp(f.parent).unwrap().layer(f.instance_layer).unwrap()).is_empty());

    let before = p.clone();
    let layer = p.comp_mut(f.parent).unwrap().layer_mut(f.instance_layer).unwrap();
    layer.props.sub_mut(essential::GROUP).unwrap().get_mut(&master).unwrap().value = Value::Scalar(55.0);
    essential::sync_project(&before, &mut p);
    assert_eq!(instance(&p, &f).get(&master).unwrap().value, Value::Scalar(55.0));
    assert_eq!(instance(&p, &f).get(&mirror).unwrap().value, Value::Scalar(55.0));
    let over = essential::overridden(p.comp(f.parent).unwrap().layer(f.instance_layer).unwrap());
    assert!(over.contains(&instance(&p, &f).get(&master).unwrap().uid));

    let before = p.clone();
    p.comp_mut(f.source).unwrap().layer_mut(f.source_layer).unwrap().props.find_mut(f.source_property).unwrap().value = Value::Scalar(80.0);
    essential::sync_project(&before, &mut p);
    assert_eq!(instance(&p, &f).get(&master).unwrap().value, Value::Scalar(55.0), "changing the source must not reset an overridden instance");
    assert_eq!(instance(&p, &f).get(&mirror).unwrap().value, Value::Scalar(55.0));
}

#[test]
fn rendering_overrides_do_not_modify_the_saved_project() {
    let f = fixture();
    let original = &f.project;
    let alias = Override { control: f.mirror, value: Value::Scalar(22.0) };
    let rendered = essential::with_overrides(original, f.source, &[alias]).unwrap();
    let source_value = |p: &Project| p.comp(f.source).unwrap().layer(f.source_layer).unwrap().props.find(f.source_property).unwrap().value.clone();
    assert_eq!(source_value(original), Value::Scalar(100.0), "rendering cannot mutate the master");
    assert_eq!(source_value(&rendered), Value::Scalar(22.0), "mirror override targets the original property");
    assert!(essential::with_overrides(original, f.source, &[Override { control: u64::MAX, value: Value::Scalar(0.0) }]).is_none());
    assert!(essential::with_overrides(original, f.parent, &[Override { control: f.master, value: Value::Scalar(0.0) }]).is_none());
}

#[test]
fn dependency_discovery_and_template_id_shift_keep_links_valid() {
    let f = fixture();
    let mut project = f.project.clone();
    let before = project.clone();
    essential::sync_project(&before, &mut project);
    assert_eq!(essential::dependencies(&project, f.parent), vec![f.source, f.parent]);
    assert!(essential::dependencies(&project, ItemId(12345678)).is_empty());

    let offset = 1000;
    essential::offset_ids(&mut project, offset);
    let parent = ItemId(f.parent.0 + offset);
    let source = ItemId(f.source.0 + offset);
    assert!(project.item(parent).is_some());
    assert!(project.item(source).is_some());
    assert_eq!(essential::dependencies(&project, parent), vec![source, parent]);
    let controls = project.comp(source).unwrap().essential.as_ref().unwrap();
    let master = controls.find(f.master + offset).unwrap();
    let alias = controls.find(f.mirror + offset).unwrap();
    assert!(matches!(alias.kind, EgKind::Mirror { of } if of == f.master + offset));
    let (layer, prop) = essential::source_prop(&project, source, master).unwrap();
    assert_eq!(layer.id, LayerId(f.source_layer.0 + offset));
    assert_eq!(prop.uid, f.source_property + offset);
    let nested = project.comp(parent).unwrap().layer(LayerId(f.instance_layer.0 + offset)).unwrap();
    assert!(essential::group(nested).unwrap().get(&essential::match_id(f.master + offset)).is_some());
}
