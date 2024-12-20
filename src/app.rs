use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

use crate::widget::{Categorizador, ListaDividas, Menu};

#[derive(Clone)]
pub enum Etapa {
    Categorizar,
    Dividas,
    Menu,
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
                ("Sair".to_string(), Etapa::Sair),
            ],
            etapa: Option::None,
            state: Default::default(),
        };

        loop {
            match self.etapa {
                Etapa::Categorizar => self.categorizar(&mut terminal),
                Etapa::Menu => self.menu(&mut terminal, menu.clone()),
                Etapa::Dividas => self.dividas(&mut terminal),
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
            Ok(_) => self.etapa = Etapa::Menu,
            Err(e) => {
                log::info!("Falha ao categorizar: {e}");
            }
        }
    }

    fn dividas(&mut self, terminal: &mut DefaultTerminal) {
        match ListaDividas::default().run(terminal) {
            Ok(_) => self.etapa = Etapa::Menu,
            Err(e) => {
                log::info!("Falha ao abrir dividas: {e}");
            }
        }
    }
}
