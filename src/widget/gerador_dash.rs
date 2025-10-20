use std::env;

use chrono::{Local, Months};
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

use crate::{dto::{Configuracao, DadosDivida, Divida, DividaMes, ParcelaDivida}, estilo::{
    alternate_colors, principal_comandos, principal_titulo, GERAL_BG, GERAL_TEXT_FG,
    LISTA_BORDA_ESTILO, LISTA_SELECIONADO_ESTILO,
}, repository::atualizar_base, widget::alerta_wgt::Alerta};

enum Etapa {
    Iniciando,
    Base,
    Dividas,
    Finalizado,
    Sair,
}

pub struct GeradorDash {
    pub items: Vec<Etapa>,
    pub state: ListState,
    pub sair: bool,


    config: Configuracao,
    lista_dividas: Vec<ParcelaDivida>,
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
        Self {
            items: vec![
                Etapa::Iniciando,
                Etapa::Base,
                Etapa::Dividas,
                Etapa::Finalizado,
                Etapa::Sair,
            ],
            state: Default::default(),
            sair: false,

            config: Configuracao::buscar(),
            lista_dividas: Vec::new(),
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

    fn executar_etapa(&mut self,  terminal: &mut DefaultTerminal) {
        if let Some(i) = self.state.selected() {
            match self.items[i] {
                Etapa::Iniciando => {
                    self.inicializar();
                    self.state.select_next();
                }
                Etapa::Base => {
                    self.atualizar_base();
                    self.state.select_next();
                }
                Etapa::Dividas => {
                    self.calcular_dividas();
                    self.state.select_next();
                }
                Etapa::Finalizado => {
                    Alerta::atencao(vec!["Dashbard concluído".to_string()]).run(terminal);
                    self.state.select_next();
                }
                Etapa::Sair => {
                    self.sair = true;
                }
            }
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
                ListItem::new(Line::styled(
                    match value {
                        Etapa::Iniciando => "Iniciando",
                        Etapa::Base => "Base dos gráficos",
                        Etapa::Dividas => "Dívidas",
                        Etapa::Finalizado => "Finalizado",
                        Etapa::Sair => "Sair",
                    },
                    GERAL_TEXT_FG,
                ))
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

    fn calcular_dividas(&mut self) {
        let limite = self.config.endividamento_max;
        let mut meses: Vec<DividaMes> = Vec::new();

        let mut data = Local::now().date_naive();

        meses.push(DividaMes::new(
            self.lista_dividas.aberta().antes_de(data),
            0.0,
            "atrasado".to_string(),
        ));

        meses.push(DividaMes::new(
            self.lista_dividas
                .data_igual_ou_maior_que(data)
                .mes_e_ano(data),
            limite,
            "corrente".to_string(),
        ));

        for _ in 0..11 {
            data = data.checked_add_months(Months::new(1)).unwrap();
            meses.push(DividaMes::new(
                self.lista_dividas.mes_e_ano(data),
                limite,
                data.format("%m/%Y").to_string(),
            ));
        }

        DividaMes::salvar(meses);
    }
}
