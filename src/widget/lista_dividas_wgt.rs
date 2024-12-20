use crate::dto::{DadosDivida, Divida};
use chrono::Utc;
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, RED, SLATE},
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

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub struct ListaDividas {
    sair: bool,
    state: ListState,
    dividas: Vec<Divida>,
}

impl Default for ListaDividas {
    fn default() -> Self {
        Self {
            sair: Default::default(),
            dividas: Divida::listar(),
            state: Default::default(),
        }
    }
}

impl ListaDividas {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.state.select_first();
        while !self.sair {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.sair = true,
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            //KeyCode::Right | KeyCode::Enter => self.selecionar(terminal),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }
}

impl Widget for &mut ListaDividas {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        ListaDividas::render_header(header_area, buf);
        ListaDividas::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

impl ListaDividas {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Financeiro")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ mover, → selecionar categoria, ESC sair")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Cadastro de dívidas").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .dividas
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
            let divida = self.dividas[i].clone();

            info.push(divida.nome.clone());
            info.push("".to_string());
            info.push(format!("--------------------------------------------------------------------"));
            info.push(format!("{:0>2} parcelas abertas       total de R$ {:0.02}", divida.aberta().quant(), divida.aberta().valor_total()));
            info.push(format!("{:0>2} parcelas fechadas      total de R$ {:0.02}", divida.pagas().quant(), divida.pagas().valor_total()));
            info.push(format!("{:0>2} parcelas total         total de R$ {:0.02}", divida.parcelas.quant(), divida.parcelas.valor_total()));
            info.push(format!("--------------------------------------------------------------------"));
            info.push("".to_string());
            info.push("".to_string());

            //atuais.sort_by(|a, b| b.regex.len().cmp(&a.regex.len()));
            let mut lanctos = divida.parcelas.clone();
            lanctos.sort_by(|a, b| a.data_vencimento.cmp(&b.data_vencimento));

            let mut itens = lanctos
                .clone()
                .into_iter()
                .map(|l| {
                    format!(
                        "{}ª parcela de R$ {:0.02} - {}{}",
                        l.num_parcela,
                        l.valor,
                        l.data_vencimento.format("%d/%m/%y"),
                        if l.pago { " ( PAGO )" } else { "" },
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
            .title(Line::raw("Parcelas").centered())
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

impl From<&Divida> for ListItem<'_> {
    fn from(divida: &Divida) -> Self {
        let now = Utc::now().naive_utc().date();
        let ult_parcela = divida.prox_parcela();
        let desc = if divida.parcelas.quant() == 1usize {
            divida.nome.clone()
        } else {
            format!(
                "{} ({} de {})",
                divida.nome,
                divida.pagas().quant(),
                divida.parcelas.quant()
            )
        };

        let line = Line::styled(
            format!(
                " {} {}    {:<50} R$ {:>8.2}",
                if ult_parcela.pago { "✓" } else { " " },
                ult_parcela.data_vencimento.format("%d/%m/%y"),
                desc,
                ult_parcela.valor,
            ),
            if ult_parcela.pago {
                COMPLETED_TEXT_FG_COLOR
            } else if ult_parcela.data_vencimento < now {
                RED.c500
            } else {
                TEXT_FG_COLOR
            },
        );
        ListItem::new(line)
    }
}
