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
use std::collections::HashMap;

use crate::dto::{Categoria, FluxoRegra, Lancamento, NovaRegra, Regra, TipoFluxo};

use super::SelecionarCategoria;

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub struct Categorizador {
    pub should_exit: bool,
    pub items: Vec<NovaRegra>,
    pub receitas: Vec<Categoria>,
    pub despesas: Vec<Categoria>,
    pub state: ListState,
}

impl Default for Categorizador {
    fn default() -> Self {
        let mut receitas: Vec<Categoria> = Vec::new();
        let mut despesas: Vec<Categoria> = Vec::new();

        Categoria::listar()
            .into_iter()
            .for_each(|c| match c.tipo.clone() {
                TipoFluxo::Receita(_) => receitas.push(c),
                TipoFluxo::Retorno => receitas.push(c),

                TipoFluxo::Despesa(_) => despesas.push(c),
                TipoFluxo::Investimento => despesas.push(c),

                TipoFluxo::Transferencias => {
                    despesas.push(c.clone());
                    receitas.push(c);
                }

                TipoFluxo::Vazio => {}
            });

        Self {
            should_exit: false,
            items: buscar_itens(),
            state: ListState::default(),
            receitas: receitas,
            despesas: despesas,
        }
    }
}

fn buscar_itens() -> Vec<NovaRegra> {
    let mut entradas: HashMap<String, Vec<Lancamento>> = HashMap::new();
    let mut saidas: HashMap<String, Vec<Lancamento>> = HashMap::new();

    Lancamento::nao_categorizados_listar()
        .into_iter()
        .for_each(|l| {
            if l.valor > 0.0 {
                entradas
                    .entry(l.descricao.clone())
                    .or_insert_with(Vec::new)
                    .push(l);
            } else {
                saidas
                    .entry(l.descricao.clone())
                    .or_insert_with(Vec::new)
                    .push(l);
            }
        });

    let mut itens: Vec<NovaRegra> = entradas
        .into_iter()
        .map(|(nome, notas)| NovaRegra::new(nome, FluxoRegra::Entrada, notas))
        .collect();

    itens.append(
        &mut saidas
            .into_iter()
            .map(|(nome, notas)| NovaRegra::new(nome, FluxoRegra::Saida, notas))
            .collect(),
    );

    itens.sort_by(|a, b| b.lancamentos.len().cmp(&a.lancamentos.len()));

    itens
}
impl Categorizador {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.state.select_first();
        while !self.should_exit && self.items.len() > 0 {
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
            KeyCode::Esc => self.should_exit = true,
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Right | KeyCode::Enter => self.categorizar(terminal),
            KeyCode::Insert => self.atualizar(),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }
    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    /// Changes the status of the selected list item
    fn categorizar(&mut self, terminal: &mut DefaultTerminal) {
        if let Some(i) = self.state.selected() {
            let item = self.items[i].clone();
            let select = SelecionarCategoria::new(
                item.texto.clone(),
                if item.lancamentos[0].valor > 0.0 {
                    self.receitas.clone()
                } else {
                    self.despesas.clone()
                },
            );
            let (regex, selecionado) = select.run(terminal).unwrap();

            self.items[i].regex = regex;
            self.items[i].categoria = selecionado;
        }
    }

    fn atualizar(&mut self) {
        let mut regras: Vec<Regra> = Vec::new();

        self.items
            .clone()
            .into_iter()
            .for_each(|nr| match nr.categoria {
                Some(cat) => regras.push(Regra {
                    regex: nr.regex,
                    fluxo: nr.fluxo,
                    categoria: cat.id,
                }),
                None => {}
            });

        Regra::adicionar(&mut regras);
        Lancamento::recategorizar();
        self.items = buscar_itens();
        self.state.select_first();
    }
}

impl Widget for &mut Categorizador {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        Categorizador::render_header(header_area, buf);
        Categorizador::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

impl Categorizador {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Financeiro")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ mover, → selecionar categoria, INSERT categorizar, ESC sair")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw(format!("Categorizar {} itens", self.items.len())).centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
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
            .highlight_symbol("▶")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.state.selected() {
            let mut info: Vec<String> = Vec::new();

            info.push(self.items[i].texto.clone());
            info.push("".to_string());

            //atuais.sort_by(|a, b| b.regex.len().cmp(&a.regex.len()));
            let mut lanctos = self.items[i].lancamentos.clone();
            lanctos.sort_by(|a, b| b.data.cmp(&a.data));

            let mut itens = lanctos
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
        let fluxo = match value.fluxo {
            FluxoRegra::Entrada => '▲',
            FluxoRegra::Saida => '▼',
            FluxoRegra::None => '_',
        };

        let line = match &value.categoria {
            None => Line::styled(
                format!(
                    " ☐ {:02} {} - {}",
                    value.lancamentos.len(),
                    fluxo,
                    value.regex
                ),
                TEXT_FG_COLOR,
            ),
            Some(_) => Line::styled(
                format!(
                    " ✓ {:02} {} - {} ({})",
                    value.lancamentos.len(),
                    fluxo,
                    value.regex,
                    value.categoria.clone().unwrap()
                ),
                COMPLETED_TEXT_FG_COLOR,
            ),
        };
        ListItem::new(line)
    }
}
