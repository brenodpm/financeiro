use color_eyre::{owo_colors::OwoColorize, Result};
use ratatui::{
    buffer::Buffer,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        style,
    },
    layout::{Constraint, Layout, Rect},
    style::{
        palette::{
            material::WHITE,
            tailwind::{BLUE, GREEN, SLATE},
        },
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::{Line, Text},
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
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

#[derive(PartialEq)]
enum Status {
    AltDesc,
    SelectCat,
    Sair,
}

pub struct SelecionarCategoria {
    pub descricao: String,
    categorias: Vec<Categoria>,
    status: Status,
    character_index: usize,
    state: ListState,
    pub selecionado: Option<Categoria>,
}

impl SelecionarCategoria {
    pub fn new(descricao: String, categorias: Vec<Categoria>) -> Self {
        Self {
            descricao,
            categorias,
            status: Status::AltDesc,
            character_index: 0,
            state: ListState::default(),
            selecionado: None,
        }
    }

    fn sair(&self) -> bool {
        self.status == Status::Sair
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.descricao.chars().count())
    }

    fn para_esquerda(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn para_direita(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn digitar(&mut self, letra: char) {
        let index = self.byte_index();
        self.descricao.insert(index, letra);
        self.para_direita();
    }

    fn apagar(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.descricao.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.descricao.chars().skip(current_index);

            self.descricao = before_char_to_delete.chain(after_char_to_delete).collect();
            self.para_esquerda();
        }
    }

    fn deletar(&mut self) {
        if self.descricao.chars().count() > self.character_index {
            self.para_direita();
            self.apagar();
        }
    }

    fn byte_index(&self) -> usize {
        self.descricao
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.descricao.len())
    }

    fn inicio(&mut self) {
        self.character_index = self.clamp_cursor(0 as usize);
    }

    fn fim(&mut self) {
        self.character_index = self.clamp_cursor(self.descricao.chars().count());
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn selecionar_categoria(&mut self) {
        if let Some(i) = self.state.selected() {
            self.selecionado = Some(self.categorias[i].clone());
        }
        self.status = Status::Sair;
    }
}

impl SelecionarCategoria {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<SelecionarCategoria> {
        while !self.sair() {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            };
        }
        Ok(self)
    }

    fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
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
            KeyCode::Tab | KeyCode::Enter => {
                self.state.select_first();
                self.status = Status::SelectCat;
            }
            KeyCode::Esc => self.status = Status::Sair,
            _ => {}
        }
    }

    fn handle_key_cat(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Enter | KeyCode::Right => self.selecionar_categoria(),
            KeyCode::Esc | KeyCode::BackTab | KeyCode::Tab | KeyCode::Left => {
                self.status = Status::AltDesc;
                self.state.select(None);
            }
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
        let mut block = Block::new()
            .title(Line::raw("Regex").centered())
            .borders(Borders::all())
            .border_set(symbols::border::PLAIN)
            .padding(Padding::horizontal(1));

        if self.status == Status::AltDesc {
            block = block.style(SELECTED_STYLE);
        }

        let mut txt = self.descricao.clone();
        txt.push_str(" ");

        let i = self.character_index;
        let msg = vec![
            txt[..i].into(),
            txt.chars().collect::<Vec<char>>()[i]
                .to_string()
                .bg(WHITE)
                .fg(SLATE.c950),
            txt[i + 1..].into(),
        ];
        let text = Text::from(Line::from(msg));

        Paragraph::new(text)
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
            .highlight_symbol(">")
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
