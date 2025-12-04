use color_eyre::eyre::Result;
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
    dto::Meta,
    estilo::{
        alternate_colors, principal_comandos, principal_titulo, GERAL_BG, GERAL_TEXT_FG,
        LISTA_BORDA_ESTILO, LISTA_SELECIONADO_ESTILO,
    },
};

use super::meta_wgt::EditarMeta;

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
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Lista de Metas", titulo, buf);
        principal_comandos(
            vec![
                "↓↑ (mover)",
                "N (novo)",
                "ENTER (selecionar)",
                "ESC (sair)",
                "DEL (remover)",
            ],
            rodape,
            buf,
        );
        self.render_list(corpo, buf);
    }
}

impl ListaMeta {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.state.select_first();
        while !self.sair {
            if let Err(erro) = terminal.draw(|frame| frame.render_widget(&mut self, frame.area())) {
                log::error!("Erro ao desenhar tela SelecionarCategoria: {}", erro);
            }
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
        match EditarMeta::new().run(terminal) {
            Ok(ok) => match ok {
                Some(meta) => {
                    meta.salvar();
                    self.metas = Meta::listar();
                }
                None => {}
            },
            Err(erro) => log::error!("problemas ao editar nova meta: {}", erro),
        }
    }

    fn alterar_meta(&mut self, terminal: &mut DefaultTerminal) {
        if let Some(i) = self.state.selected() {
            let meta = self.metas[i].clone();
            match EditarMeta::set(meta).run(terminal) {
                Ok(ok) => match ok {
                    Some(meta) => {
                        meta.salvar();
                        self.metas = Meta::listar();
                    }
                    None => {}
                },
                Err(erro) => log::error!("problemas ao editar meta: {}", erro),
            }
        }
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Metas").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(LISTA_BORDA_ESTILO)
            .bg(GERAL_BG);

        let items: Vec<ListItem> = self
            .metas
            .iter()
            .enumerate()
            .map(|(i, todo_item)| ListItem::from(todo_item).bg(alternate_colors(i)))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(LISTA_SELECIONADO_ESTILO)
            .highlight_symbol("▶")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl From<&Meta> for ListItem<'_> {
    fn from(meta: &Meta) -> Self {
        let line = Line::styled(format!(" {}", meta.nome), GERAL_TEXT_FG);
        ListItem::new(line)
    }
}
