use color_eyre::eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
    DefaultTerminal,
};

use crate::{
    componentes::input_wgt::Input,
    dto::Configuracao,
    estilo::{principal_comandos, principal_titulo},
};

#[derive(PartialEq)]
enum Status {
    EditarSalario,
    Endividamento,
    Sair,
}

pub struct EditarConfiguracoes {
    status: Status,

    salario: Input,
    endividamento_max: Input,
}

impl Default for EditarConfiguracoes {
    fn default() -> Self {
        let configs = Configuracao::buscar();

        Self {
            status: Status::EditarSalario,
            salario: Input::new_monetario("Salário", configs.salario),
            endividamento_max: Input::new_monetario("Endividamento máximo", configs.endividamento_max),
        }
    }
}

impl Widget for &mut EditarConfiguracoes {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Configurações", titulo, buf);
        principal_comandos(
            match self.status {
                Status::EditarSalario => vec!["Tab (Proximo)", "ESC Sair"],
                _ => vec!["Editar", "Tab (próximo)", "ESC Sair", "F5 (salvar)"],
            },
            rodape,
            buf,
        );
        self.render(corpo, buf)
    }
}

impl EditarConfiguracoes {
    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [linha1, linha2] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);

        self.salario
            .render(self.status == Status::EditarSalario, linha1, buf);
        self.endividamento_max
            .render(self.status == Status::Endividamento, linha2, buf);
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !matches!(self.status, Status::Sair) {
            if let Err(erro) = terminal.draw(|frame| frame.render_widget(&mut self, frame.area())){
                log::error!("Erro ao desenhar tela Editar Configurações: {}", erro);
            };

            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }

        if let Status::Sair = self.status {
            return Ok(());
        }
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Tab => self.proximo_input(),
            KeyCode::BackTab => self.anterior_input(),
            KeyCode::Esc => self.status = Status::Sair,
            _ => self.alterar_input(key),
        }
    }

    fn proximo_input(&mut self) {
        self.salvar();

        match self.status {
            Status::EditarSalario => self.status = Status::Endividamento,
            Status::Endividamento => self.status = Status::EditarSalario,
            Status::Sair => {}
        }
    }

    fn anterior_input(&mut self) {
        self.salvar();

        match self.status {
            Status::Endividamento => self.status = Status::EditarSalario,
            Status::EditarSalario => self.status = Status::Endividamento,
            Status::Sair => {}
        }
    }

    fn salvar(&mut self) {
        let mut conf = Configuracao::buscar();

        if conf.salario != self.salario.to_f64() {
            conf.salario = self.salario.to_f64();
            self.endividamento_max.set_monetario(self.salario.to_f64()*0.4);
        }
        conf.endividamento_max = self.endividamento_max.to_f64();

        Configuracao::salvar(&conf);
    }

    fn alterar_input(&mut self, key: KeyEvent) {
        match self.status {
            Status::EditarSalario => self.salario.handle_key(key),
            Status::Endividamento => self.endividamento_max.handle_key(key),
            Status::Sair => {}
        }
    }
}
