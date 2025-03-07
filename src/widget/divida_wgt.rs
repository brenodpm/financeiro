use crate::{
    componentes::{check_wgt::Check, input_wgt::Input},
    dto::{DadosDivida, Divida, ParcelaDivida},
    estilo::{alternate_colors, principal_comandos, principal_titulo, GERAL_BG, LISTA_BORDA_ESTILO, LISTA_SELECIONADO_ESTILO},
};
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::
        Stylize
    ,
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Paragraph, StatefulWidget,
        Widget,
    },
    DefaultTerminal,
};

#[derive(PartialEq)]
enum Status {
    AltNome,
    AltQuantidade,
    AltValor,
    AltInicio,
    AltPagos,
    AltCobrancaAuto,
    AltLista,
    Quitar,
    Sair(Option<Divida>),
}
pub struct EditarDivida {
    status: Status,

    divida: Divida,
    nome: Input,
    quant: Input,
    valor: Input,
    inicio: Input,
    pagos: Input,
    cobranca_auto: Check,
    aberto: Input,
    pago: Input,
    total: Input,
    quitar: Check,
    state: ListState,
}

impl Widget for &mut EditarDivida {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Edição de Dívidas", titulo, buf);
        principal_comandos(
            match self.status {
                Status::AltLista => vec![
                    "↓↑ (mover)",
                    "Enter (alterar pago)",
                    "ESC Sair",
                    "INSERT (salvar)",
                ],
                _ => vec!["Editar", "Tab (próximo)", "ESC Sair", "INSERT (salvar)"],
            },
            rodape,
            buf,
        );
        self.render(corpo, buf)
    }
}

impl EditarDivida {
    pub fn new() -> Self {
        Self {
            status: Status::AltNome,
            divida: Divida::default(),
            nome: Input::new_texto("Nome", String::new()),
            quant: Input::new_inteiro("Quant", 0),
            valor: Input::new_monetario("valor", 0.0),
            inicio: Input::new_data("Início", "00/00/00".to_string()),
            pagos: Input::new_inteiro("Pagos", 0),
            cobranca_auto: Check::new("Cobrança automática", false),
            aberto: Input::new_texto("Aberto", String::new()),
            pago: Input::new_texto("Pago", String::new()),
            total: Input::new_texto("Total", String::new()),
            quitar: Check::new("Quitar dívida", false),
            state: ListState::default(),
        }
    }

