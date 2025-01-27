use std::fmt::{self, Formatter};

use chrono::{NaiveDate, ParseResult};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{
        palette::tailwind::WHITE, Modifier, Style, Stylize,
    },
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Padding, Paragraph,
        Widget, Wrap,
    },
};

use super::{estilo_input, estilo_input_foco, fg_color};

#[derive(PartialEq)]
pub enum TipoValor {
    Texto,
    Data,
    Inteiro,
    Monetario,
}

pub struct Input {
    nome: String,
    valor: String,

    tipo: TipoValor,
    cursor: usize,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.nome, self.valor))
    }
}

impl Input {
    pub fn new_texto(nome: &str, valor: String) -> Self {
        Input {
            nome: nome.to_string(),
            valor: valor,

            tipo: TipoValor::Texto,
            cursor: 0usize,
        }
    }

    pub fn new_data(nome: &str, valor: String) -> Self {
        let mut resp = Input {
            nome: nome.to_string(),
            valor: valor,

            tipo: TipoValor::Data,
            cursor: 0usize,
        };
        resp.formatar();
        resp
    }

    pub fn new_inteiro(nome: &str, valor: i32) -> Self {
        let mut resp = Input {
            nome: nome.to_string(),
            valor: valor.to_string(),

            tipo: TipoValor::Inteiro,
            cursor: 0usize,
        };
        resp.formatar();
        resp
    }

    pub fn new_monetario(nome: &str, valor: f64) -> Self {
        let mut resp = Input {
            nome: nome.to_string(),
            valor: valor.to_string(),

            tipo: TipoValor::Monetario,
            cursor: 0usize,
        };
        resp.formatar();
        resp
    }

    pub fn _set_texto(&mut self, valor: String) {
        self.valor = valor;
        self.cursor = 0usize;
    }

    pub fn _set_data(&mut self, valor: String) {
        self.valor = valor;
        self.cursor = 0usize;
        self.formatar();
    }

    pub fn _set_inteiro(&mut self, valor: i32) {
        self.valor = valor.to_string();
        self.cursor = 0usize;
        self.formatar();
    }

    pub fn set_monetario(&mut self, valor: f64) {
        self.valor = valor.to_string();
        self.cursor = 0usize;
        self.formatar();
    }

    pub fn to_string(&self) -> String {
        self.valor.clone()
    }

    pub fn to_naivedate(&self) -> ParseResult<NaiveDate> {
        NaiveDate::parse_from_str(&self.valor, "%d/%m/%y")
    }

    pub fn to_i32(&self) -> i32 {
        self.valor.parse().unwrap_or(0)
    }

    pub fn to_f64(&self) -> f64 {
        self.valor
            .replace(".", "")
            .replace(",", ".")
            .parse()
            .unwrap_or(0.0)
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

        let mut texto = self.valor.clone();
        texto.push_str(" ");
        let mut spans = Vec::new();

        if foco {
            let mut chars = texto.chars();
            for (i, c) in chars.by_ref().enumerate() {
                if i == self.cursor {
                    spans.push(Span::styled(
                        c.to_string(),
                        Style::default()
                            .fg(fg_color())
                            .bg(WHITE)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::styled(c.to_string(), Style::default().fg(fg_color())));
                }
            }
        } else {
            spans.push(Span::styled(texto.clone(), Style::default().fg(fg_color())));
        }

        Paragraph::new(Text::from(Line::from(spans)))
            .block(block)
            .fg(fg_color())
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(to_insert) => self.digitar(to_insert),
            KeyCode::Left => self.para_esquerda(),
            KeyCode::Right => self.para_direita(),
            KeyCode::Backspace => self.apagar(),
            KeyCode::Delete => self.deletar(),
            KeyCode::Home => self.inicio(),
            KeyCode::End => self.fim(),
            _ => {}
        }
    }

    fn digitar(&mut self, letra: char) {
        let index = self.byte_index();

        if self.tipo == TipoValor::Data && self.cursor < self.valor.chars().count() {
            self.valor.remove(index);
        }

        if (self.tipo == TipoValor::Monetario || self.tipo == TipoValor::Inteiro)
            && self.valor.len() > 11
        {
            return;
        }

        self.valor.insert(index, letra);

        self.formatar();
        self.para_direita();
    }

    fn byte_index(&self) -> usize {
        self.valor
            .chars()
            .take(self.cursor)
            .map(|c| c.len_utf8())
            .sum()
    }

