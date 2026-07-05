#[allow(dead_code)]
/// Visual identity: Tron meets Pink & Mystery.
/// Background: True Black (#000000)
/// Primary Glow: Hot Pink (#FF007F)
/// Accent: Deep Arcane Purple (#5D3FD3)
/// Gold: Mystic Gold (#D4AF37)
/// Text: Soft Lavender-White (#F3E5F5)
use ratatui::style::{Color, Modifier, Style};

pub const BG: Color = Color::Black;
pub const PINK: Color = Color::Rgb(0xFF, 0x00, 0x7F);
pub const PURPLE: Color = Color::Rgb(0x5D, 0x3F, 0xD3);
pub const GOLD: Color = Color::Rgb(0xD4, 0xAF, 0x37);
pub const LAVENDER: Color = Color::Rgb(0xF3, 0xE5, 0xF5);
pub const DIM_PINK: Color = Color::Rgb(0x80, 0x00, 0x40);
pub const DIM_PURPLE: Color = Color::Rgb(0x2E, 0x1F, 0x69);
pub const GREEN: Color = Color::Rgb(0x00, 0xFF, 0x7F);
pub const RED: Color = Color::Rgb(0xFF, 0x33, 0x33);
pub const YELLOW: Color = Color::Rgb(0xFF, 0xCC, 0x00);
pub const CYAN: Color = Color::Rgb(0x00, 0xFF, 0xCC);

pub fn pink_style() -> Style {
    Style::default().fg(PINK).bg(BG)
}

pub fn purple_style() -> Style {
    Style::default().fg(PURPLE).bg(BG)
}

pub fn gold_style() -> Style {
    Style::default().fg(GOLD).bg(BG)
}

pub fn lavender_style() -> Style {
    Style::default().fg(LAVENDER).bg(BG)
}

pub fn header_style() -> Style {
    Style::default()
        .fg(PINK)
        .bg(BG)
        .add_modifier(Modifier::BOLD)
}

pub fn border_style() -> Style {
    Style::default().fg(PINK).bg(BG)
}

pub fn dim_border_style() -> Style {
    Style::default().fg(DIM_PINK).bg(BG)
}

pub fn title_style() -> Style {
    Style::default()
        .fg(GOLD)
        .bg(BG)
        .add_modifier(Modifier::BOLD)
}

pub fn success_style() -> Style {
    Style::default().fg(GREEN).bg(BG)
}

pub fn error_style() -> Style {
    Style::default().fg(RED).bg(BG)
}

pub fn warn_style() -> Style {
    Style::default().fg(YELLOW).bg(BG)
}
