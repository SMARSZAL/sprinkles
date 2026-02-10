use sprinkles::asset::*;

// --- Range ---

#[test]
fn range_default() {
    let range = Range::default();
    assert_eq!(range.min, 0.0);
    assert_eq!(range.max, 1.0);
}

#[test]
fn range_with_values() {
    let range = Range::new(2.0, 5.0);
    assert_eq!(range.min, 2.0);
    assert_eq!(range.max, 5.0);
}

#[test]
fn range_span() {
    let range = Range::new(1.0, 4.0);
    assert_eq!(range.span(), 3.0);
}

#[test]
fn range_span_zero_returns_one() {
    let range = Range::new(5.0, 5.0);
    assert_eq!(range.span(), 1.0, "span of zero should return 1.0");
}

// --- CurveTexture ---

#[test]
fn curve_sample_edges() {
    let curve = CurveTexture {
        name: None,
        points: vec![
            CurvePoint::new(0.0, 0.0),
            CurvePoint::new(1.0, 1.0),
        ],
        range: Range::new(0.0, 1.0),
    };
    let at_zero = curve.sample(0.0);
    let at_one = curve.sample(1.0);
    assert!((at_zero - 0.0).abs() < 0.01, "sample at t=0 should be ~0, got {at_zero}");
    assert!((at_one - 1.0).abs() < 0.01, "sample at t=1 should be ~1, got {at_one}");
}

#[test]
fn curve_sample_midpoint() {
    let curve = CurveTexture {
        name: None,
        points: vec![
            CurvePoint::new(0.0, 0.0),
            CurvePoint::new(1.0, 1.0),
        ],
        range: Range::new(0.0, 1.0),
    };
    let mid = curve.sample(0.5);
    // default mode is DoubleCurve with default tension, so midpoint should be ~0.5
    assert!(
        (mid - 0.5).abs() < 0.1,
        "sample at t=0.5 should be ~0.5, got {mid}"
    );
}

#[test]
fn curve_sample_empty_returns_one() {
    let curve = CurveTexture {
        name: None,
        points: vec![],
        range: Range::new(0.0, 1.0),
    };
    assert_eq!(curve.sample(0.5), 1.0);
}

#[test]
fn curve_sample_single_point() {
    let curve = CurveTexture {
        name: None,
        points: vec![CurvePoint::new(0.5, 0.75)],
        range: Range::new(0.0, 1.0),
    };
    assert_eq!(curve.sample(0.0), 0.75);
    assert_eq!(curve.sample(1.0), 0.75);
}

#[test]
fn curve_hold_mode() {
    let curve = CurveTexture {
        name: None,
        points: vec![
            CurvePoint::new(0.0, 1.0),
            CurvePoint::new(1.0, 0.0).with_mode(CurveMode::Hold),
        ],
        range: Range::new(0.0, 1.0),
    };
    // hold mode returns left value (no interpolation)
    let mid = curve.sample(0.5);
    assert!(
        (mid - 1.0).abs() < 0.01,
        "hold mode should stay at left value, got {mid}"
    );
}

#[test]
fn curve_is_constant() {
    let constant = CurveTexture {
        name: None,
        points: vec![
            CurvePoint::new(0.0, 1.0),
            CurvePoint::new(1.0, 1.0),
        ],
        range: Range::new(0.0, 1.0),
    };
    assert!(constant.is_constant());

    let varying = CurveTexture {
        name: None,
        points: vec![
            CurvePoint::new(0.0, 1.0),
            CurvePoint::new(1.0, 0.0),
        ],
        range: Range::new(0.0, 1.0),
    };
    assert!(!varying.is_constant());
}

#[test]
fn curve_default() {
    let curve = CurveTexture::default();
    assert_eq!(curve.points.len(), 2);
    assert!(curve.is_constant(), "default curve should be constant");
    assert_eq!(curve.sample(0.0), 1.0);
    assert_eq!(curve.sample(1.0), 1.0);
}

#[test]
fn curve_cache_key_differs_for_different_curves() {
    let curve_a = CurveTexture {
        name: None,
        points: vec![
            CurvePoint::new(0.0, 1.0),
            CurvePoint::new(1.0, 0.0),
        ],
        range: Range::new(0.0, 1.0),
    };
    let curve_b = CurveTexture {
        name: None,
        points: vec![
            CurvePoint::new(0.0, 0.0),
            CurvePoint::new(1.0, 1.0),
        ],
        range: Range::new(0.0, 1.0),
    };
    assert_ne!(curve_a.cache_key(), curve_b.cache_key());
}

// --- Gradient ---

#[test]
fn gradient_default() {
    let gradient = Gradient::default();
    assert_eq!(gradient.stops.len(), 2);
    assert_eq!(gradient.interpolation, GradientInterpolation::Linear);
    assert_eq!(gradient.stops[0].position, 0.0);
    assert_eq!(gradient.stops[1].position, 1.0);
}

#[test]
fn gradient_white() {
    let gradient = Gradient::white();
    assert_eq!(gradient.stops.len(), 2);
    assert_eq!(gradient.stops[0].color, [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(gradient.stops[1].color, [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn gradient_cache_key_differs() {
    let grad_a = Gradient {
        stops: vec![
            GradientStop { color: [1.0, 0.0, 0.0, 1.0], position: 0.0 },
            GradientStop { color: [0.0, 0.0, 1.0, 1.0], position: 1.0 },
        ],
        interpolation: GradientInterpolation::Linear,
    };
    let grad_b = Gradient {
        stops: vec![
            GradientStop { color: [0.0, 1.0, 0.0, 1.0], position: 0.0 },
            GradientStop { color: [1.0, 1.0, 0.0, 1.0], position: 1.0 },
        ],
        interpolation: GradientInterpolation::Linear,
    };
    assert_ne!(grad_a.cache_key(), grad_b.cache_key());
}

#[test]
fn gradient_interpolation_variants() {
    let linear = GradientInterpolation::Linear;
    let steps = GradientInterpolation::Steps;
    let smoothstep = GradientInterpolation::Smoothstep;

    assert_ne!(linear, steps);
    assert_ne!(linear, smoothstep);
    assert_ne!(steps, smoothstep);
    assert_eq!(GradientInterpolation::default(), GradientInterpolation::Linear);
}

// --- SolidOrGradientColor ---

#[test]
fn solid_color_default() {
    let color = SolidOrGradientColor::default();
    assert!(color.is_solid());
    assert!(!color.is_gradient());
    assert_eq!(color.as_solid_color(), Some([1.0, 1.0, 1.0, 1.0]));
}

#[test]
fn gradient_color() {
    let color = SolidOrGradientColor::Gradient {
        gradient: Gradient::default(),
    };
    assert!(!color.is_solid());
    assert!(color.is_gradient());
    assert_eq!(color.as_solid_color(), None);
}
