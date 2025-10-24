use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::{ListState, Widget},
    DefaultTerminal,
};

use crate::{
    componentes::input_wgt::Input,
    dto::Configuracao,
    estilo::{principal_comandos, principal_titulo},
};
use color_eyre::Result;
struct Posicao {
    entradas: bool,
    linha: u8,
    valor: bool,
}
struct ContraChequeItem {
    nome: Input,
    valor: Input,
}
pub struct ContraCheque {
    sair: bool,
    posicao: Posicao,
    entradas: Vec<ContraChequeItem>,
    saidas: Vec<ContraChequeItem>,
}

impl Default for ContraCheque {
    fn default() -> Self {
        let config = Configuracao::buscar();

        let entradas: Vec<ContraChequeItem> = config
            .contracheque_entradas
            .iter()
            .enumerate()
            .map(|(i, nome_item)| ContraChequeItem {
                nome: Input::new_texto("nome", nome_item.clone()),
                valor: Input::new_monetario("valor", 0.0),
            })
            .collect();

        let saidas: Vec<ContraChequeItem> = Vec::new();

        Self {
            sair: Default::default(),
            posicao: Posicao {
                entradas: true,
                linha: 0,
                valor: true,
            },
            entradas: entradas,
            saidas: saidas,
        }
    }
}

impl Widget for &mut ContraCheque {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Adicionar Contra-cheque", titulo, buf);
        principal_comandos(
            vec![
                "↓↑ (mover)",
                "Enter (alterar pago)",
                "ESC Sair",
                "F5 (salvar)",
            ],
            rodape,
            buf,
        );
        self.render(corpo, buf)
    }
}

impl ContraCheque {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.sair {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }

        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            //KeyCode::F(5) => self.salvar(),
            KeyCode::Esc => self.sair = true,
            _ => {}
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [titulo, parcelas, resumo, lista_parcelas] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Length(5),
            Constraint::Fill(1),
        ])
        .areas(area);

        // self.render_titulo(titulo, buf);
        // self.render_parcelas(parcelas, buf);
        // self.render_resumo(resumo, buf);
        // self.render_lista_parcelas(lista_parcelas, buf);
    }
}
