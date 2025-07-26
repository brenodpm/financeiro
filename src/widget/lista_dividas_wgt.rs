use crate::{
    dto::{DadosDivida, Divida},
    estilo::{
        alternate_colors, principal_comandos, principal_titulo, GERAL_BG, GERAL_TEXT_FG,
        LISTA_BORDA_ESTILO, LISTA_SELECIONADO_ESTILO,
    },
};
use chrono::{Datelike, Utc};
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{GREEN, RED},
        Color, Stylize,
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal,
};

use super::divida_wgt::EditarDivida;

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
            KeyCode::Char('n') | KeyCode::Char('N') => self.nova_divida(terminal),
            KeyCode::Right | KeyCode::Enter => self.alterar_divida(terminal),
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn nova_divida(&mut self, terminal: &mut DefaultTerminal) {
        match EditarDivida::new().run(terminal).unwrap() {
            Some(divida) => {
                divida.salvar();
                self.dividas = Divida::listar();

                if let Some(i) = self.dividas.iter().position(|a| a.id == divida.id) {
                    self.state.select(Some(i));
                    self.alterar_divida(terminal);
                }
            }
            None => {}
        }
    }

    fn alterar_divida(&mut self, terminal: &mut DefaultTerminal) {
        if let Some(i) = self.state.selected() {
            let divida = self.dividas[i].clone();
            match EditarDivida::from(&divida).run(terminal).unwrap() {
                Some(divida) => {
                    divida.salvar();
                    self.dividas = Divida::listar();
                }
                None => {}
            }
        }
    }
}

impl Widget for &mut ListaDividas {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Controle de dívidas", titulo, buf);
        principal_comandos(
            vec!["↓↑ (mover)", "ENTER (selecionar)", "N (nova)", "ESC (sair)"],
            rodape,
            buf,
        );

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(corpo);

        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

impl ListaDividas {
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Dividas ativas").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(LISTA_BORDA_ESTILO)
            .bg(GERAL_BG);

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
            .highlight_style(LISTA_SELECIONADO_ESTILO)
            .highlight_symbol("▶")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        let hoje = Utc::now().naive_utc().date();
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.state.selected() {
            let mut info: Vec<String> = Vec::new();
            let divida = self.dividas[i].clone();

            info.push(format!(
                "{}{}",
                divida.nome.clone(),
                if divida.cobranca_automatica {
                    " (Cobrança automática)"
                } else {
                    ""
                }
            ));

            if divida.parcelas.aberta().len() > 0 {
                let meses_faltando =
                    (divida.parcelas.aberta().ultima().data_vencimento.year() - hoje.year()) * 12
                        + divida.parcelas.aberta().ultima().data_vencimento.month() as i32
                        - hoje.month() as i32;
                if meses_faltando > 0 {
                    if meses_faltando < 12 {
                        info.push(format!("Faltam {} meses para acabar", meses_faltando));
                    } else if meses_faltando < 24 {
                        info.push(format!(
                            "Falta 1 ano e {} meses para acabar",
                            meses_faltando - 12
                        ));
                    } else {
                        info.push(format!("Faltam {} anos para acabar", meses_faltando / 12));
                    }
                } else {
                    info.push("Só faltam os atrasados".to_string());
                }
            }

            info.push("".to_string());
            info.push(format!(
                "--------------------------------------------------------------------"
            ));
            info.push(format!(
                "{:0>2} parcelas abertas       total de R$ {:0.02}",
                divida.parcelas.aberta().quant(),
                divida.parcelas.aberta().valor_total()
            ));
            info.push(format!(
                "{:0>2} parcelas fechadas      total de R$ {:0.02}",
                divida.parcelas.pagas().quant(),
                divida.parcelas.pagas().valor_total()
            ));
            info.push(format!(
                "{:0>2} parcelas total         total de R$ {:0.02}",
                divida.parcelas.quant(),
                divida.parcelas.valor_total()
            ));
            info.push(format!(
                "--------------------------------------------------------------------"
            ));
            info.push("".to_string());
            info.push("".to_string());

            let mut lanctos = divida.parcelas.aberta().clone();
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
                        if l.data_vencimento < hoje {
                            " [ ATRASADA ]"
                        } else {
                            ""
                        },
                    )
                })
                .collect::<Vec<String>>();

            info.append(&mut itens);
            info.join("\n").to_string()
        } else {
            "Selecione uma dívida".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Parcelas").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(LISTA_BORDA_ESTILO)
            .bg(GERAL_BG)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .fg(GERAL_TEXT_FG)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

impl From<&Divida> for ListItem<'_> {
    fn from(divida: &Divida) -> Self {
        let now = Utc::now().naive_utc().date();
        let ult_parcela = divida.prox_parcela();
        let desc = if divida.parcelas.quant() == 1 {
            divida.nome.clone()
        } else {
            format!(
                "{} ({} de {})",
                divida.nome,
                divida.parcelas.pagas().quant(),
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
                GERAL_TEXT_FG
            },
        );
        ListItem::new(line)
    }
}
