use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

use crate::widget::{Categorizador, GeradorDash, ListaDividas, ListaMeta, Menu};

#[derive(Clone)]
pub enum Etapa {
    Categorizar,
    Dividas,
    Metas,
    Menu,
    Dash,
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
                ("Metas".to_string(), Etapa::Metas),
                ("Gerar Gráfico".to_string(), Etapa::Dash),
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
}
