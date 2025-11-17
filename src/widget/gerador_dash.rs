use chrono::{Local, Months};
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
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
    calc::{self, calcular_gasto_por_conta_d30, calcular_resumo},
    dto::{
        Banco, Categoria, Configuracao, DadosDivida, DashDivida, DashGastoPorConta, DashResumo, Divida, DividaMes, Lancamento, OptionalLazy, OptionalLazyFn, ParcelaDivida, TipoFluxo
    },
    estilo::{
        GERAL_BG, GERAL_TEXT_FG, LISTA_BORDA_ESTILO, LISTA_SELECIONADO_ESTILO, alternate_colors, principal_comandos, principal_titulo
    },
    repository::atualizar_base,
    widget::alerta_wgt::Alerta,
};

enum Etapa {
    Iniciando,
    Base,
    Resumo,
    GastoPorConta,
    Dividas,
    Finalizado,
    Sair,
}

impl Etapa {
    fn to_string(&self) -> String {
        match self {
            Etapa::Iniciando => "Iniciando".to_string(),
            Etapa::Base => "Base dos gráficos".to_string(),
            Etapa::Resumo => "Resumo dos Gastos".to_string(),
            Etapa::GastoPorConta => "Gasto por conta".to_string(),
            Etapa::Dividas => "Dívidas".to_string(),
            Etapa::Finalizado => "Finalizado".to_string(),
            Etapa::Sair => "Sair".to_string(),
        }
    }
}

pub struct GeradorDash {
    items: Vec<Etapa>,
    state: ListState,
    sair: bool,

    config: Configuracao,
    lista_contas: Vec<String>,
    lista_dividas: Vec<ParcelaDivida>,
    lista_lancamentos: Vec<Lancamento>,
}

impl Widget for &mut GeradorDash {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Gerador de Dash", titulo, buf);
        principal_comandos(vec!["Aguarde..."], rodape, buf);
        self.render_list(corpo, buf);
    }
}

impl GeradorDash {
    pub fn new() -> Self {
        let mut lancamentos = Lancamento::lancamentos_listar();
        let categorias = Categoria::listar();

        lancamentos.iter_mut().for_each(|l| {
            if let OptionalLazy::Id(cat_id) = l.categoria.clone() {
                if let Some(cat) = categorias.iter().find(|c| c.id == cat_id) {
                    l.categoria = OptionalLazy::Some(cat.clone());
                }
            }
        });

        Self {
            items: vec![
                Etapa::Iniciando,
                Etapa::Base,
                Etapa::Resumo,
                Etapa::GastoPorConta,
                Etapa::Dividas,
                Etapa::Finalizado,
                Etapa::Sair,
            ],
            state: Default::default(),
            sair: false,

            config: Configuracao::buscar(),
            lista_contas: Banco::listar()
                .iter()
                .map(|b| b.contas.iter().map(|c| c.nome.clone()).collect())
                .collect(),
            lista_dividas: Vec::new(),
            lista_lancamentos: lancamentos,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.state.select_first();

        while !self.sair {
            self.executar_etapa(terminal);

            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
        }

        Ok(())
    }

    fn executar_etapa(&mut self, terminal: &mut DefaultTerminal) {
        if let Some(i) = self.state.selected() {
            match self.items[i] {
                Etapa::Iniciando => self.inicializar(),
                Etapa::Base => self.atualizar_base(),
                Etapa::Resumo => self.resumo_valores(),
                Etapa::Dividas => self.calcular_dividas(),
                Etapa::GastoPorConta => self.calcular_gasto_por_conta(),
                Etapa::Finalizado => {
                    let _ = Alerta::atencao(vec!["Dashboard concluído".to_string()]).run(terminal);
                }

                Etapa::Sair => self.sair = true,
            }
            self.state.select_next();
        }
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Gerar dash").centered())
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
                ListItem::new(Line::styled(value.to_string(), GERAL_TEXT_FG))
                    .bg(alternate_colors(i))
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

    /******************************************************************************************************************
     *                                         PROCESSAMENTOS
     ******************************************************************************************************************/

    fn inicializar(&mut self) {
        self.lista_dividas = Divida::listar()
            .iter()
            .flat_map(|d| d.parcelas.clone())
            .collect();
    }

    fn atualizar_base(&mut self) {
        atualizar_base();
    }

    fn resumo_valores(&mut self) {
        DashResumo::salvar(calcular_resumo(self.lista_lancamentos.clone()));
    }

    fn calcular_dividas(&mut self) {
        DashDivida::salvar(calc::calcular_dividas(
            self.lista_dividas.clone(),
            self.config.endividamento_max,
        ));
    }

    fn calcular_gasto_por_conta(&mut self) {
        DashGastoPorConta::salvar(calcular_gasto_por_conta_d30(self.lista_lancamentos.clone()));
    }
}
