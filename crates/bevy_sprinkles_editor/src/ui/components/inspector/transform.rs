use bevy::prelude::*;

use crate::ui::widgets::inspector_field::InspectorFieldProps;
use crate::ui::widgets::vector_edit::VectorSuffixes;

use super::{InspectorSection, inspector_section};

pub fn transform_section(asset_server: &AssetServer) -> impl Bundle {
    inspector_section(
        InspectorSection::new(
            "Transform",
            vec![vec![
                InspectorFieldProps::new("position")
                    .vector(VectorSuffixes::XYZ)
                    .into(),
            ]],
        ),
        asset_server,
    )
}
