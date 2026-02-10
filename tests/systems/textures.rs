use super::helpers::*;

use bevy::asset::Assets;
use bevy::prelude::*;
use sprinkles::textures::baked::{CurveTextureCache, GradientTextureCache};

fn create_texture_test_app() -> App {
    let mut app = create_minimal_app();
    app.init_asset::<Image>();
    app.init_resource::<GradientTextureCache>();
    app.init_resource::<CurveTextureCache>();
    app.update();
    app
}

#[test]
fn gradient_texture_cache_stores_textures() {
    let mut app = create_texture_test_app();

    let gradient = sprinkles::asset::Gradient {
        stops: vec![
            sprinkles::asset::GradientStop {
                color: [1.0, 0.0, 0.0, 1.0],
                position: 0.0,
            },
            sprinkles::asset::GradientStop {
                color: [0.0, 0.0, 1.0, 1.0],
                position: 1.0,
            },
        ],
        interpolation: sprinkles::asset::GradientInterpolation::Linear,
    };

    let handle = app
        .world_mut()
        .resource_scope(|world, mut cache: Mut<GradientTextureCache>| {
            let mut images = world.resource_mut::<Assets<Image>>();
            cache.get_or_create(&gradient, &mut images)
        });

    let cached = app
        .world()
        .resource::<GradientTextureCache>()
        .get(&gradient);
    assert!(cached.is_some(), "gradient should be cached");
    assert_eq!(cached.unwrap(), handle, "cached handle should match");
}

#[test]
fn curve_texture_cache_stores_textures() {
    let mut app = create_texture_test_app();

    let curve = sprinkles::asset::CurveTexture {
        name: None,
        points: vec![
            sprinkles::asset::CurvePoint::new(0.0, 1.0),
            sprinkles::asset::CurvePoint::new(1.0, 0.0),
        ],
        range: sprinkles::asset::Range::new(0.0, 1.0),
    };

    let handle = app
        .world_mut()
        .resource_scope(|world, mut cache: Mut<CurveTextureCache>| {
            let mut images = world.resource_mut::<Assets<Image>>();
            cache.get_or_create(&curve, &mut images)
        });

    let cached = app.world().resource::<CurveTextureCache>().get(&curve);
    assert!(cached.is_some(), "curve should be cached");
    assert_eq!(cached.unwrap(), handle, "cached handle should match");
}

#[test]
fn gradient_cache_different_gradients_get_different_handles() {
    let mut app = create_texture_test_app();

    let gradient_a = sprinkles::asset::Gradient {
        stops: vec![
            sprinkles::asset::GradientStop {
                color: [1.0, 0.0, 0.0, 1.0],
                position: 0.0,
            },
            sprinkles::asset::GradientStop {
                color: [0.0, 0.0, 1.0, 1.0],
                position: 1.0,
            },
        ],
        interpolation: sprinkles::asset::GradientInterpolation::Linear,
    };
    let gradient_b = sprinkles::asset::Gradient {
        stops: vec![
            sprinkles::asset::GradientStop {
                color: [0.0, 1.0, 0.0, 1.0],
                position: 0.0,
            },
            sprinkles::asset::GradientStop {
                color: [1.0, 1.0, 0.0, 1.0],
                position: 1.0,
            },
        ],
        interpolation: sprinkles::asset::GradientInterpolation::Linear,
    };

    let handle_a = app
        .world_mut()
        .resource_scope(|world, mut cache: Mut<GradientTextureCache>| {
            let mut images = world.resource_mut::<Assets<Image>>();
            cache.get_or_create(&gradient_a, &mut images)
        });
    let handle_b = app
        .world_mut()
        .resource_scope(|world, mut cache: Mut<GradientTextureCache>| {
            let mut images = world.resource_mut::<Assets<Image>>();
            cache.get_or_create(&gradient_b, &mut images)
        });

    assert_ne!(
        handle_a, handle_b,
        "different gradients should have different handles"
    );
}

#[test]
fn curve_cache_different_curves_get_different_handles() {
    let mut app = create_texture_test_app();

    let curve_a = sprinkles::asset::CurveTexture {
        name: None,
        points: vec![
            sprinkles::asset::CurvePoint::new(0.0, 1.0),
            sprinkles::asset::CurvePoint::new(1.0, 0.0),
        ],
        range: sprinkles::asset::Range::new(0.0, 1.0),
    };
    let curve_b = sprinkles::asset::CurveTexture {
        name: None,
        points: vec![
            sprinkles::asset::CurvePoint::new(0.0, 0.0),
            sprinkles::asset::CurvePoint::new(1.0, 1.0),
        ],
        range: sprinkles::asset::Range::new(0.0, 1.0),
    };

    let handle_a = app
        .world_mut()
        .resource_scope(|world, mut cache: Mut<CurveTextureCache>| {
            let mut images = world.resource_mut::<Assets<Image>>();
            cache.get_or_create(&curve_a, &mut images)
        });
    let handle_b = app
        .world_mut()
        .resource_scope(|world, mut cache: Mut<CurveTextureCache>| {
            let mut images = world.resource_mut::<Assets<Image>>();
            cache.get_or_create(&curve_b, &mut images)
        });

    assert_ne!(
        handle_a, handle_b,
        "different curves should have different handles"
    );
}
