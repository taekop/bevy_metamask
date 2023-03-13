use bevy::prelude::Color;

// window
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const BACKGROUND_COLOR: Color = Color::WHITE;

// button
pub const BUTTON_WIDTH: f32 = 200.0;
pub const BUTTON_HEIGHT: f32 = 100.0;
pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.3, 0.9, 0.4);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.95, 0.3, 0.1);
pub const CLICKED_BUTTON_COLOR: Color = Color::rgb(0.7, 0.2, 0.1);
pub const BUTTON_FONT: &str = "fonts/FiraMono-Medium.ttf";
pub const BUTTON_FONT_SIZE: f32 = 20.0;
pub const BUTTON_TEXT_COLOR: Color = Color::BLACK;
