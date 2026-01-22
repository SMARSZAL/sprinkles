use bevy::prelude::*;
use bevy_starling::{
    asset::ParticleSystemAsset, core::ParticleSystem3D, runtime::ParticleSystemRuntime,
};

use crate::state::EditorState;

#[derive(Component)]
pub struct EditorParticlePreview;

pub fn spawn_preview_particle_system(
    mut commands: Commands,
    editor_state: Res<EditorState>,
    assets: Res<Assets<ParticleSystemAsset>>,
    existing: Query<Entity, With<EditorParticlePreview>>,
) {
    let Some(handle) = &editor_state.current_project else {
        // no project loaded, despawn any existing preview
        for entity in existing.iter() {
            commands.entity(entity).despawn();
        }
        return;
    };

    // check if asset is loaded
    if assets.get(handle).is_none() {
        return;
    }

    // if we already have a preview for this project, skip
    if !existing.is_empty() {
        return;
    }

    // spawn the particle system preview
    commands.spawn((
        ParticleSystem3D {
            handle: handle.clone(),
        },
        Transform::default(),
        Visibility::default(),
        EditorParticlePreview,
        Name::new("Particle Preview"),
    ));
}

pub fn despawn_preview_on_project_change(
    mut commands: Commands,
    editor_state: Res<EditorState>,
    existing: Query<(Entity, &ParticleSystem3D), With<EditorParticlePreview>>,
) {
    if !editor_state.is_changed() {
        return;
    }

    for (entity, particle_system) in existing.iter() {
        // if the handle doesn't match the current project, despawn
        let should_despawn = match &editor_state.current_project {
            Some(handle) => particle_system.handle != *handle,
            None => true,
        };

        if should_despawn {
            commands.entity(entity).despawn();
        }
    }
}

pub fn sync_playback_state(
    editor_state: Res<EditorState>,
    mut query: Query<&mut ParticleSystemRuntime, With<EditorParticlePreview>>,
) {
    for mut runtime in query.iter_mut() {
        // sync emitting state with editor playback
        if runtime.emitting != editor_state.is_playing {
            runtime.emitting = editor_state.is_playing;
        }

        // if stopped (not playing), reset the system
        if !editor_state.is_playing && runtime.system_time > 0.0 {
            runtime.reset();
        }
    }
}
