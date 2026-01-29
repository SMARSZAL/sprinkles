use bevy::color::palettes::tailwind;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;

use crate::ui::tokens::{
    CORNER_RADIUS_LG, FONT_PATH, PRIMARY_COLOR, TEXT_BODY_COLOR, TEXT_DISPLAY_COLOR, TEXT_SIZE,
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, handle_hover);
}

#[derive(Component)]
pub struct EditorButton;

#[derive(Component, Default, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Ghost,
    Active,
}

#[derive(Component, Default, Clone, Copy)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Icon,
}

impl ButtonVariant {
    pub fn bg_color(&self) -> Srgba {
        match self {
            Self::Default | Self::Ghost => tailwind::ZINC_700,
            Self::Primary | Self::Active => PRIMARY_COLOR,
        }
    }
    pub fn bg_opacity(&self, hovered: bool) -> f32 {
        match (self, hovered) {
            (Self::Ghost, false) => 0.0,
            (Self::Active, false) => 0.2,
            (Self::Default, false) => 0.5,
            (Self::Default | Self::Ghost, true) => 0.8,
            (Self::Active, true) => 0.3,
            (Self::Primary, _) => 1.0,
        }
    }
    pub fn text_color(&self) -> Srgba {
        match self {
            Self::Default | Self::Ghost => TEXT_BODY_COLOR,
            Self::Primary => TEXT_DISPLAY_COLOR,
            Self::Active => PRIMARY_COLOR,
        }
    }
    pub fn border_color(&self) -> Srgba {
        match self {
            Self::Default | Self::Ghost => tailwind::ZINC_700,
            Self::Primary => PRIMARY_COLOR,
            Self::Active => PRIMARY_COLOR,
        }
    }
    pub fn border(&self) -> Val {
        match self {
            Self::Default | Self::Ghost | Self::Active => Val::Px(1.0),
            Self::Primary => Val::Px(0.0),
        }
    }
    pub fn border_opacity(&self, hovered: bool) -> f32 {
        match (self, hovered) {
            (Self::Ghost, false) => 0.0,
            (Self::Active, _) => 0.4,
            _ => 1.0,
        }
    }
}

impl ButtonSize {
    fn width(&self) -> Val {
        match self {
            Self::Icon => Val::Px(28.0),
            _ => Val::Auto,
        }
    }
    fn height(&self) -> Val {
        Val::Px(28.0)
    }
    fn padding(&self) -> Val {
        match self {
            Self::Default => Val::Px(12.0),
            Self::Sm => Val::Px(6.0),
            Self::Icon => Val::Px(0.0),
        }
    }
    fn icon_size(&self) -> Val {
        Val::Px(16.0)
    }
}

#[derive(Default)]
pub struct ButtonProps {
    pub content: String,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

impl ButtonProps {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..default()
        }
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
}

#[derive(Default)]
pub struct IconButtonProps {
    pub icon: String,
    pub color: Option<Srgba>,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

impl IconButtonProps {
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            size: ButtonSize::Icon,
            ..default()
        }
    }
    pub fn color(mut self, color: Srgba) -> Self {
        self.color = Some(color);
        self
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
}

fn button_base(variant: ButtonVariant, size: ButtonSize) -> impl Bundle {
    (
        Button,
        EditorButton,
        variant,
        size,
        Hovered::default(),
        Node {
            width: size.width(),
            height: size.height(),
            padding: UiRect::horizontal(size.padding()),
            border: UiRect::all(variant.border()),
            border_radius: BorderRadius::all(CORNER_RADIUS_LG),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(
            variant
                .bg_color()
                .with_alpha(variant.bg_opacity(false))
                .into(),
        ),
        BorderColor::all(variant.border_color().with_alpha(variant.border_opacity(false))),
    )
}

pub fn button(props: ButtonProps, asset_server: &AssetServer) -> impl Bundle {
    let ButtonProps {
        content,
        variant,
        size,
    } = props;
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    (
        button_base(variant, size),
        children![(
            Text::new(content),
            TextFont {
                font: font.into(),
                font_size: TEXT_SIZE,
                weight: FontWeight::MEDIUM,
                ..default()
            },
            TextColor(variant.text_color().into()),
        )],
    )
}

pub fn icon_button(props: IconButtonProps, asset_server: &AssetServer) -> impl Bundle {
    let IconButtonProps {
        icon,
        color,
        variant,
        size,
    } = props;
    let icon_color = color.unwrap_or(variant.text_color());

    (
        button_base(variant, size),
        children![(
            ImageNode::new(asset_server.load(&icon)).with_color(Color::Srgba(icon_color)),
            Node {
                width: size.icon_size(),
                height: size.icon_size(),
                ..default()
            },
        )],
    )
}

fn handle_hover(
    mut buttons: Query<
        (&ButtonVariant, &Hovered, &mut BackgroundColor, &mut BorderColor),
        (Changed<Hovered>, With<EditorButton>),
    >,
) {
    for (variant, hovered, mut bg, mut border) in &mut buttons {
        let is_hovered = hovered.get();
        bg.0 = variant
            .bg_color()
            .with_alpha(variant.bg_opacity(is_hovered))
            .into();
        *border = BorderColor::all(
            variant
                .border_color()
                .with_alpha(variant.border_opacity(is_hovered)),
        );
    }
}

pub fn set_button_variant(
    variant: ButtonVariant,
    bg: &mut BackgroundColor,
    border: &mut BorderColor,
) {
    bg.0 = variant
        .bg_color()
        .with_alpha(variant.bg_opacity(false))
        .into();
    *border = BorderColor::all(
        variant
            .border_color()
            .with_alpha(variant.border_opacity(false)),
    );
}
