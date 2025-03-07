use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE, WHITE},
        Color, Modifier, Style, Stylize,
    },
    widgets::{Paragraph, Widget},
};

/********************************************************************
 *                               GERAL
 ********************************************************************/
pub const GERAL_BG: Color = SLATE.c950;
pub const GERAL_TEXT_FG: Color = SLATE.c200;

/********************************************************************
 *                             PRINCIPAL
 ********************************************************************/
pub fn principal_titulo(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Financeiro")
        .bold()
        .centered()
        .fg(SLATE.c100)
        .bg(BLUE.c800)
        .render(area, buf);
}

pub fn principal_comandos(comandos: Vec<&str>, area: Rect, buf: &mut Buffer) {
    let constraints: Vec<_> = comandos.iter().map(|_| Constraint::Fill(1)).collect();
    let layout = Layout::horizontal(constraints);
    let binding = layout.split(area);
    let areas: Vec<&Rect> = binding.into_iter().collect();

    for (i, &comando) in comandos.iter().enumerate() {
        Paragraph::new(comando)
            .centered()
            .fg(SLATE.c100)
            .bg(BLUE.c800)
            .render(*areas[i], buf);
    }
}

/********************************************************************
 *                         lista
 ********************************************************************/
pub const LISTA_BORDA_ESTILO: Style = Style::new().fg(SLATE.c100).bg(BLUE.c600);
pub const LISTA_SELECIONADO_ESTILO: Style =
    Style::new().bg(SLATE.c600).add_modifier(Modifier::BOLD);

const ALT_ROW_BG_COLOR: Color = SLATE.c800;
pub fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        GERAL_BG
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