    fn apagar(&mut self) {
        if self.tipo == TipoValor::Data {
            if self.cursor == 3 || self.cursor == 6 {
                self.cursor -= 1;
            }
        }

        if self.cursor > 0 {
            let index = self.byte_index();
            self.valor.remove(index - 1);
            self.para_esquerda();
            if self.tipo == TipoValor::Data {
                self.valor.insert(index - 1, '0');
            }
        }
        self.formatar();
    }

    fn deletar(&mut self) {
        if self.cursor < self.valor.chars().count() {
            let index = self.byte_index();
            self.valor.remove(index);
            if self.tipo == TipoValor::Data {
                self.valor.insert(index, '0');
                self.para_direita();
            }
        }
        self.formatar();
    }

    fn para_direita(&mut self) {
        if self.cursor < self.valor.chars().count() {
            self.cursor += 1;

            if self.tipo == TipoValor::Data {
                if self.cursor == 2 || self.cursor == 5 {
                    self.para_direita();
                }
            }
        }
    }

    fn para_esquerda(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;

            if self.tipo == TipoValor::Data {
                if self.cursor == 2 || self.cursor == 5 {
                    self.para_esquerda();
                }
            }
        }
    }

    fn inicio(&mut self) {
        self.cursor = 0;
    }

    fn fim(&mut self) {
        self.cursor = self.valor.chars().count();
    }

    fn formatar(&mut self) {
        match self.tipo {
            TipoValor::Data => {
                self.formatar_data();
            }
            TipoValor::Inteiro => {
                self.formatar_numerico();
            }
            TipoValor::Monetario => {
                self.formatar_monetario();
            }
            _ => {}
        }
    }

    fn formatar_numerico(&mut self) {
        let pontos_antes = self.valor[0..self.byte_index()]
            .chars()
            .filter(|c| *c == '.')
            .count();
        self.valor.retain(|c| c.is_digit(10));

        while self.valor.len() > 1 && self.valor.chars().nth(0usize).unwrap() == '0' {
            self.valor.remove(0usize);
            self.para_esquerda();
        }
        if self.valor.len() == 0 {
            self.valor.push('0');
        }

        let mut idx = self.valor.len();
        while idx > 3 {
            idx -= 3;
            self.valor.insert(idx, '.');
        }

        let pontos_depois = self.valor[0..self.byte_index()]
            .chars()
            .filter(|c| *c == '.')
            .count();

        if pontos_antes < pontos_depois {
            self.cursor += 1;
        } else if pontos_antes > pontos_depois {
            self.cursor -= 1;
        }
        if self.valor.chars().nth(0usize).unwrap() == '0' && self.cursor == 0 {
            self.cursor += 1;
        }
    }

    fn formatar_monetario(&mut self) {
        let pontos_antes = self.valor[0..self.byte_index()]
            .chars()
            .filter(|c| *c == '.')
            .count();
        self.valor.retain(|c| c.is_digit(10) || c == ',');

        while self.valor.len() > 0 && self.valor.chars().nth(0usize).unwrap() == '0' {
            self.valor.remove(0usize);
            self.para_esquerda();
        }

        if self.valor.len() > 0 && self.valor.chars().nth(0usize).unwrap() == ',' {
            self.valor.insert(0, '0');
        }

        if self.valor.len() == 0 {
            self.valor.push_str("0,00");
        }

        if !self.valor.contains(',') {
            self.valor.push_str(",00");
        }

        let mut idx = self.valor.chars().position(|c| c == ',').unwrap_or(0);
        while self.valor.len() < idx + 3 {
            self.valor.push('0');
        }
        self.valor.truncate(idx + 3);

        while idx > 3 {
            idx -= 3;
            self.valor.insert(idx, '.');
        }

        let pontos_depois = self.valor[0..self.byte_index()]
            .chars()
            .filter(|c| *c == '.')
            .count();

        if pontos_antes < pontos_depois {
            self.cursor += 1;
        } else if pontos_antes > pontos_depois {
            self.cursor -= 1;
        }

        if self.valor.chars().nth(0usize).unwrap() == '0' && self.cursor == 0 {
            self.cursor += 1;
        }
    }

    fn formatar_data(&mut self) {
        self.valor.retain(|c| c.is_digit(10));
        if self.valor.len() > 2 {
            self.valor.insert(2, '/');
        }
        if self.valor.len() > 5 {
            self.valor.insert(5, '/');
        }
        if self.valor.len() > 8 {
            self.valor.truncate(8);
        }
    }
}
