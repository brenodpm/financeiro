use crate::{
    dto::{
        Categoria, FluxoRegra, Lancamento, Lazy, LazyFn, OptionalLazy, OptionalLazyFn, Regra,
        TipoFluxo, Unico,
    },
    estilo::alternate_colors,
    repository::Buscar,
};
use color_eyre::Result;
use itertools::Itertools;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{GREEN, RED, SLATE, WHITE},
        Color, Stylize,
    },
    text::Line,
    widgets::{Paragraph, Row, StatefulWidget, Table, TableState, Widget},
    DefaultTerminal,
};

use super::SelecionarCategoria;

pub struct ConfirmarCategorias {
    pub should_exit: bool,
    itens: Vec<Lancamento>,
    nao_encontrados: Vec<Lancamento>,
    state: TableState,
    receitas: Vec<Categoria>,
    despesas: Vec<Categoria>,
}

impl Default for ConfirmarCategorias {
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

                TipoFluxo::SemCategoria => {
                    despesas.push(c.clone());
                    receitas.push(c);
                }
            });
        let (enc, nenc) = encontrar_categoria();
        Self {
            should_exit: false,
            itens: enc,
            nao_encontrados: nenc,
            state: TableState::default(),
            receitas,
            despesas,
        }
    }
}

fn encontrar_categoria() -> (Vec<Lancamento>, Vec<Lancamento>) {
    let regras = Regra::listar();
    let mut encontrados: Vec<Lancamento> = Vec::new();
    let mut nao_encontrado: Vec<Lancamento> = Vec::new();

    let pendente: Vec<Lancamento> = Lancamento::nao_categorizados_listar();
    for mut item in pendente {
        match &regras.buscar(
            &item.descricao.to_lowercase(),
            if item.valor > 0.0 {
                FluxoRegra::Entrada
            } else {
                FluxoRegra::Saida
            },
        ) {
            Some(regra) => {
                item.categoria = OptionalLazy::Some(regra.categoria.some());
                item.regra = OptionalLazy::Some(regra.clone());
                encontrados.push(item);
            }
            None => {
                nao_encontrado.push(item);
            }
        }
    }

    log::info!(
        "{} lançamento(s) categorizado(s), restando {}",
        encontrados.len(),
        nao_encontrado.len()
    );

    //Lancamento::nao_categorizados_salvar(&nao_encontrado);
    (
        encontrados
            .into_iter()
            .sorted_by(|a, b| a.data.cmp(&b.data))
            .collect(),
        nao_encontrado,
    )
}

impl ConfirmarCategorias {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.state.select_first();
        while !self.should_exit && self.itens.len() > 0 {
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
            KeyCode::F(5) => self.atualizar(),
            _ => {}
        }
    }

    fn atualizar(&mut self) {
        Lancamento::lancamentos_adicionar(&self.itens);
        Lancamento::nao_categorizados_salvar(&self.nao_encontrados);

        self.should_exit = true;
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
            let item = self.itens[i].clone();
            let select = SelecionarCategoria::new(
                item.descricao.clone(),
                if item.valor > 0.0 {
                    self.receitas.clone()
                } else {
                    self.despesas.clone()
                },
            );
            let (regex, selecionado) = select.run(terminal).unwrap();
            match selecionado {
                Some(cat) => {
                    if regex != item.descricao {
                        let mut regra = Regra {
                            id: String::new(),
                            categoria: Lazy::Some(cat.clone()),
                            fluxo: if item.valor > 0.0 {
                                FluxoRegra::Entrada
                            } else {
                                FluxoRegra::Saida
                            },
                            regex: regex,
                        };

                        regra.gerar_id();

                        Regra::nova(regra.clone());
                        self.itens[i].regra = OptionalLazy::Some(regra);
                    } else {
                        self.itens[i].regra = OptionalLazy::None;
                    }
                    self.itens[i].categoria = OptionalLazy::Some(cat);
                }
                None => {}
            }
        }
    }
}

impl Widget for &mut ConfirmarCategorias {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        ConfirmarCategorias::render_header(header_area, buf);
        ConfirmarCategorias::render_footer(footer_area, buf);
        self.render_list(main_area, buf);
    }
}

impl ConfirmarCategorias {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Financeiro")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ mover, → selecionar categoria, F5 categorizar, ESC sair")
            .centered()
            .render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let selected = match self.state.selected() {
            Some(index) => index,
            None => 0usize,
        };

        let rows: Vec<Row> = self
            .itens
            .iter()
            .enumerate()
            .map(|(i, lanct)| {
                let (bg, fg, fgrs) = cores(i, selected, lanct.valor);

                Row::new(vec![
                    Line::from(lanct.data.format("%d/%m/%Y").to_string())
                        .bg(bg)
                        .fg(fg),
                    Line::from(match lanct.conta.clone() {
                        Some(ct) => ct,
                        None => "Não identificada".to_string(),
                    })
                    .bg(bg)
                    .fg(fg),
                    Line::from(lanct.descricao.clone()).bg(bg).fg(fg),
                    Line::from(format!("{:.2}", lanct.valor))
                        .bg(bg)
                        .fg(fgrs)
                        .right_aligned(),
                    Line::from(lanct.categoria.some().unwrap().to_string())
                        .bg(bg)
                        .fg(fg),
                ])
            })
            .collect();

        let widths = [
            Constraint::Length(10),
            Constraint::Length(20),
            Constraint::Fill(3),
            Constraint::Length(10),
            Constraint::Fill(2),
        ];

        let table = Table::new(rows, widths).highlight_symbol("▶ ");

        StatefulWidget::render(table, area, buf, &mut self.state);
    }
}

fn cores(i: usize, selected: usize, valor: f64) -> (Color, Color, Color) {
    if i == selected {
        (
            SLATE.c600,
            WHITE,
            if valor > 0f64 { GREEN.c500 } else { RED.c500 },
        )
    } else {
        (
            alternate_colors(i),
            SLATE.c300,
            if valor > 0f64 { GREEN.c200 } else { RED.c200 },
        )
    }
}
