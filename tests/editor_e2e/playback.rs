use super::helpers::*;

use sprinkles::{EmitterRuntime, ParticleSystemRuntime};

#[test]
fn test_play_starts_particle_system() {
    let mut runtime = ParticleSystemRuntime {
        paused: true,
        force_loop: true,
        global_seed: 0,
    };
    assert!(runtime.paused);

    runtime.resume();

    assert!(!runtime.paused);
}

#[test]
fn test_pause_stops_particle_system() {
    let mut runtime = ParticleSystemRuntime {
        paused: false,
        force_loop: true,
        global_seed: 0,
    };
    assert!(!runtime.paused);

    runtime.pause();

    assert!(runtime.paused);
}

#[test]
fn test_stop_resets_particle_system() {
    let mut emitter_runtime = EmitterRuntime::new(0, None);
    emitter_runtime.system_time = 5.0;
    emitter_runtime.emitting = true;

    emitter_runtime.stop(None);

    assert_eq!(emitter_runtime.system_time, 0.0);
    assert!(!emitter_runtime.emitting);
}

#[test]
fn test_loop_toggle() {
    let mut runtime = ParticleSystemRuntime {
        paused: false,
        force_loop: true,
        global_seed: 0,
    };
    assert!(runtime.force_loop);

    runtime.force_loop = false;
    assert!(!runtime.force_loop);

    runtime.force_loop = true;
    assert!(runtime.force_loop);
}

#[test]
fn test_seek_updates_time() {
    let mut emitter_runtime = EmitterRuntime::new(0, None);
    assert_eq!(emitter_runtime.system_time, 0.0);

    emitter_runtime.seek(2.5);

    assert_eq!(emitter_runtime.system_time, 2.5);
    assert_eq!(emitter_runtime.prev_system_time, 2.5);
}

#[test]
fn test_elapsed_time_advances() {
    let mut app = create_minimal_app();

    let handle = load_fixture(&mut app, "minimal_particle_system.ron");
    assert!(run_until_loaded(&mut app, &handle, 100));
    spawn_3d_particle_system(&mut app, handle);

    advance_time(&mut app, 0.2);

    let mut found = false;
    for runtime in app.world_mut().query::<&EmitterRuntime>().iter(app.world()) {
        assert!(
            runtime.system_time > 0.0,
            "system_time should advance, got {}",
            runtime.system_time
        );
        found = true;
    }
    assert!(found, "should find at least one EmitterRuntime");
}
