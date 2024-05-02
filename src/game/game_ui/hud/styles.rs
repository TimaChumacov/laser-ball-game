use bevy::prelude::*;

pub const HUD_BLOCK_COLOR: Color = Color::rgba(0.15, 0.15, 0.15, 0.2);

pub const HUD_BLOCK_STYLE: Style = Style {
    flex_direction: FlexDirection::Row,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(300.0), Val::Px(40.0)),
    ..Style::DEFAULT
};

pub const HUD_BLOCK_HALVES_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
    ..Style::DEFAULT
};

pub const HUD_IMAGE_STYLE: Style = Style {
    margin: UiRect::all(Val::Px(20.0)),
    size: Size::new(Val::Px(40.0), Val::Px(40.0)),
    ..Style::DEFAULT
};

pub const WRAPP_STYLE: Style = Style {
    position_type: PositionType::Absolute,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    gap: Size::new(Val::Px(18.0), Val::Px(18.0)),
    ..Style::DEFAULT
};