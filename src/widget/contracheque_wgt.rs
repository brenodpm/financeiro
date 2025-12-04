use chrono::Utc;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    widgets::{Paragraph, Widget},
    DefaultTerminal,
};

use crate::{
    app::Etapa,
    componentes::input_wgt::Input,
    dto::{Banco, Configuracao, Conta, Lancamento, OptionalLazy, Unico},
    estilo::{principal_comandos, principal_titulo},
    widget::alerta_wgt::Alerta,
};
use color_eyre::Result;

#[derive(Clone, PartialEq)]
struct Tupla {
    linha: usize,
    coluna: usize,
}

#[derive(Clone, PartialEq)]
enum Editar {
    Empresa,
    Data,
    Tupla(Tupla),
}

enum Bloco {
    Entrada,
    Saida,
}
enum Tipo {
    Nome,
    Valor,
}

impl Editar {
    fn foco(&self, linha: usize, bloco: &Bloco, tipo: Tipo) -> bool {
        if let Editar::Tupla(tupla) = self {
            tupla.eq(&Tupla {
                linha,
                coluna: match bloco {
                    Bloco::Entrada => 0,
                    Bloco::Saida => 2,
                } + match tipo {
                    Tipo::Nome => 0,
                    Tipo::Valor => 1,
                },
            })
        } else {
            false
        }
    }
}

struct ContraChequeItem {
    nome: Input,
    valor: Input,
}
pub struct ContraCheque {
    sair: bool,
    editar: Editar,
    empresa: Input,
    data_pagamento: Input,
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
            .map(|(_, nome_item)| ContraChequeItem {
                nome: Input::new_texto("nome", nome_item.clone()),
                valor: Input::new_monetario("valor", 0.0),
            })
            .collect();

        let saidas: Vec<ContraChequeItem> = config
            .contracheque_saidas
            .iter()
            .enumerate()
            .map(|(_, nome_item)| ContraChequeItem {
                nome: Input::new_texto("nome", nome_item.clone()),
                valor: Input::new_monetario("valor", 0.0),
            })
            .collect();

        Self {
            sair: Default::default(),
            empresa: Input::new_texto("Empresa", config.contracheque_empresa),
            data_pagamento: Input::new_data(
                "Empresa",
                Utc::now().naive_utc().date().format("%d/%m/%y").to_string(),
            ),
            editar: Editar::Empresa,
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
            vec!["Tab e ↓↑ (mover)", "ESC Sair", "F5 (salvar)"],
            rodape,
            buf,
        );
        self.render(corpo, buf)
    }
}

impl ContraCheque {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Etapa> {
        while !self.sair {
            self.atualizar_listas();

            if let Err(erro) = terminal.draw(|frame| frame.render_widget(&mut self, frame.area())){
                log::error!("Erro ao desenhar tela Contracheque: {}", erro);
            };
            
            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            };
        }

