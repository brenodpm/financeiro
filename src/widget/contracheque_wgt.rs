use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, SLATE},
        Stylize,
    },
    widgets::{ListState, Paragraph, Widget},
    DefaultTerminal,
};
use sha1::digest::crypto_common::KeyInit;

use crate::{
    componentes::input_wgt::Input,
    dto::Configuracao,
    estilo::{principal_comandos, principal_titulo},
};
use color_eyre::Result;

#[derive(Clone, PartialEq)]
enum Bloco {
    Entrada,
    Saida,
}

#[derive(Clone, PartialEq)]
enum Ponto {
    Nome,
    Valor,
}

#[derive(Clone)]
struct Posicao {
    bloco: Bloco,
    ponto: Ponto,
    linha: usize,
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

        let mut entradas: Vec<ContraChequeItem> = config
            .contracheque_entradas
            .iter()
            .enumerate()
            .map(|(i, nome_item)| ContraChequeItem {
                nome: Input::new_texto("nome", nome_item.clone()),
                valor: Input::new_monetario("valor", 0.0),
            })
            .collect();

        let saidas: Vec<ContraChequeItem> = config
            .contracheque_saidas
            .iter()
            .enumerate()
            .map(|(i, nome_item)| ContraChequeItem {
                nome: Input::new_texto("nome", nome_item.clone()),
                valor: Input::new_monetario("valor", 0.0),
            })
            .collect();

        Self {
            sair: Default::default(),
            posicao: Posicao {
                bloco: Bloco::Entrada,
                ponto: Ponto::Nome,
                linha: 0,
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
                self.posicao.linha.to_string().as_str(),
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
            self.atualizar_listas();

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
            KeyCode::BackTab => self.para_esquerda(),
            KeyCode::Tab => self.para_direita(),
            KeyCode::Up => self.para_cima(),
            KeyCode::Down | KeyCode::Enter => self.para_baixo(),
            KeyCode::Delete => self.remover_item(),

            _ => self.alter_input(key),
        }
    }

    fn remover_item(&mut self) {
        match self.posicao.bloco {
            Bloco::Entrada => self.entradas.remove(self.posicao.linha),
            Bloco::Saida =>  self.saidas.remove(self.posicao.linha),
        };
        self.obedecer_o_ultimo();
    }

    fn para_esquerda(&mut self) {
        if self.posicao.ponto == Ponto::Valor {
            self.posicao.ponto = Ponto::Nome;
        } else if self.posicao.bloco == Bloco::Saida {
            self.posicao.bloco = Bloco::Entrada;
            self.posicao.ponto = Ponto::Valor;
        }
        self.obedecer_o_ultimo();
    }

    fn para_direita(&mut self) {
        if self.posicao.ponto == Ponto::Nome {
            self.posicao.ponto = Ponto::Valor;
        } else if self.posicao.bloco == Bloco::Entrada {
            self.posicao.bloco = Bloco::Saida;
            self.posicao.ponto = Ponto::Nome;
        }
        self.obedecer_o_ultimo();
    }

    fn obedecer_o_ultimo(&mut self) {
        while self.posicao.linha > 0
            && self.posicao.linha
                > (match self.posicao.bloco {
                    Bloco::Entrada => self.entradas.len(),
                    Bloco::Saida => self.saidas.len(),
                } - 1)
        {
            self.posicao.linha -= 1;
        }
    }

    fn para_cima(&mut self) {
        if self.posicao.linha > 0 {
            self.posicao.linha -= 1;
        }
    }

    fn para_baixo(&mut self) {
        if self.posicao.linha
            < (match self.posicao.bloco {
                Bloco::Entrada => self.entradas.len(),
                Bloco::Saida => self.saidas.len(),
            } - 1)
        {
            self.posicao.linha += 1;
        } else {
            self.posicao.linha = 0;
        }
    }

    fn alter_input(&mut self, key: KeyEvent) {
        match self.posicao.bloco {
            Bloco::Entrada => match self.posicao.ponto {
                Ponto::Nome => self.entradas[self.posicao.linha].nome.handle_key(key),
                Ponto::Valor => self.entradas[self.posicao.linha].valor.handle_key(key),
            },
            Bloco::Saida => match self.posicao.ponto {
                Ponto::Nome => self.saidas[self.posicao.linha].nome.handle_key(key),
                Ponto::Valor => self.saidas[self.posicao.linha].valor.handle_key(key),
            },
        }
    }

    fn atualizar_listas(&mut self) {
        if self.entradas.len() == 0 {
            self.entradas.push(ContraChequeItem {
                nome: Input::new_texto("Desc", String::new()),
                valor: Input::new_monetario("Valor", 0.0),
            });
        } else if let Some(entrada) = self.entradas.last() {
            if entrada.nome.to_string().len() != 0usize {
                self.entradas.push(ContraChequeItem {
                    nome: Input::new_texto("Desc", String::new()),
                    valor: Input::new_monetario("Valor", 0.0),
                });
            }
        }

        if self.saidas.len() == 0 {
            self.saidas.push(ContraChequeItem {
                nome: Input::new_texto("Desc", String::new()),
                valor: Input::new_monetario("Valor", 0.0),
            });
        } else if let Some(saida) = self.saidas.last() {
            if saida.nome.to_string().len() != 0usize {
                self.saidas.push(ContraChequeItem {
                    nome: Input::new_texto("Desc", String::new()),
                    valor: Input::new_monetario("Valor", 0.0),
                });
            }
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [valores, resumo] =
            Layout::vertical([Constraint::Fill(4), Constraint::Fill(1)]).areas(area);

        let [entradas, _separador, saidas] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(valores);

        Self::render_itens(
            &mut self.entradas,
            "ENTRADAS",
            entradas,
            buf,
            Bloco::Entrada,
            self.posicao.clone(),
        );
        Self::render_itens(
            &mut self.saidas,
            "SAÍDAS",
            saidas,
            buf,
            Bloco::Saida,
            self.posicao.clone(),
        );
        self.render_resumo(resumo, buf);
    }

    fn render_itens(
        itens: &mut Vec<ContraChequeItem>,
        nome: &str,
        area: Rect,
        buf: &mut Buffer,
        bloco: Bloco,
        posicao: Posicao,
    ) {
        let constraints = Self::build_constraints_for_rows_in_area(itens.iter_mut().count());
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        if let Some(rect) = chunks.get(0) {
            Paragraph::new(format!("\n{}", nome))
                .bold()
                .centered()
                .render(*rect, buf);
        }

        for (i, item) in itens.iter_mut().enumerate() {
            if let Some(rect) = chunks.get(i + 1) {
                let [nome, valor] =
                    Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).areas(*rect);
                item.nome.render(
                    Self::foco(posicao.clone(), i, bloco.clone(), Ponto::Nome),
                    nome,
                    buf,
                );
                item.valor.render(
                    Self::foco(posicao.clone(), i, bloco.clone(), Ponto::Valor),
                    valor,
                    buf,
                );
            }
        }
    }

    fn foco(posicao: Posicao, linha: usize, bloco: Bloco, ponto: Ponto) -> bool {
        posicao.linha == linha && posicao.ponto == ponto && posicao.bloco == bloco
    }

    fn render_resumo(&mut self, area: Rect, buf: &mut Buffer) {}

    fn build_constraints_for_rows_in_area(item_count: usize) -> Vec<Constraint> {
        let mut constraints = Vec::with_capacity(item_count + 1);

        constraints.push(Constraint::Length(2));
        for _ in 0..item_count {
            constraints.push(Constraint::Length(3));
        }
        constraints.push(Constraint::Fill(1));

        constraints
    }
}
