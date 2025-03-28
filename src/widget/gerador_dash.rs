use chrono::{Datelike, Local, Months, NaiveDate};
use color_eyre::eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
    DefaultTerminal,
};

use crate::{
    componentes::check_wgt::Check,
    dto::{DadosDivida, Divida, DividaMes, ParcelaDivida},
    estilo::{principal_comandos, principal_titulo},
    repository::atualizar_base,
};

#[derive(PartialEq)]
enum Etapa {
    Iniciando,
    Base,
    Dividas,
    Finalizado,
    Sair,
}

pub struct GeradorDash {
    etapa: Etapa,
    descricao: String,

    lista_dividas: Vec<ParcelaDivida>,

    base: Check,
    dividas: Check,
    fim: Check,
}

impl Widget for &mut GeradorDash {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Gerador de Dashboard", titulo, buf);
        principal_comandos(vec![self.descricao.as_str()], rodape, buf);

        self.render_list(corpo, buf);
    }
}

impl GeradorDash {
    pub fn new() -> Self {
        Self {
            etapa: Etapa::Iniciando,
            base: Check::new("Base dos gráficos", false),
            dividas: Check::new("Dividas", false),
            fim: Check::new("Encerrar", false),

            descricao: "Iniciando...".to_string(),
            lista_dividas: Vec::new(),
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.etapa != Etapa::Sair {
            if self.etapa == Etapa::Finalizado {
                if let Event::Key(key) = event::read()? {
                    self.handle_key(key);
                };
            }
            self.executar_etapa();

            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind == KeyEventKind::Press {
            self.etapa = Etapa::Sair
        }
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let [base, divida, fim] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .areas(area);

        self.base.render(self.etapa == Etapa::Base, base, buf);
        self.dividas
            .render(self.etapa == Etapa::Dividas, divida, buf);
        self.fim.render(self.etapa == Etapa::Finalizado, fim, buf);
    }

    fn executar_etapa(&mut self) {
        match self.etapa {
            Etapa::Iniciando => self.inicializar(),
            Etapa::Base => self.atualizar_base(),
            Etapa::Dividas => self.calcular_dividas(600f64),
            Etapa::Finalizado | Etapa::Sair => {}
        }
    }

    /******************************************************************************************************************
     *                                         PROCESSAMENTOS
     ******************************************************************************************************************/

    fn inicializar(&mut self) {
        self.lista_dividas = Divida::listar()
            .iter()
            .flat_map(|d| d.parcelas.clone())
            .collect();

        self.descricao = "Atualizando base dos gráficos".to_string();
        self.etapa = Etapa::Base;
    }

    fn atualizar_base(&mut self) {
        atualizar_base();
        self.base.set_checked(true);

        self.descricao = "Calculando dívidas".to_string();
        self.etapa = Etapa::Dividas;
    }

    fn calcular_dividas(&mut self, limite: f64) {
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

        self.descricao = "Pressione qualquer tecla para sair".to_string();
        self.etapa = Etapa::Finalizado;
    }
}
