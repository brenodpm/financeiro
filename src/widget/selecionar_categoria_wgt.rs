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
    componentes::input_wgt::Input,
    dto::{Categoria, TipoFluxo},
    estilo::{
        alternate_colors, principal_comandos, principal_titulo, GERAL_BG, GERAL_TEXT_FG,
        LISTA_BORDA_ESTILO, LISTA_SELECIONADO_ESTILO,
    },
};

#[derive(PartialEq)]
enum Status {
    AltDesc,
    SelectCat,
    Sair,
}

pub struct SelecionarCategoria {
    regex: Input,
    selecionado: Option<Categoria>,

    texto_original: String,
    categorias: Vec<Categoria>,
    status: Status,
    state: ListState,
}

impl Widget for &mut SelecionarCategoria {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Selecionar Categoria", titulo, buf);
        principal_comandos(
            match self.status {
                Status::AltDesc => vec![
                    "Editar",
                    "Tab (Selecionar categoria)",
                    "F5 (Restaurar)",
                    "ESC Sair",
                ],
                Status::SelectCat => vec![
                    "↓↑ (mover)",
                    "Enter (selecionar)",
                    "Tab (editar regex)",
                    "ESC Sair",
                ],
                Status::Sair => Vec::new(),
            },
            rodape,
            buf,
        );

        let [regex, categorias] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(corpo);

        self.regex
            .render(self.status == Status::AltDesc, regex, buf);
        self.render_categorias(categorias, buf);
    }
}

impl SelecionarCategoria {
    pub fn new(texto: String, categorias: Vec<Categoria>) -> Self {
        Self {
            texto_original: texto.clone(),
            regex: Input::new_texto("Regex", texto),
            categorias,
            status: Status::AltDesc,
            state: ListState::default(),
            selecionado: None,
        }
    }

    fn sair(&self) -> bool {
        self.status == Status::Sair
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
            None => self.alterar_regex(),
        }
    }

    fn selecionar_categoria(&mut self) {
        if let Some(i) = self.state.selected() {
            self.selecionado = Some(self.categorias[i].clone());
            if self.categorias[i].tipo == TipoFluxo::SemCategoria {
                self.regex.set_texto(self.texto_original.clone());
            }
        }

        self.status = Status::Sair;
    }
    fn alterar_regex(&mut self) {
        self.status = Status::AltDesc;
        self.state.select(None);
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<(String, Option<Categoria>)> {
        while !self.sair() {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok((self.regex.to_string(), self.selecionado))
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match self.status {
            Status::AltDesc => match key.code {
                KeyCode::Esc => {
                    if self.regex.to_string() != self.texto_original {
                        self.regex.set_texto(self.texto_original.clone());
                    } else {
                        self.status = Status::Sair
                    }
                }
                KeyCode::Down | KeyCode::Up | KeyCode::BackTab | KeyCode::Tab => {
                    if self.regex.to_string().len() < 3 {
                        self.regex.set_texto(self.texto_original.clone());
                    } else {
                        self.state.select_first();
                        self.status = Status::SelectCat;
                    }
                }
                _ => self.regex.handle_key(key),
            },
            Status::SelectCat => self.handle_key_cat(key),
            Status::Sair => {}
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

    fn render_categorias(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Categorias").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(LISTA_BORDA_ESTILO)
            .bg(GERAL_BG);

        let items: Vec<ListItem> = self
            .categorias
            .iter()
            .enumerate()
            .map(|(i, todo_item)| ListItem::from(todo_item).bg(alternate_colors(i)))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(LISTA_SELECIONADO_ESTILO)
            .highlight_symbol("▶ ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}

impl From<&Categoria> for ListItem<'_> {
    fn from(value: &Categoria) -> Self {
        let line = Line::styled(format!(" ☐ {}", value), GERAL_TEXT_FG);
        ListItem::new(line)
    }
}