    pub fn from(divida: &Divida) -> Self {
        Self {
            status: Status::AltNome,
            divida: divida.clone(),
            nome: Input::new_texto("Nome", divida.nome.clone()),
            quant: Input::new_inteiro("Quant", divida.parcelas.quant()),
            valor: Input::new_monetario("valor", divida.parcelas.primeira().valor),
            inicio: Input::new_data(
                "Início",
                divida
                    .parcelas
                    .primeira()
                    .data_vencimento
                    .format("%d/%m/%y")
                    .to_string(),
            ),
            pagos: Input::new_inteiro("Pagos", divida.pagas().quant()),
            cobranca_auto: Check::new("Cobrança automática", divida.cobranca_automatica),
            aberto: Input::new_monetario("Aberto", divida.aberta().valor_total()),
            pago: Input::new_monetario("Pago", divida.pagas().valor_total()),
            total: Input::new_monetario("Total", divida.parcelas.valor_total()),
            quitar: Check::new("Quitar dívida", false),
            state: ListState::default(),
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Option<Divida>> {
        while !matches!(self.status, Status::Sair(_)) {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }

        if let Status::Sair(divida) = self.status {
            return Ok(divida);
        }
        Ok(None)
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if self.status == Status::AltLista {
            match key.code {
                KeyCode::Down => self.select_next(),
                KeyCode::Up => self.select_previous(),
                KeyCode::Enter | KeyCode::Right => self.mudar_pagamento(),
                KeyCode::Insert => self.salvar(),
                KeyCode::Esc => self.status = Status::Sair(None),
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Tab => self.proximo_input(),
                KeyCode::BackTab => self.anterior_input(),
                KeyCode::Esc => self.status = Status::Sair(None),
                KeyCode::Insert => self.salvar(),
                _ => self.alterar_input(key),
            }
        }
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
                    self.status = Status::AltNome;
                }
            }
            None => self.state.select_first(),
        }
    }

    fn alterar_input(&mut self, key: KeyEvent) {
        match self.status {
            Status::AltNome => self.nome.handle_key(key),
            Status::AltQuantidade => self.quant.handle_key(key),
            Status::AltValor => self.valor.handle_key(key),
            Status::AltInicio => self.inicio.handle_key(key),
            Status::AltPagos => self.pagos.handle_key(key),
            Status::AltCobrancaAuto => self.cobranca_auto.handle_key(key),
            Status::Quitar => self.quitar.handle_key(key),
            Status::AltLista => {}

            Status::Sair(_) => {}
        }
    }

    fn proximo_input(&mut self) {
        if self.divida.id.is_empty() {
            match self.status {
                Status::AltNome => self.status = Status::AltQuantidade,
                Status::AltQuantidade => self.status = Status::AltValor,
                Status::AltValor => self.status = Status::AltInicio,
                Status::AltInicio => self.status = Status::AltPagos,
                Status::AltPagos => self.status = Status::AltCobrancaAuto,
                Status::AltCobrancaAuto => self.status = Status::AltNome,

                Status::Sair(_) | Status::AltLista | Status::Quitar => {}
            }
        } else {
            match self.status {
                Status::AltNome => self.status = Status::AltCobrancaAuto,
                Status::AltCobrancaAuto => self.status = Status::Quitar,
                Status::Quitar => {
                    self.status = Status::AltLista;
                    self.state.select(Some(self.divida.pagas().len()));
                }
                Status::AltLista => self.status = Status::AltNome,

                _ => {}
            }
        }
    }

    fn anterior_input(&mut self) {
        if self.divida.id.is_empty() {
            match self.status {
                Status::AltNome => self.status = Status::AltCobrancaAuto,
                Status::AltQuantidade => self.status = Status::AltNome,
                Status::AltValor => self.status = Status::AltQuantidade,
                Status::AltInicio => self.status = Status::AltValor,
                Status::AltPagos => self.status = Status::AltInicio,
                Status::AltCobrancaAuto => self.status = Status::AltPagos,

                Status::Sair(_) | Status::AltLista | Status::Quitar => {}
            }
        } else {
            match self.status {
                Status::AltCobrancaAuto => self.status = Status::AltNome,
                Status::AltLista => self.status = Status::Quitar,
                Status::Quitar => self.status = Status::AltCobrancaAuto,
                Status::AltNome => {
                    self.status = Status::AltLista;
                    self.state.select(Some(self.divida.pagas().len()));
                }

                _ => {}
            }
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [titulo, parcelas, resumo, lista_parcelas] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Length(5),
            Constraint::Fill(1),
        ])
        .areas(area);

        self.nome
            .render(self.status == Status::AltNome, titulo, buf);
        self.render_parcelas(parcelas, buf);
        self.render_resumo(resumo, buf);
        self.render_lista_parcelas(lista_parcelas, buf);
    }

    fn render_parcelas(&mut self, area: Rect, buf: &mut Buffer) {
        let [area1, area2, area3] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .areas(area);

        Paragraph::new("\nPARCELAS")
            .bold()
            .left_aligned()
            .render(area1, buf);

        let [quant, valor, inicio] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(area2);

        self.quant
            .render(self.status == Status::AltQuantidade, quant, buf);
        self.valor
            .render(self.status == Status::AltValor, valor, buf);
        self.inicio
            .render(self.status == Status::AltInicio, inicio, buf);
        let [pagos, cobranca_auto, quitar] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(area3);

        self.pagos
            .render(self.status == Status::AltPagos, pagos, buf);

        self.cobranca_auto
            .render(self.status == Status::AltCobrancaAuto, cobranca_auto, buf);

        self.quitar
            .render(self.status == Status::Quitar, quitar, buf);
    }

    fn render_resumo(&mut self, area: Rect, buf: &mut Buffer) {
        let [area1, area2] =
            Layout::vertical([Constraint::Length(2), Constraint::Length(3)]).areas(area);

        Paragraph::new("\nRESUMO")
            .bold()
            .left_aligned()
            .render(area1, buf);

        let [aberto, pago, total] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(area2);

        self.aberto.render(false, aberto, buf);
        self.pago.render(false, pago, buf);
        self.total.render(false, total, buf);
    }

    fn render_lista_parcelas(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Parcelas").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(LISTA_BORDA_ESTILO)
            .bg(GERAL_BG);

        let items: Vec<ListItem> = self
            .divida
            .parcelas
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(LISTA_SELECIONADO_ESTILO)
            .highlight_symbol("▶")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    fn salvar(&mut self) {
        if self.divida.id.is_empty() {
            self.salvar_novo()
        } else {
            self.salvar_alteracao()
        }
    }

    fn salvar_alteracao(&mut self) {
        self.divida.nome = self.nome.to_string();
        self.divida.cobranca_automatica = self.cobranca_auto.get_checked();

        if self.quitar.get_checked() {
            for parcela in self.divida.parcelas.iter_mut() {
                parcela.pago = true;
            }
        }

        self.status = Status::Sair(Some(self.divida.clone()));
    }

    fn salvar_novo(&mut self) {
        match self.inicio.to_naivedate() {
            Ok(data) => {
                self.status = Status::Sair(Some(Divida::new(
                    self.nome.to_string(),
                    self.cobranca_auto.get_checked(),
                    self.quant.to_i32(),
                    self.valor.to_f64(),
                    data,
                    self.pagos.to_i32(),
                )));
            }
            Err(_) => {
                self.status = Status::AltInicio;
            }
        }
    }

    fn mudar_pagamento(&mut self) {
        if let Some(i) = self.state.selected() {
            self.divida.parcelas[i].pago = !self.divida.parcelas[i].pago;
        }

        self.aberto
            .set_monetario(self.divida.aberta().valor_total());
        self.pago.set_monetario(self.divida.pagas().valor_total());
        self.total.set_monetario(self.divida.parcelas.valor_total());
    }
}

impl From<&ParcelaDivida> for ListItem<'_> {
    fn from(parcela: &ParcelaDivida) -> Self {
        ListItem::new(format!(
            "{:02} - {} {}{}",
            parcela.num_parcela,
            parcela.data_vencimento.format("%d/%m/%Y"),
            parcela.valor,
            if parcela.pago { " (Pago)" } else { "" }
        ))
    }
}
