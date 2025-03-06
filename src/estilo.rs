use ratatui::style::{
    palette::tailwind::{SLATE, WHITE},
    Color, Modifier, Style,
};

const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;



pub fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}
pub fn estilo_input() -> Style {
    Style::new()
}
pub fn estilo_input_foco() -> Style {
    Style::default()
        .fg(WHITE)
        .bg(SLATE.c900)
        .add_modifier(Modifier::BOLD)
}
pub fn fg_color() -> Color {
    SLATE.c200
}
