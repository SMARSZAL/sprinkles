use super::helpers::*;

use bevy::asset::AssetServer;
use bevy::prelude::*;
use sprinkles::asset::ParticleSystemAsset;

#[test]
fn sprinkles_plugin_registers_asset_type() {
    let mut app = create_minimal_app();
    app.update();

    // if ParticleSystemAsset wasn't registered, this load would panic
    let asset_server = app.world().resource::<AssetServer>();
    let handle: Handle<ParticleSystemAsset> =
        asset_server.load("minimal_particle_system.ron".to_string());

    assert!(
        run_until_loaded(&mut app, &handle, 100),
        "should load ParticleSystemAsset - asset type is registered"
    );
}

#[test]
fn plugin_smoke_test_no_panics() {
    let mut app = create_minimal_app();
    let handle = load_fixture(&mut app, "minimal_particle_system.ron");
    spawn_3d_particle_system(&mut app, handle.clone());

    assert!(run_until_loaded(&mut app, &handle, 100));

    // just run many frames without panicking
    advance_frames(&mut app, 50);
}

#[test]
fn plugin_handles_multiple_systems() {
    let mut app = create_minimal_app();
    let handle_a = load_fixture(&mut app, "minimal_particle_system.ron");
    let handle_b = load_fixture(&mut app, "two_emitters.ron");
    spawn_3d_particle_system(&mut app, handle_a.clone());
    spawn_3d_particle_system(&mut app, handle_b.clone());

    assert!(run_until_loaded(&mut app, &handle_a, 100));
    assert!(run_until_loaded(&mut app, &handle_b, 100));

    advance_frames(&mut app, 20);

    let emitter_count = app
        .world_mut()
        .query::<&sprinkles::runtime::EmitterRuntime>()
        .iter(app.world())
        .len();
    // 1 emitter from minimal + 2 from two_emitters = 3
    assert_eq!(emitter_count, 3, "should have 3 emitters across 2 systems");
}

#[test]
fn plugin_handles_system_with_all_features() {
    let mut app = create_minimal_app();
    let handle = load_fixture(&mut app, "maximal_emitter.ron");
    spawn_3d_particle_system(&mut app, handle.clone());

    assert!(run_until_loaded(&mut app, &handle, 100));

    // run many frames - should not panic even with all features enabled
    advance_frames(&mut app, 30);
}
