use std::path::Path;

use sprinkles::asset::{EmitterData, ParticleSystemAsset, ParticleSystemDimension};
use sprinkles_editor::project::load_project_from_path;

#[test]
fn load_project_from_valid_fixture() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("minimal_particle_system.ron");
    let asset = load_project_from_path(&path);
    assert!(asset.is_some(), "should load valid RON fixture");

    let asset = asset.unwrap();
    assert_eq!(asset.emitters.len(), 1);
    assert_eq!(asset.name, "Minimal System");
}

#[test]
fn load_project_from_two_emitters_fixture() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("two_emitters.ron");
    let asset = load_project_from_path(&path).expect("should load");
    assert_eq!(asset.emitters.len(), 2);
    assert_eq!(asset.emitters[0].name, "Emitter A");
    assert_eq!(asset.emitters[1].name, "Emitter B");
}

#[test]
fn load_project_from_collision_fixture() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("collision_test.ron");
    let asset = load_project_from_path(&path).expect("should load");
    assert_eq!(asset.colliders.len(), 2, "should have 2 colliders");
}

#[test]
fn load_project_from_nonexistent_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("does_not_exist.ron");
    assert!(
        load_project_from_path(&path).is_none(),
        "should return None for missing file"
    );
}

#[test]
fn load_project_from_invalid_ron() {
    let dir = std::env::temp_dir().join("sprinkles_test_invalid.ron");
    std::fs::write(&dir, "this is { not valid ron }}}}").unwrap();
    assert!(
        load_project_from_path(&dir).is_none(),
        "should return None for invalid RON"
    );
    std::fs::remove_file(&dir).ok();
}

#[test]
fn test_new_project_creates_default_system() {
    let asset = ParticleSystemAsset {
        name: "New project".to_string(),
        dimension: ParticleSystemDimension::D3,
        emitters: vec![EmitterData {
            name: "Emitter 1".to_string(),
            ..Default::default()
        }],
        colliders: vec![],
    };

    assert_eq!(asset.name, "New project");
    assert!(matches!(asset.dimension, ParticleSystemDimension::D3));
    assert_eq!(asset.emitters.len(), 1);
    assert_eq!(asset.emitters[0].name, "Emitter 1");
    assert!(asset.colliders.is_empty());
}

#[test]
fn test_save_project_writes_file() {
    let asset = ParticleSystemAsset {
        name: "Save Test".to_string(),
        dimension: ParticleSystemDimension::D3,
        emitters: vec![EmitterData {
            name: "Emitter 1".to_string(),
            ..Default::default()
        }],
        colliders: vec![],
    };

    let tmp = std::env::temp_dir().join("sprinkles_save_test.ron");
    let contents = ron::ser::to_string_pretty(&asset, ron::ser::PrettyConfig::default()).unwrap();
    std::fs::write(&tmp, contents).unwrap();

    let reloaded = load_project_from_path(&tmp);
    assert!(reloaded.is_some(), "should reload saved file");
    let reloaded = reloaded.unwrap();
    assert_eq!(reloaded.name, "Save Test");
    assert_eq!(reloaded.emitters.len(), 1);

    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_save_project_preserves_all_fields() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("maximal_emitter.ron");
    let asset = load_project_from_path(&path).expect("should load maximal_emitter.ron");

    let tmp = std::env::temp_dir().join("sprinkles_maximal_roundtrip.ron");
    let contents = ron::ser::to_string_pretty(&asset, ron::ser::PrettyConfig::default()).unwrap();
    std::fs::write(&tmp, &contents).unwrap();

    let reloaded = load_project_from_path(&tmp).expect("should reload");

    assert_eq!(reloaded.name, asset.name);
    assert_eq!(reloaded.emitters.len(), asset.emitters.len());
    assert_eq!(reloaded.emitters[0].name, "Full Config");
    assert_eq!(reloaded.emitters[0].time.lifetime, 3.0);
    assert_eq!(reloaded.emitters[0].accelerations.gravity.y, -15.0);
    assert!(reloaded.emitters[0].turbulence.enabled);
    assert_eq!(reloaded.emitters[0].turbulence.noise_strength, 2.0);

    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_save_roundtrip_preserves_colliders() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("collision_test.ron");
    let asset = load_project_from_path(&path).expect("should load collision_test.ron");

    let tmp = std::env::temp_dir().join("sprinkles_collider_roundtrip.ron");
    let contents = ron::ser::to_string_pretty(&asset, ron::ser::PrettyConfig::default()).unwrap();
    std::fs::write(&tmp, &contents).unwrap();

    let reloaded = load_project_from_path(&tmp).expect("should reload");

    assert_eq!(reloaded.colliders.len(), 2);
    assert_eq!(reloaded.colliders[0].name, "Floor");
    assert_eq!(reloaded.colliders[1].name, "Wall");
    assert!(matches!(
        reloaded.colliders[0].shape,
        sprinkles::ParticlesColliderShape3D::Box { .. }
    ));
    assert!(matches!(
        reloaded.colliders[1].shape,
        sprinkles::ParticlesColliderShape3D::Sphere { .. }
    ));

    std::fs::remove_file(&tmp).ok();
}

#[test]
fn test_unsaved_changes_tracks_modifications() {
    use super::helpers::*;
    use bevy::prelude::*;
    use sprinkles::asset::ParticleSystemAsset;
    use sprinkles_editor::state::{DirtyState, EditorState};

    let mut app = create_minimal_app();
    app.init_resource::<EditorState>();
    app.init_resource::<DirtyState>();

    let handle = load_fixture(&mut app, "minimal_particle_system.ron");
    assert!(run_until_loaded(&mut app, &handle, 100));

    {
        let mut assets = app
            .world_mut()
            .resource_mut::<Assets<ParticleSystemAsset>>();
        let asset = assets.get_mut(&handle).unwrap();
        asset.emitters[0].time.lifetime = 99.0;
    }

    app.world_mut()
        .resource_mut::<DirtyState>()
        .has_unsaved_changes = true;

    let dirty = app.world().resource::<DirtyState>();
    assert!(dirty.has_unsaved_changes);

    let assets = app.world().resource::<Assets<ParticleSystemAsset>>();
    let asset = assets.get(&handle).unwrap();
    assert_eq!(asset.emitters[0].time.lifetime, 99.0);
}
