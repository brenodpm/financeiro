use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget,
    },
    DefaultTerminal,
};

use crate::{
    app::Etapa,
    estilo::{
        alternate_colors, principal_comandos, principal_titulo, GERAL_BG, GERAL_TEXT_FG,
        LISTA_BORDA_ESTILO, LISTA_SELECIONADO_ESTILO,
    },
};

#[derive(Clone)]
pub struct Menu {
    pub items: Vec<(String, Etapa)>,
    pub etapa: Option<Etapa>,
    pub state: ListState,
}

impl Menu {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Etapa> {
        self.state.select_first();

        while self.etapa.is_none() {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }

        Ok(self.etapa.unwrap())
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.etapa = Some(Etapa::Sair),
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Right | KeyCode::Enter => self.acessar(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn acessar(&mut self) {
        if let Some(i) = self.state.selected() {
            self.etapa = Some(self.items[i].1.clone());
        }
    }
}

impl Widget for &mut Menu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo(header_area, buf);
        principal_comandos(
            vec!["↓↑ mover", "→ selecionar", "ESC Sair"],
            footer_area,
            buf,
        );
        self.render_list(main_area, buf);
    }
}

impl Menu {
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Menu").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(LISTA_BORDA_ESTILO)
            .bg(GERAL_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, value)| {
                ListItem::new(Line::styled(value.0.clone(), GERAL_TEXT_FG)).bg(alternate_colors(i))
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(LISTA_SELECIONADO_ESTILO)
            .highlight_symbol("▶ ")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
