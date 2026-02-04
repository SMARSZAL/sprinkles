use aracari::prelude::*;
use bevy::prelude::*;

use crate::ui::widgets::inspector_field::InspectorFieldProps;
use crate::ui::widgets::variant_edit::{VariantDefinition, VariantEditProps, VariantField};
use crate::ui::widgets::vector_edit::VectorSuffixes;

use super::{InspectorItem, InspectorSection, inspector_section};

pub fn plugin(_app: &mut App) {}

pub fn emission_section(asset_server: &AssetServer) -> impl Bundle {
    inspector_section(
        InspectorSection::new(
            "Emission",
            vec![
                vec![InspectorFieldProps::new("amount").u32().into()],
                vec![InspectorItem::Variant {
                    path: "process.spawn.position.emission_shape".into(),
                    props: VariantEditProps::new("process.spawn.position.emission_shape")
                        .with_variants(emission_shape_variants()),
                }],
                vec![
                    InspectorFieldProps::new("process.spawn.position.emission_shape_offset")
                        .vec3(VectorSuffixes::XYZ)
                        .into(),
                ],
                vec![
                    InspectorFieldProps::new("process.spawn.position.emission_shape_scale")
                        .vec3(VectorSuffixes::XYZ)
                        .into(),
                ],
            ],
        ),
        asset_server,
    )
}

fn emission_shape_variants() -> Vec<VariantDefinition> {
    vec![
        VariantDefinition::new("Point")
            .with_icon("icons/blender_empty_axis.png")
            .with_default(EmissionShape::Point),
        VariantDefinition::new("Sphere")
            .with_icon("icons/blender_sphere.png")
            .with_default(EmissionShape::Sphere { radius: 1.0 })
            .with_rows(vec![vec![VariantField::f32("radius")]]),
        VariantDefinition::new("SphereSurface")
            .with_icon("icons/blender_mesh_uvsphere.png")
            .with_default(EmissionShape::SphereSurface { radius: 1.0 })
            .with_rows(vec![vec![VariantField::f32("radius")]]),
        VariantDefinition::new("Box")
            .with_icon("icons/blender_cube.png")
            .with_default(EmissionShape::Box { extents: Vec3::ONE })
            .with_rows(vec![vec![VariantField::vec3(
                "extents",
                VectorSuffixes::XYZ,
            )]]),
        VariantDefinition::new("Ring")
            .with_icon("icons/blender_mesh_torus.png")
            .with_default(EmissionShape::Ring {
                axis: Vec3::Y,
                height: 0.0,
                radius: 1.0,
                inner_radius: 0.0,
            })
            .with_rows(vec![
                vec![VariantField::vec3("axis", VectorSuffixes::XYZ)],
                vec![VariantField::f32("height")],
                vec![
                    VariantField::f32("radius"),
                    VariantField::f32("inner_radius"),
                ],
            ]),
    ]
}