        Ok(Etapa::Categorizar)
    }

    pub fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::F(5) => self.confirmar_concusao(terminal),
            KeyCode::Esc => self.sair(terminal),

            _ => match &self.editar {
                Editar::Empresa => self.handle_key_alt_empresa(key),
                Editar::Data => self.handle_key_alt_data(key),
                Editar::Tupla(_) => self.handle_key_alt_coluna(key),
            },
        }
    }

    fn confirmar_concusao(&mut self, terminal: &mut DefaultTerminal) {
        let entradas: f64 = self.entradas.iter().map(|c| c.valor.to_f64()).sum();
        let saidas: f64 = self.saidas.iter().map(|c| c.valor.to_f64()).sum();
        let total = entradas - saidas;

        if let Ok(resp) = Alerta::atencao(vec![
            "Certifique-se que todos os dados estão corretos".to_string(),
            String::new(),
            format!("Entradas:      R$ {:0.02}", entradas),
            format!("Saídas:        R$ {:0.02}", saidas),
            format!("Total:         R$ {:0.02}", total),
        ])
        .run(terminal)
        {
            if resp {
                self.salvar(total);
                self.sair = true;
            }
        }
    }

    fn salvar(&self, total: f64) {
        let conta = buscar_conta(self.empresa.to_string());
        let data = self.data_pagamento.to_naivedate().unwrap();
        let mut lancamentos: Vec<Lancamento> = Vec::new();

        let mut nome_entradas: Vec<String> = Vec::new();
        self.entradas.iter().for_each(|f| {
            if !f.nome.to_string().is_empty() && f.valor.to_f64() != 0f64 {
                lancamentos.push(Lancamento {
                    id: String::new(),
                    descricao: f.nome.to_string(),
                    valor: f.valor.to_f64(),
                    data: data,
                    categoria: OptionalLazy::None,
                    conta: Some(conta.id.clone()),
                    regra: OptionalLazy::None,
                });
                nome_entradas.push(f.nome.to_string());
            }
        });

        let mut nome_saidas: Vec<String> = Vec::new();
        self.saidas.iter().for_each(|f| {
            if !f.nome.to_string().is_empty() && f.valor.to_f64() != 0f64 {
                lancamentos.push(Lancamento {
                    id: String::new(),
                    descricao: f.nome.to_string(),
                    valor: f.valor.to_f64() * -1f64,
                    data: data,
                    categoria: OptionalLazy::None,
                    conta: Some(conta.id.clone()),
                    regra: OptionalLazy::None,
                });
                nome_saidas.push(f.nome.to_string());
            }
        });

        lancamentos.push(Lancamento {
            id: String::new(),
            descricao: "Transferencia de Salário Liquido".to_string(),
            valor: total,
            data: data,
            categoria: OptionalLazy::None,
            conta: Some(conta.id.clone()),
            regra: OptionalLazy::None,
        });

        lancamentos.iter_mut().for_each(|l| {
            l.gerar_id();
        });

        Lancamento::categorizar(&lancamentos);
        Configuracao::atualizar_contracheque(
            self.empresa.to_string(),
            nome_entradas,
            nome_saidas,
        );
    }

    fn sair(&mut self, terminal: &mut DefaultTerminal) {
        if let Ok(resp) = Alerta::atencao(vec![
            "Tem certeza que deseja sair?".to_string(),
            "todos os dados já preenchidos serão perdidos".to_string(),
        ])
        .run(terminal)
        {
            self.sair = resp;
        }
    }

    fn handle_key_alt_empresa(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab | KeyCode::Enter => self.editar = Editar::Data,

            _ => self.empresa.handle_key(key),
        }
    }

    fn handle_key_alt_data(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::BackTab => self.editar = Editar::Empresa,
            KeyCode::Tab | KeyCode::Enter => {
                self.editar = Editar::Tupla(Tupla {
                    coluna: 0,
                    linha: 0,
                })
            }

            _ => self.data_pagamento.handle_key(key),
        }
    }

    fn handle_key_alt_coluna(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab => self.para_direita(),
            KeyCode::BackTab => self.para_esquerda(),
            KeyCode::Up => self.para_cima(),
            KeyCode::Down => self.para_baixo(false),
            KeyCode::Enter => self.para_baixo(true),
            KeyCode::Delete => self.remover_item(),

            _ => self.alter_input(key),
        }
    }

    fn remover_item(&mut self) {
        if let Editar::Tupla(mut tupla) = self.editar.clone() {
            if tupla.coluna < 2 {
                self.entradas.remove(tupla.linha);
            } else {
                self.saidas.remove(tupla.linha);
            }
            self.obedecer_o_ultimo(&mut tupla);
            self.editar = Editar::Tupla(tupla);
        }
    }

    fn para_esquerda(&mut self) {
        if let Editar::Tupla(mut tupla) = self.editar.clone() {
            if tupla.coluna > 0 {
                tupla.coluna -= 1;
            } else {
                tupla.coluna = 3;
            }

            self.obedecer_o_ultimo(&mut tupla);
            self.editar = Editar::Tupla(tupla);
        }
    }

    fn para_direita(&mut self) {
        if let Editar::Tupla(mut tupla) = self.editar.clone() {
            if tupla.coluna < 3 {
                tupla.coluna += 1;
            } else {
                tupla.coluna = 0;
                tupla.linha += 1;
            }

            self.obedecer_o_ultimo(&mut tupla);
            self.editar = Editar::Tupla(tupla);
        }
    }

    fn para_cima(&mut self) {
        if let Editar::Tupla(mut tupla) = self.editar.clone() {
            if tupla.linha > 0 {
                tupla.linha -= 1;
                self.obedecer_o_ultimo(&mut tupla);
                self.editar = Editar::Tupla(tupla);
            } else {
                self.editar = Editar::Data;
            }
        }
    }

    fn para_baixo(&mut self, quebra: bool) {
        if let Editar::Tupla(mut tupla) = self.editar.clone() {
            tupla.linha += 1;
            if quebra && (tupla.coluna == 1 || tupla.coluna == 3) {
                tupla.coluna -= 1;
            }
            self.obedecer_o_ultimo(&mut tupla);
            self.editar = Editar::Tupla(tupla);
        }
    }

    fn obedecer_o_ultimo(&mut self, tupla: &mut Tupla) {
        if tupla.coluna < 2 {
            while tupla.linha >= self.entradas.len() {
                tupla.linha -= 1;
            }
        } else {
            while tupla.linha >= self.saidas.len() {
                tupla.linha -= 1;
            }
        }
    }

    fn alter_input(&mut self, key: KeyEvent) {
        if let Editar::Tupla(tupla) = self.editar.clone() {
            match tupla.coluna {
                0 => self.entradas[tupla.linha].nome.handle_key(key),
                1 => self.entradas[tupla.linha].valor.handle_key(key),
                2 => self.saidas[tupla.linha].nome.handle_key(key),
                3 => self.saidas[tupla.linha].valor.handle_key(key),
                _ => {}
            }
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
        let [cabecalho, valores, resumo] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(6),
        ])
        .areas(area);

        let [empresa, data, _] = Layout::horizontal([
            Constraint::Fill(2),
            Constraint::Length(15),
            Constraint::Fill(3),
        ])
        .areas(cabecalho);

        self.empresa
            .render(self.editar == Editar::Empresa, empresa, buf);
        self.data_pagamento
            .render(self.editar == Editar::Data, data, buf);

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
            self.editar.clone(),
        );
        Self::render_itens(
            &mut self.saidas,
            "SAÍDAS",
            saidas,
            buf,
            Bloco::Saida,
            self.editar.clone(),
        );
        self.render_resumo(resumo, buf);
    }

    fn render_itens(
        itens: &mut Vec<ContraChequeItem>,
        nome: &str,
        area: Rect,
        buf: &mut Buffer,
        bloco: Bloco,
        editar: Editar,
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
                item.nome
                    .render(editar.foco(i, &bloco, Tipo::Nome), nome, buf);
                item.valor
                    .render(editar.foco(i, &bloco, Tipo::Valor), valor, buf);
            }
        }
    }

    fn render_resumo(&mut self, area: Rect, buf: &mut Buffer) {
        let [_, resumo] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(30)]).areas(area);

        let [entradas_rect, saidas_rect, total_rect] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .areas(resumo);

        let entradas: f64 = self.entradas.iter().map(|c| c.valor.to_f64()).sum();
        let saidas: f64 = self.saidas.iter().map(|c| c.valor.to_f64()).sum();
        let total = entradas - saidas;

        Paragraph::new(format!("Entradas:      R$ {:0.02}", entradas))
            .bold()
            .render(entradas_rect, buf);

        Paragraph::new(format!("Saídas:        R$ {:0.02}", saidas))
            .bold()
            .render(saidas_rect, buf);

        Paragraph::new(format!("Total:         R$ {:0.02}", total))
            .bold()
            .render(total_rect, buf);
    }

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

fn buscar_conta(nome: String) -> Conta {
    let mut banco = if let Some(atual) = Banco::buscar_id("Salario".to_string()) {
        atual
    } else {
        let novo = Banco::novo("Salario".to_string());
        Banco::salvar(novo.clone());
        novo
    };

    if let Some(conta) = banco.contas.iter().find(|c| c.id == nome.clone()) {
        conta.clone()
    } else {
        let conta = Conta {
            id: nome.clone(),
            nome: nome.clone(),
        };
        banco.contas.push(conta.clone());
        Banco::salvar(banco);
        conta
    }
}
