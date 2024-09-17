use std::collections::HashMap;
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal,
};

use crate::{
    lancamento::Lancamento,
    regra::{Buscar, Regra},
};

impl Lancamento {
    pub fn categorizar(itens: Vec<Lancamento>) {
        println!("\n\nCategorizar");
        let mut pendente = Lancamento::nao_categorizados_listar();

        itens.into_iter().for_each(|novo| {
            if !pendente.iter().any(|a| a.id == novo.id) {
                pendente.push(novo);
            }
        });

        println!("Total a ser categorizados: {}", pendente.len());
        encontrar_categoria(pendente);
        let _ = categorizador_tui_start();
    }
}
fn encontrar_categoria(pendente: Vec<Lancamento>) {
    let regras = Regra::listar();
    let mut encontrados: Vec<Lancamento> = Vec::new();
    let mut nao_encontrado: Vec<Lancamento> = Vec::new();

    for mut item in pendente {
        match &regras.buscar(&item.descricao) {
            Some(c) => {
                item.categoria = Some(c.clone());
                encontrados.push(item);
            }
            None => {
                nao_encontrado.push(item);
            }
        }
    }

    Lancamento::nao_categorizados_salvar(&nao_encontrado);
}

/*********************************************************************************************************************
 *                                             TUI DE CATEGORIZAÇÃO
 *********************************************************************************************************************/


const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

fn categorizador_tui_start() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    should_exit: bool,
    todo_list: NovasRegrasList,
}

struct NovasRegrasList {
    items: Vec<NovaRegra>,
    state: ListState,
}

#[derive(Debug)]
struct NovaRegra {
    regex: String,
    lancamentos: Vec<Lancamento>,
    categoria: Option<String>,
    info: String,
}

impl Default for App {
    fn default() -> Self {
        let mut mapa: HashMap<String, Vec<Lancamento>> = HashMap::new();

        Lancamento::nao_categorizados_listar()
            .into_iter()
            .for_each(|l| {
                mapa.entry(l.descricao.clone())
                    .or_insert_with(Vec::new)
                    .push(l);
            });

        let mut itens: Vec<NovaRegra> = mapa
            .into_iter()
            .map(|(nome, notas)| NovaRegra::new(nome, notas))
            .collect();

        itens.sort_by(|a, b| b.lancamentos.len().cmp(&a.lancamentos.len()));

        Self {
            should_exit: false,
            todo_list: NovasRegrasList {
                items: itens,
                state: ListState::default(),
            },
        }
    }
}

impl NovaRegra {
    fn new(todo: String, lancamentos: Vec<Lancamento>) -> Self {
        Self {
            categoria: None,
            regex: todo.clone(),
            info: todo,
            lancamentos: lancamentos,
        }
    }
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.should_exit = true,
            KeyCode::Left => self.select_none(),
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Right | KeyCode::Enter => self.toggle_status(),
            _ => {}
        }
    }

    fn select_none(&mut self) {
        self.todo_list.state.select(None);
    }

    fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }
    fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }

    fn select_first(&mut self) {
        self.todo_list.state.select_first();
    }

    fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }

    /// Changes the status of the selected list item
    fn toggle_status(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items[i].categoria = match &self.todo_list.items[i].categoria {
                Some(_) => None,
                None => Some("A Categoria entra aqui".to_string()),
            }
        }
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

/// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Financeiro")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, → to change status, g/G to go top/bottom.")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Categorizar").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .todo_list
            .items
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
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.todo_list.state.selected() {
            let mut info: Vec<String> = Vec::new();
            info.push(self.todo_list.items[i].regex.clone());
            info.push("".to_string());

            let mut itens = self.todo_list.items[i]
                .lancamentos
                .clone()
                .into_iter()
                .map(|l| {
                    format!(
                        "Data: {}; Valor: RS {:0.02}",
                        l.data.format("%d/%m/%Y"),
                        l.valor
                    )
                })
                .collect::<Vec<String>>();

            info.append(&mut itens);
            info.join("\n").to_string()
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Lançamentos").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&NovaRegra> for ListItem<'_> {
    fn from(value: &NovaRegra) -> Self {
        let line = match &value.categoria {
            None => Line::styled(
                format!(" ☐ {:02} - {}", value.lancamentos.len(), value.regex),
                TEXT_FG_COLOR,
            ),
            Some(_) => Line::styled(
                format!(" ✓ {:02} - {}", value.lancamentos.len(), value.regex),
                COMPLETED_TEXT_FG_COLOR,
            ),
        };
        ListItem::new(line)
    }
}
