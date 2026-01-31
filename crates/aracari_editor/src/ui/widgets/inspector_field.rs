use bevy::prelude::*;
use inflector::Inflector;

use super::checkbox::{CheckboxProps, checkbox};
use super::text_edit::{TextEditPrefix, TextEditProps, text_edit};
use crate::ui::components::inspector::binding::{Field, FieldKind};

const UPPERCASE_ACRONYMS: &[&str] = &["fps"];

fn path_to_label(path: &str) -> String {
    let field_name = path.split('.').last().unwrap_or(path);
    let sentence = field_name.to_sentence_case();

    sentence
        .split_whitespace()
        .map(|word| {
            let lower = word.to_lowercase();
            if UPPERCASE_ACRONYMS.contains(&lower.as_str()) {
                lower.to_uppercase()
            } else {
                word.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub struct InspectorFieldProps {
    path: String,
    kind: FieldKind,
    label: Option<String>,
    icon: Option<String>,
    suffix: Option<String>,
    placeholder: Option<String>,
    min: Option<f32>,
    max: Option<f32>,
}

impl InspectorFieldProps {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            kind: FieldKind::F32,
            label: None,
            icon: None,
            suffix: None,
            placeholder: None,
            min: None,
            max: None,
        }
    }

    pub fn percent(mut self) -> Self {
        self.kind = FieldKind::F32Percent;
        self
    }

    pub fn u32_or_empty(mut self) -> Self {
        self.kind = FieldKind::U32OrEmpty;
        self
    }

    pub fn optional_u32(mut self) -> Self {
        self.kind = FieldKind::OptionalU32;
        self
    }

    pub fn bool(mut self) -> Self {
        self.kind = FieldKind::Bool;
        self
    }

    pub fn with_icon(mut self, path: impl Into<String>) -> Self {
        self.icon = Some(path.into());
        self
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn with_min(mut self, min: f32) -> Self {
        self.min = Some(min);
        self
    }

    pub fn with_max(mut self, max: f32) -> Self {
        self.max = Some(max);
        self
    }

    fn inferred_label(&self) -> String {
        self.label
            .clone()
            .unwrap_or_else(|| path_to_label(&self.path))
    }

    fn inferred_suffix(&self) -> Option<&str> {
        if self.suffix.is_some() {
            return self.suffix.as_deref();
        }
        match self.kind {
            FieldKind::F32Percent => Some("%"),
            _ => None,
        }
    }

    fn inferred_min(&self) -> Option<f32> {
        if self.min.is_some() {
            return self.min;
        }
        match self.kind {
            FieldKind::F32Percent | FieldKind::U32OrEmpty | FieldKind::OptionalU32 => Some(0.0),
            _ => None,
        }
    }

    fn inferred_max(&self) -> Option<f32> {
        if self.max.is_some() {
            return self.max;
        }
        match self.kind {
            FieldKind::F32Percent => Some(100.0),
            _ => None,
        }
    }

    fn should_allow_empty(&self) -> bool {
        matches!(self.kind, FieldKind::U32OrEmpty | FieldKind::OptionalU32)
    }

    fn is_integer(&self) -> bool {
        matches!(
            self.kind,
            FieldKind::U32 | FieldKind::U32OrEmpty | FieldKind::OptionalU32
        )
    }
}

pub fn spawn_inspector_field(
    spawner: &mut ChildSpawnerCommands,
    props: InspectorFieldProps,
    asset_server: &AssetServer,
) {
    let field = Field::new(&props.path).with_kind(props.kind);
    let label = props.inferred_label();

    if props.kind == FieldKind::Bool {
        spawner.spawn((field, checkbox(CheckboxProps::new(label), asset_server)));
        return;
    }

    let mut text_props = TextEditProps::default().with_label(label);

    if props.is_integer() {
        text_props = text_props.numeric_i32();
    } else {
        text_props = text_props.numeric_f32();
    }

    if let Some(suffix) = props.inferred_suffix() {
        text_props = text_props.with_suffix(suffix);
    }

    if let Some(ref placeholder) = props.placeholder {
        text_props = text_props.with_placeholder(placeholder);
    }

    if let Some(ref icon) = props.icon {
        text_props = text_props.with_prefix(TextEditPrefix::Icon { path: icon.clone() });
    }

    if let Some(min) = props.inferred_min() {
        text_props = text_props.with_min(min as f64);
    }

    if let Some(max) = props.inferred_max() {
        text_props = text_props.with_max(max as f64);
    }

    if props.should_allow_empty() {
        text_props = text_props.allow_empty();
    }

    spawner.spawn((field, text_edit(text_props)));
}

pub fn fields_row() -> impl Bundle {
    Node {
        width: Val::Percent(100.0),
        column_gap: Val::Px(12.0),
        ..default()
    }
}
