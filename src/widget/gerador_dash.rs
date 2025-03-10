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
    estilo::{principal_comandos, principal_titulo},
    repository::atualizar_base,
};

#[derive(PartialEq)]
enum Etapa {
    Iniciando,
    Base,
    Finalizado,
    Sair,
}

pub struct GeradorDash {
    etapa: Etapa,

    base: Check,
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
        principal_comandos(
            match self.etapa {
                Etapa::Iniciando => vec!["Iniciando..."],
                Etapa::Base => vec!["Atualizando base dos gráficos"],
                Etapa::Finalizado => vec!["Pressione qualquer tecla para sair"],
                Etapa::Sair => vec!["Saindo..."],
            },
            rodape,
            buf,
        );
        self.render_list(corpo, buf);
    }
}

impl GeradorDash {
    pub fn new() -> Self {
        Self {
            etapa: Etapa::Iniciando,
            base: Check::new("Base dos gráficos", false),
            fim: Check::new("Encerrar", false),
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
        let [base, fim] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);

        self.base.render(self.etapa == Etapa::Base, base, buf);
        self.fim.render(self.etapa == Etapa::Finalizado, fim, buf);
    }

    fn executar_etapa(&mut self) {
        match self.etapa {
            Etapa::Iniciando => self.inicializar(),
            Etapa::Base => self.atualizar_base(),

            Etapa::Finalizado | Etapa::Sair => {}
        }
    }

    fn inicializar(&mut self) {
        self.etapa = Etapa::Base;
    }

    fn atualizar_base(&mut self) {
        atualizar_base();
        self.base.set_checked(true);

        self.etapa = Etapa::Finalizado;
    }
}
