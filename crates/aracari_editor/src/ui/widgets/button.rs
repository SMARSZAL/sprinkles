use bevy::color::palettes::css::WHITE;
use bevy::color::palettes::tailwind;
use bevy::picking::hover::Hovered;
use bevy::prelude::*;

use crate::ui::tokens::{
    CORNER_RADIUS_LG, FONT_PATH, PRIMARY_COLOR, TEXT_BODY_COLOR, TEXT_DISPLAY_COLOR, TEXT_SIZE,
};

const ICON_MORE: &str = "icons/ri-more-fill.png";

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            setup_button_callbacks,
            handle_hover,
            handle_more_hover,
            handle_more_visibility,
            handle_button_click,
            handle_more_click,
            handle_button_right_click,
        ),
    );
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
            (Self::Active, false) => 0.1,
            (Self::Active, true) => 0.15,
            (Self::Default, false) => 0.5,
            (Self::Default | Self::Ghost, true) => 0.8,
            (Self::Primary, false) => 1.0,
            (Self::Primary, true) => 0.9,
        }
    }
    pub fn text_color(&self) -> Srgba {
        match self {
            Self::Default | Self::Ghost => TEXT_BODY_COLOR,
            Self::Primary => TEXT_DISPLAY_COLOR,
            Self::Active => PRIMARY_COLOR.lighter(0.05),
        }
    }
    pub fn border_color(&self) -> Srgba {
        match self {
            Self::Default | Self::Ghost => tailwind::ZINC_700,
            Self::Primary | Self::Active => PRIMARY_COLOR,
        }
    }
    pub fn border(&self) -> Val {
        match self {
            Self::Default | Self::Ghost => Val::Px(1.0),
            Self::Primary | Self::Active => Val::Px(0.0),
        }
    }
    pub fn border_opacity(&self, hovered: bool) -> f32 {
        match (self, hovered) {
            (Self::Ghost, false) => 0.0,
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

#[derive(Component)]
struct OnClickCallback(fn());

#[derive(Component)]
struct OnMoreCallback(fn());

#[derive(Component)]
struct ButtonMoreContainer;

#[derive(Component)]
struct ButtonMoreButton(Entity);

#[derive(Component)]
struct ButtonMoreIcon;

#[derive(Component)]
struct ButtonState {
    variant: ButtonVariant,
    on_click: Option<fn()>,
    on_more: Option<fn()>,
}

#[derive(Default)]
pub struct ButtonProps {
    pub content: String,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub align_left: bool,
    pub on_click: Option<fn()>,
    pub on_more: Option<fn()>,
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
    pub fn align_left(mut self) -> Self {
        self.align_left = true;
        self
    }
    pub fn on_click(mut self, callback: fn()) -> Self {
        self.on_click = Some(callback);
        self
    }
    pub fn on_more(mut self, callback: fn()) -> Self {
        self.on_more = Some(callback);
        self
    }
}

#[derive(Default)]
pub struct IconButtonProps {
    pub icon: String,
    pub color: Option<Srgba>,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub on_click: Option<fn()>,
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
    pub fn on_click(mut self, callback: fn()) -> Self {
        self.on_click = Some(callback);
        self
    }
}

fn button_base(variant: ButtonVariant, size: ButtonSize, align_left: bool) -> impl Bundle {
    (
        Button,
        EditorButton,
        variant,
        size,
        Hovered::default(),
        Node {
            width: if align_left {
                percent(100)
            } else {
                size.width()
            },
            height: size.height(),
            padding: UiRect::horizontal(size.padding()),
            border: UiRect::all(variant.border()),
            border_radius: BorderRadius::all(CORNER_RADIUS_LG),
            justify_content: if align_left {
                JustifyContent::Start
            } else {
                JustifyContent::Center
            },
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(
            variant
                .bg_color()
                .with_alpha(variant.bg_opacity(false))
                .into(),
        ),
        BorderColor::all(
            variant
                .border_color()
                .with_alpha(variant.border_opacity(false)),
        ),
    )
}

pub fn button(props: ButtonProps, asset_server: &AssetServer) -> impl Bundle {
    let ButtonProps {
        content,
        variant,
        size,
        align_left,
        on_click,
        on_more,
    } = props;
    let font: Handle<Font> = asset_server.load(FONT_PATH);

    (
        button_base(variant, size, align_left),
        ButtonState {
            variant,
            on_click,
            on_more,
        },
        children![
            (
                Text::new(content),
                TextFont {
                    font: font.into(),
                    font_size: TEXT_SIZE,
                    weight: FontWeight::MEDIUM,
                    ..default()
                },
                TextColor(variant.text_color().into()),
                Node {
                    flex_grow: if align_left { 1.0 } else { 0.0 },
                    ..default()
                },
            ),
            (
                ButtonMoreContainer,
                Node {
                    position_type: PositionType::Absolute,
                    right: px(0),
                    top: px(0),
                    width: px(28.0),
                    height: px(28.0),
                    ..default()
                },
            ),
        ],
    )
}

fn setup_button_callbacks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    new_buttons: Query<(Entity, &ButtonState, &Children), Added<EditorButton>>,
    containers: Query<Entity, With<ButtonMoreContainer>>,
) {
    for (button_entity, state, button_children) in &new_buttons {
        // attach on_click callback
        if let Some(callback) = state.on_click {
            commands
                .entity(button_entity)
                .insert(OnClickCallback(callback));
        }

        // attach on_more callback and spawn more button
        let Some(on_more) = state.on_more else {
            continue;
        };
        commands
            .entity(button_entity)
            .insert(OnMoreCallback(on_more));

        // find the more container (second child)
        let Some(&container_entity) = button_children.get(1) else {
            continue;
        };
        if containers.get(container_entity).is_err() {
            continue;
        }

        let initial_display = if state.variant == ButtonVariant::Ghost {
            Display::None
        } else {
            Display::Flex
        };

        let more_entity = commands
            .spawn(more_button(
                button_entity,
                state.variant,
                initial_display,
                &asset_server,
            ))
            .id();

        commands.entity(container_entity).add_child(more_entity);
    }
}

fn handle_hover(
    mut buttons: Query<
        (
            &ButtonVariant,
            &Hovered,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
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

fn handle_more_hover(
    mut more_buttons: Query<
        (&Hovered, &ButtonMoreButton, &mut BackgroundColor, &Children),
        Changed<Hovered>,
    >,
    parent_buttons: Query<&ButtonVariant>,
    mut icons: Query<&mut ImageNode, With<ButtonMoreIcon>>,
) {
    for (hovered, more_button, mut bg, children) in &mut more_buttons {
        let Ok(variant) = parent_buttons.get(more_button.0) else {
            continue;
        };

        let Some(&icon_entity) = children.first() else {
            continue;
        };
        let Ok(mut icon) = icons.get_mut(icon_entity) else {
            continue;
        };

        if hovered.get() {
            bg.0 = WHITE.with_alpha(0.01).into();
            icon.color = Color::Srgba(variant.text_color().lighter(0.2));
        } else {
            bg.0 = Color::NONE;
            icon.color = Color::Srgba(variant.text_color());
        }
    }
}

fn handle_more_visibility(
    buttons: Query<(&Hovered, &ButtonState, &Children), (Changed<Hovered>, With<EditorButton>)>,
    containers: Query<&Children, With<ButtonMoreContainer>>,
    mut more_buttons: Query<&mut Node, With<ButtonMoreButton>>,
) {
    for (hovered, state, button_children) in &buttons {
        if state.variant != ButtonVariant::Ghost || state.on_more.is_none() {
            continue;
        }

        let Some(&container_entity) = button_children.get(1) else {
            continue;
        };
        let Ok(container_children) = containers.get(container_entity) else {
            continue;
        };
        let Some(&more_entity) = container_children.first() else {
            continue;
        };
        let Ok(mut more_node) = more_buttons.get_mut(more_entity) else {
            continue;
        };

        more_node.display = if hovered.get() {
            Display::Flex
        } else {
            Display::None
        };
    }
}

fn handle_button_click(
    interactions: Query<
        (&Interaction, &OnClickCallback),
        (Changed<Interaction>, With<EditorButton>),
    >,
) {
    for (interaction, callback) in &interactions {
        if *interaction == Interaction::Pressed {
            (callback.0)();
        }
    }
}

fn handle_more_click(
    interactions: Query<(&Interaction, &ButtonMoreButton), Changed<Interaction>>,
    callbacks: Query<&OnMoreCallback>,
) {
    for (interaction, more_button) in &interactions {
        if *interaction == Interaction::Pressed {
            if let Ok(callback) = callbacks.get(more_button.0) {
                (callback.0)();
            }
        }
    }
}

fn handle_button_right_click(
    buttons: Query<(Entity, &Hovered), With<EditorButton>>,
    callbacks: Query<&OnMoreCallback>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if !mouse.just_pressed(MouseButton::Right) {
        return;
    }

    for (entity, hovered) in &buttons {
        if hovered.get() {
            if let Ok(callback) = callbacks.get(entity) {
                (callback.0)();
            }
        }
    }
}

fn more_button(
    parent_entity: Entity,
    variant: ButtonVariant,
    initial_display: Display,
    asset_server: &AssetServer,
) -> impl Bundle {
    (
        ButtonMoreButton(parent_entity),
        Button,
        Hovered::default(),
        Pickable {
            should_block_lower: true,
            is_hoverable: true,
        },
        Node {
            width: px(28.0),
            height: px(28.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(CORNER_RADIUS_LG),
            display: initial_display,
            ..default()
        },
        BackgroundColor(Color::NONE),
        children![(
            ButtonMoreIcon,
            ImageNode::new(asset_server.load(ICON_MORE))
                .with_color(Color::Srgba(variant.text_color())),
            Node {
                width: px(16.0),
                height: px(16.0),
                ..default()
            },
        )],
    )
}

pub fn icon_button(props: IconButtonProps, asset_server: &AssetServer) -> impl Bundle {
    let IconButtonProps {
        icon,
        color,
        variant,
        size,
        on_click,
    } = props;
    let icon_color = color.unwrap_or(variant.text_color());

    (
        button_base(variant, size, false),
        ButtonState {
            variant,
            on_click,
            on_more: None,
        },
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
