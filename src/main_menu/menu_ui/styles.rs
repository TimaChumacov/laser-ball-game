use bevy::prelude::*;

pub const MENU_BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const BLOCK_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(200.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const IMAGE_STYLE: Style = Style {
    margin: UiRect::all(Val::Px(20.0)),
    size: Size::new(Val::Px(80.0), Val::Px(80.0)),
    ..Style::DEFAULT
};

pub const TITLE_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(400.0), Val::Px(160.0)),
    ..Style::DEFAULT
};

pub const WRAPP_STYLE: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    gap: Size::new(Val::Px(18.0), Val::Px(18.0)),
    ..Style::DEFAULT
};