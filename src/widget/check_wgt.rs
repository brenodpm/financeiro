use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{
        Style, Stylize,
    },
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Padding, Paragraph,
        Widget, Wrap,
    },
};

use super::{estilo_input, estilo_input_foco, fg_color};

pub struct Check {
    nome: String,
    valor: bool,
}

impl Check {
    pub fn get_checked(&self) -> bool {
        self.valor
    }
    pub fn set_checked(&mut self, valor: bool) {
        self.valor = valor;
    }

    pub fn new(nome: &str, valor: bool) -> Self {
        Check {
            nome: nome.to_string(),
            valor: valor,
        }
    }

    pub fn render(&mut self, foco: bool, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(self.nome.clone()).left_aligned())
            .borders(Borders::all())
            .border_set(symbols::border::PLAIN)
            .padding(Padding::horizontal(1))
            .style(if foco {
                estilo_input_foco()
            } else {
                estilo_input()
            });

        let mut spans = Vec::new();
        spans.push(Span::styled(
            format!("({}) {}", if self.valor { 'X' } else { ' ' }, self.nome),
            Style::default().fg(fg_color()),
        ));

        Paragraph::new(Text::from(Line::from(spans)))
            .block(block)
            .fg(fg_color())
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => self.valor = !self.valor,
            KeyCode::Char('S')
            | KeyCode::Char('s')
            | KeyCode::Char('T')
            | KeyCode::Char('t')
            | KeyCode::Char('Y')
            | KeyCode::Char('y')
            | KeyCode::Char('V')
            | KeyCode::Char('v') => self.valor = true,
            KeyCode::Char('N') | KeyCode::Char('n') | KeyCode::Char('F') | KeyCode::Char('f') => {
                self.valor = false
            }

            _ => {}
        }
    }
}
