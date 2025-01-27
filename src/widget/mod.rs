mod categorizador_wgt;
mod check_wgt;
mod confirmar_categorizacao_wgt;
mod divida_wgt;
mod input_wgt;
mod lista_dividas_wgt;
mod menu_wgt;
mod selecionar_categoria_wgt;
mod gerador_dash;

pub use categorizador_wgt::Categorizador;
pub use lista_dividas_wgt::ListaDividas;
pub use menu_wgt::Menu;
pub use selecionar_categoria_wgt::SelecionarCategoria;
pub use gerador_dash::GeradorDash;

/******************************************************************************
 *                              ESTILOS
 *****************************************************************************/
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;

use ratatui::style::{
    palette::tailwind::{SLATE, WHITE},
    Color, Modifier, Style,
};
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
pub fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}
