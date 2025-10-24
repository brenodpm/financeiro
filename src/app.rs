use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

use crate::widget::{Categorizador, ContraCheque, EditarConfiguracoes, GeradorDash, ListaCategoria, ListaDividas, ListaMeta, Menu};

#[derive(Clone)]
pub enum Etapa {
    Categorizar,
    Dividas,
    ContraCheque,
    Metas,
    Menu,
    Dash,
    Configuracoes,
    Categorias,
    Sair,
}

pub struct App {
    etapa: Etapa,
}

impl Default for App {
    fn default() -> Self {
        Self {
            etapa: Etapa::Categorizar,
        }
    }
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let menu = Menu {
            items: vec![
                ("Categorizar".to_string(), Etapa::Categorizar),
                ("Dívidas".to_string(), Etapa::Dividas),
                ("Contra-cheques".to_string(), Etapa::ContraCheque),
                ("Metas".to_string(), Etapa::Metas),
                ("Gerar Gráfico".to_string(), Etapa::Dash),
                ("Configurações".to_string(), Etapa::Configuracoes),
                ("Categorias".to_string(), Etapa::Categorias),
                ("Sair".to_string(), Etapa::Sair),
            ],
            etapa: Option::None,
            state: Default::default(),
        };

        loop {
            match self.etapa {
                Etapa::Menu => self.menu(&mut terminal, menu.clone()),

                Etapa::Categorizar => self.categorizar(&mut terminal),
                Etapa::Metas => self.metas(&mut terminal),
                Etapa::Dividas => self.dividas(&mut terminal),
                Etapa::Dash => self.dash(&mut terminal),
                Etapa::Configuracoes => self.configuracoes(&mut terminal),
                Etapa::Categorias => self.categorias(&mut terminal),
                Etapa::ContraCheque => self.contracheque(&mut terminal),
                
                Etapa::Sair => break,
            }
        }

        Ok(())
    }

    fn menu(&mut self, terminal: &mut DefaultTerminal, menu: Menu) {
        match menu.run(terminal) {
            Ok(etapa) => self.etapa = etapa,
            Err(e) => {
                log::info!("Falha ao abrir menu: {e}");
            }
        }
    }

    fn categorizar(&mut self, terminal: &mut DefaultTerminal) {
        match Categorizador::default().run(terminal) {
            Ok(_) => {}
            Err(e) => {
                log::info!("Falha ao categorizar: {e}");
            }
        }
        self.etapa = Etapa::Menu
    }

    fn contracheque(&mut self, terminal: &mut DefaultTerminal) {
        match ContraCheque::default().run(terminal) {
            Ok(_) => {}
            Err(e) => {
                log::info!("Falha ao informar contra-cheque: {e}");
            }
        }
        self.etapa = Etapa::Menu
    }

    fn dash(&mut self, terminal: &mut DefaultTerminal) {
        match GeradorDash::new().run(terminal) {
            Ok(_) => {}
            Err(e) => {
                log::info!("Falha ao gerar dash: {e}");
            }
        }
        self.etapa = Etapa::Menu
    }

    fn dividas(&mut self, terminal: &mut DefaultTerminal) {
        match ListaDividas::default().run(terminal) {
            Ok(_) => {}
            Err(e) => {
                log::info!("Falha ao abrir dividas: {e}");
            }
        }
        self.etapa = Etapa::Menu
    }

    fn metas(&mut self, terminal: &mut DefaultTerminal) {
        match ListaMeta::default().run(terminal) {
            Ok(_) => {}
            Err(e) => {
                log::info!("Falha ao abrir metas: {e}");
            }
        }
        self.etapa = Etapa::Menu
    }

    fn configuracoes(&mut self, terminal: &mut DefaultTerminal) {
        match EditarConfiguracoes::default().run(terminal) {
            Ok(_) => {}
            Err(e) => {
                log::info!("Falha ao abrir configurações: {e}");
            }
        }
        self.etapa = Etapa::Menu
    }

    fn categorias(&mut self, terminal: &mut DefaultTerminal) {
        match ListaCategoria::default().run(terminal) {
            Ok(_) => {}
            Err(e) => {
                log::info!("Falha ao abrir categirias: {e}");
            }
        }
        self.etapa = Etapa::Menu
    }
}
