use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    style::{
        palette::{
            material::BLUE,
            tailwind::SLATE,
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

use crate::estilo::{alternate_colors, estilo_input, estilo_input_foco, fg_color};

#[derive(Clone)]
pub struct ItemListaSuspensa {
    pub id: String,
    pub texto: String,
}

#[derive(Clone)]
pub struct ListaSuspensa {
    pub nome: String,
    lista: Vec<ItemListaSuspensa>,

    selecionado: usize,
    modo_lista: bool,
    modo_lista_state: ListState,
}

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

impl Widget for &mut ListaSuspensa {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(self.nome.clone()).centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .lista
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::new(format!(" {}", todo_item.texto.clone())).bg(color)
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
        StatefulWidget::render(list, area, buf, &mut self.modo_lista_state);
    }
}

impl ListaSuspensa {
    pub fn new(nome: &str, itens: Vec<ItemListaSuspensa>) -> Self {
        ListaSuspensa {
            nome: nome.to_string(),
            selecionado: 0usize,
            lista: itens,
            modo_lista: false,
            modo_lista_state: Default::default(),
        }
    }
    pub fn new_string(nome: &str, itens: Vec<&str>) -> Self {
        ListaSuspensa {
            nome: nome.to_string(),
            selecionado: 0usize,
            lista: itens
                .iter()
                .map(|s| ItemListaSuspensa {
                    id: s.to_string(),
                    texto: s.to_string(),
                })
                .collect(),
            modo_lista: false,
            modo_lista_state: Default::default(),
        }
    }

    pub fn set_lista(&mut self, itens: Vec<ItemListaSuspensa>) {
        self.lista = itens;
        self.selecionado = 0usize;
    }

    pub fn get_id_selecionado(&mut self) -> String {
        if self.lista.len() > 0 {
            self.lista[self.selecionado].id.clone()
        } else {
            String::new()
        }
    }

    pub fn set_id_selecionado(&mut self, id:String) {
        if let Some(pos) = self.lista.iter().position(|item| item.id == id) {
            self.selecionado = pos;
        }else{
            self.selecionado = 0usize;
        }
    }

    pub fn get_texto_selecionado(&mut self) -> String {
        if self.lista.len() > 0 {
            self.lista[self.selecionado].texto.clone()
        } else {
            String::new()
        }
    }

    pub fn id_selecionado_eh(&self, id: String) -> bool {
        if self.lista.len() > 0 {
            self.lista[self.selecionado].id == id
        } else {
            id.is_empty()
        }
    }

    pub fn texto_selecionado_eh(&self, nome: &str) -> bool {
        if self.lista.len() > 0 {
            self.lista[self.selecionado].texto == nome
        } else {
            nome.is_empty()
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
            self.get_texto_selecionado(),
            Style::default().fg(fg_color()),
        ));

        Paragraph::new(Text::from(Line::from(spans)))
            .block(block)
            .fg(fg_color())
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }

    pub fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        match key.code {
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Enter => self.selecionado = self.clone().mostrar_lista(terminal),

            _ => {}
        }
    }

    fn select_next(&mut self) {
        if !self.lista.is_empty() && self.selecionado < (self.lista.len() - 1) {
            self.selecionado += 1;
        }
    }

    fn select_previous(&mut self) {
        if !self.lista.is_empty() && self.selecionado > 0 {
            self.selecionado -= 1;
        }
    }

    fn mostrar_lista(mut self, terminal: &mut DefaultTerminal) -> usize {
        self.modo_lista = true;
        self.modo_lista_state.select_first();
        while self.modo_lista {
            terminal
                .draw(|frame| frame.render_widget(&mut self, frame.area()))
                .unwrap();
            if let Event::Key(key) = event::read().unwrap() {
                self.modo_lista_handle_key(key);
            };
        }
        self.selecionado
    }

    pub fn modo_lista_handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.modo_lista = false,
            KeyCode::Down => self.modo_lista_select_next(),
            KeyCode::Up => self.modo_lista_select_previous(),
            KeyCode::Right | KeyCode::Enter => self.modo_lista_selecionar(),
            _ => {}
        }
    }
    fn modo_lista_select_next(&mut self) {
        self.modo_lista_state.select_next();
    }

    fn modo_lista_select_previous(&mut self) {
        self.modo_lista_state.select_previous();
    }

    fn modo_lista_selecionar(&mut self) {
        if let Some(i) = self.modo_lista_state.selected() {
            self.selecionado = i;
        } else {
            self.selecionado = 0usize;
        }
        self.modo_lista = false;
    }
}
