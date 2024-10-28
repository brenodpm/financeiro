use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::{
            material::WHITE,
            tailwind::{BLUE, GREEN, SLATE},
        },
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal,
};

use crate::dto::Categoria;

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;

#[derive(PartialEq)]
enum Status {
    AltDesc,
    SelectCat,
    Sair,
}

pub struct SelecionarCategoria {
    pub descricao: String,
    pub selecionado: Option<Categoria>,

    texto_original: String,
    categorias: Vec<Categoria>,
    status: Status,
    character_index: usize,
    state: ListState,
}

impl SelecionarCategoria {
    pub fn new(descricao: String, categorias: Vec<Categoria>) -> Self {
        Self {
            texto_original: descricao.clone(),
            descricao,
            categorias,
            status: Status::AltDesc,
            character_index: 0,
            state: ListState::default(),
            selecionado: None,
        }
    }

    fn modificado(&self) -> bool {
        self.descricao != self.texto_original
    }

    fn sair(&self) -> bool {
        self.status == Status::Sair
    }

    fn byte_index(&self) -> usize {
        self.descricao
            .chars()
            .take(self.character_index)
            .map(|c| c.len_utf8())
            .sum()
    }

    fn para_esquerda(&mut self) {
        if self.character_index > 0 {
            self.character_index -= 1;
        }
    }

    fn para_direita(&mut self) {
        if self.character_index < self.descricao.chars().count() {
            self.character_index += 1;
        }
    }

    fn digitar(&mut self, letra: char) {
        let index = self.byte_index();
        self.descricao.insert(index, letra);
        self.para_direita();
    }

    fn apagar(&mut self) {
        if self.character_index > 0 {
            let index = self.byte_index();
            self.descricao.remove(index - 1);
            self.para_esquerda();
        }
    }

    fn deletar(&mut self) {
        if self.character_index < self.descricao.chars().count() {
            let index = self.byte_index();
            self.descricao.remove(index);
        }
    }

    fn inicio(&mut self) {
        self.character_index = 0;
    }

    fn fim(&mut self) {
        self.character_index = self.descricao.chars().count();
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        match self.state.selected() {
            Some(i) => {
                if i > 0 {
                    self.state.select_previous();
                } else {
                    self.alterar_regex();
                }
            }
            None => self.inicio(),
        }
    }

    fn selecionar_categoria(&mut self) {
        if let Some(i) = self.state.selected() {
            self.selecionado = Some(self.categorias[i].clone());
        }
        self.status = Status::Sair;
    }
    fn alterar_regex(&mut self) {
        self.status = Status::AltDesc;
        self.state.select(None);
    }
}

impl SelecionarCategoria {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<SelecionarCategoria> {
        while !self.sair() {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(self)
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match self.status {
            Status::AltDesc => self.handle_key_edit(key),
            Status::SelectCat => self.handle_key_cat(key),
            Status::Sair => {}
        }
    }

    fn handle_key_edit(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(to_insert) => self.digitar(to_insert),
            KeyCode::Left => self.para_esquerda(),
            KeyCode::Right => self.para_direita(),
            KeyCode::Backspace => self.apagar(),
            KeyCode::Delete => self.deletar(),
            KeyCode::Home => self.inicio(),
            KeyCode::End => self.fim(),
            KeyCode::Tab | KeyCode::Enter | KeyCode::Down => {
                self.descricao = self.descricao.trim().to_string();
                if self.descricao.len() < 3 {
                    self.descricao = self.texto_original.clone();
                } else {
                    self.state.select_first();
                    self.status = Status::SelectCat;
                }
            }
            KeyCode::Esc => {
                if self.modificado() {
                    self.descricao = self.texto_original.clone();
                } else {
                    self.status = Status::Sair
                }
            }
            _ => {}
        }
    }

    fn handle_key_cat(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Enter | KeyCode::Right => self.selecionar_categoria(),
            KeyCode::Esc | KeyCode::BackTab | KeyCode::Tab | KeyCode::Left => self.alterar_regex(),
            _ => {}
        }
    }
}

impl Widget for &mut SelecionarCategoria {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        self.render_titulo(titulo, buf);
        self.render_rodape(rodape, buf);

        let [regex, categorias] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(corpo);

        self.render_regex(regex, buf);
        self.render_categorias(categorias, buf);
    }
}

impl SelecionarCategoria {
    fn render_titulo(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Categorizar")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_rodape(&mut self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(match self.status {
            Status::AltDesc => "Esc (sair); Tab (Selecionar categoria); F5 (Restaurar)",
            Status::SelectCat => "Esc (sair); ↓↑ (mover); Enter (selecionar); Tab (editar regex) ",
            Status::Sair => "",
        })
        .centered()
        .render(area, buf);
    }

    fn render_regex(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Regex").centered())
            .borders(Borders::all())
            .border_set(symbols::border::PLAIN)
            .padding(Padding::horizontal(1))
            .style(match self.status {
                Status::AltDesc => SELECTED_STYLE,
                _ => Style::new(),
            });

        let mut text = self.descricao.clone();
        text.push_str(" ");
        let mut spans = Vec::new();

        let fg: Color = if self.modificado() {
            GREEN.c500
        } else {
            TEXT_FG_COLOR
        };

        if self.status == Status::AltDesc {
            let mut chars = text.chars();
            for (i, c) in chars.by_ref().enumerate() {
                if i == self.character_index {
                    spans.push(Span::styled(
                        c.to_string(),
                        Style::default()
                            .fg(fg)
                            .bg(WHITE)
                            .add_modifier(Modifier::BOLD),
                    ));
                } else {
                    spans.push(Span::styled(c.to_string(), Style::default().fg(fg)));
                }
            }
        } else {
            spans.push(Span::styled(text, Style::default().fg(fg)));
        }

        Paragraph::new(Text::from(Line::from(spans)))
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }

    fn render_categorias(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Categorias").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        let items: Vec<ListItem> = self
            .categorias
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("▶")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&Categoria> for ListItem<'_> {
    fn from(value: &Categoria) -> Self {
        let line = Line::styled(format!(" ☐ {}", value), TEXT_FG_COLOR);
        ListItem::new(line)
    }
}
