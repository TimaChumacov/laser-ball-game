use bevy::prelude::*;

pub const MENU_BG_COLOR: Color = Color::rgba(0.15, 0.15, 0.15, 0.2);
pub const BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub const MENU: Style = Style {
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(400.0), Val::Px(500.0)),
    ..Style::DEFAULT
};

pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Px(100.0), Val::Px(50.0)),
    ..Style::DEFAULT
};

pub const WRAPP_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
    gap: Size::new(Val::Px(18.0), Val::Px(18.0)),
    ..Style::DEFAULT
};