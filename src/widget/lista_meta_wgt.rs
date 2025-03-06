use color_eyre::eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal,
};

use crate::{dto::Meta, estilo::alternate_colors};

use super::meta_wgt::EditarMeta;

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const TEXT_FG_COLOR: Color = SLATE.c200;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct ListaMeta {
    sair: bool,
    metas: Vec<Meta>,
    state: ListState,
}

impl Default for ListaMeta {
    fn default() -> Self {
        Self {
            sair: false,
            metas: Meta::listar(),
            state: Default::default(),
        }
    }
}

impl Widget for &mut ListaMeta {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        ListaMeta::render_header(header_area, buf);
        ListaMeta::render_footer(footer_area, buf);
        self.render_list(main_area, buf);
    }
}

impl ListaMeta {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.state.select_first();
        while !self.sair {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            };
        }
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.sair = true,
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Char('n') | KeyCode::Char('N') => self.nova_meta(terminal),
            KeyCode::Right | KeyCode::Enter => self.alterar_meta(terminal),
            KeyCode::Delete => self.deletar(),
            _ => {}
        }
    }

    fn deletar(&mut self) {
        if let Some(i) = self.state.selected() {
            let meta = self.metas[i].clone();

            meta.deletar();
            self.metas = Meta::listar();
        }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn nova_meta(&mut self, terminal: &mut DefaultTerminal) {
        match EditarMeta::new().run(terminal).unwrap() {
            Some(meta) => {
                meta.salvar();
                self.metas = Meta::listar();
            }
            None => {}
        }
    }

    fn alterar_meta(&mut self, terminal: &mut DefaultTerminal) {
        if let Some(i) = self.state.selected() {
            let meta = self.metas[i].clone();
            match EditarMeta::set(meta).run(terminal).unwrap() {
                Some(meta) => {
                    meta.salvar();
                    self.metas = Meta::listar();
                }
                None => {}
            }
        }
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Financeiro")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ mover, N novo, ENTER selecionar, ESC sair, DEL remover")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Cadastro de metas").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .metas
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("▶")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl From<&Meta> for ListItem<'_> {
    fn from(meta: &Meta) -> Self {
        let line = Line::styled(format!(" {}", meta.nome), TEXT_FG_COLOR);
        ListItem::new(line)
    }
}
