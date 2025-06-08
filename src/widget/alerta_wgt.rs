use color_eyre::eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Paragraph, Widget, Wrap},
    DefaultTerminal,
};

use crate::estilo::{
    principal_comandos, principal_titulo_alerta, GERAL_TEXT_FG,
};

pub struct Alerta {
    sair: bool,
    resp: bool,
    titulo: String,
    texto: Vec<String>,
}

impl Widget for &mut Alerta {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let tamanho: u16 = (self.texto.len() + 2).try_into().unwrap_or(u16::MAX);

         let [_esquerdo, caixa, _direito] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(3),
            Constraint::Fill(1),
        ])
        .areas(area);

        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(tamanho),
            Constraint::Length(1),
        ])
        .areas(caixa);

        principal_titulo_alerta(self.titulo.as_str(), titulo, buf);
        self.render_texto(corpo, buf);
        principal_comandos(vec!["ENTER (OK)", "ESC (Cancelar)"], rodape, buf);
    }
}

impl Alerta {
    pub fn atencao(texto: Vec<String>) -> Self {
        Self {
            sair: false,
            resp: false,
            titulo: "Atenção".to_string(),
            texto: texto,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<bool> {
        while !self.sair {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(self.resp)
    }

    fn render_texto(&self, area: Rect, buf: &mut Buffer) {
        let info = self.texto.join("\n");

        Paragraph::new(info)
            .fg(GERAL_TEXT_FG)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Enter => {
                self.resp = true;
                self.sair = true;
            }
            KeyCode::Esc => {
                self.resp = false;
                self.sair = true;
            },
            _ => {}
        }
    }
}
