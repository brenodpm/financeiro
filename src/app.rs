use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;

use crate::widget::Categorizador;

pub enum Etapa {
    Categorizar,
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
        loop {
            match self.etapa {
                Etapa::Categorizar => self.categorizar(&mut terminal),
                Etapa::Sair => break,
            }
        }

        Ok(())
    }

fn categorizar(&mut self, terminal: &mut DefaultTerminal) {
        match Categorizador::default().run(terminal) {
            Ok(_) => self.etapa = Etapa::Sair,
            Err(e) => {
                log::info!("Falha ao categorizar: {e}");
            }
        }
    }
}
