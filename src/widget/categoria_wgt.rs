use color_eyre::eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
    DefaultTerminal,
};

use crate::{
    componentes::{
        input_wgt::Input,
        lista_suspensa::{ItemListaSuspensa, ListaSuspensa},
    },
    dto::{Categoria, GrupoDespesa, TipoDespesa, TipoFluxo, Unico},
    estilo::{principal_comandos, principal_titulo},
};

#[derive(PartialEq)]
enum Status {
    AltTipo,
    AltTipoDespesa,
    AltGrupo,
    AltNome,

    Sair(Option<Categoria>),
}

pub struct EditarCategoria {
    status: Status,
    id: String,

    nome: Input,
    tipo: ListaSuspensa,
    tipo_despesa: ListaSuspensa,
    grupo: Input,
}

impl Widget for &mut EditarCategoria {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo(
            if self.id.is_empty() {
                "Nova Categoria"
            } else {
                "Edição de Categoria"
            },
            titulo,
            buf,
        );
        principal_comandos(
            vec!["(Editar)", "TAB (próximo)", "Esc (sair)", "F5 (salvar)"],
            rodape,
            buf,
        );
        self.render(corpo, buf)
    }
}

impl EditarCategoria {
    pub fn new() -> Self {
        let tipos = vec![
            ItemListaSuspensa::new("Receita"),
            ItemListaSuspensa::new("Despesa"),
            ItemListaSuspensa::new("Investimento"),
            ItemListaSuspensa::new("Retorno"),
            ItemListaSuspensa::new2("Transferencias", "Transferências"),
            ItemListaSuspensa::new2("SemCategoria", "Sem categoria"),
        ];

        let tipo_despesas = vec![
            ItemListaSuspensa::new("Fixa"),
            ItemListaSuspensa::new2("Variavel", "Variável"),
            ItemListaSuspensa::new("Perda"),
        ];

        Self {
            status: Status::AltTipo,
            id: String::new(),

            nome: Input::new_texto("Nome", "".to_string()),
            tipo: ListaSuspensa::new("Tipo", tipos, true),
            tipo_despesa: ListaSuspensa::new("Tipo de despesa", tipo_despesas, true),
            grupo: Input::new_texto("Grupo", "".to_string()),
        }
    }

    pub fn set(cat: Categoria) -> Self {
        let mut resp = Self::new();

        resp.id = cat.id;
        resp.nome.set_texto(cat.nome);

        match cat.tipo {
            crate::dto::TipoFluxo::Receita(grupo) => {
                resp.tipo.set_id_selecionado("Receita".to_string());
                resp.grupo.set_texto(grupo);
            }
            crate::dto::TipoFluxo::Despesa(gd) => {
                resp.tipo.set_id_selecionado("Despesa".to_string());
                match gd.tipo {
                    crate::dto::TipoDespesa::Fixa => {
                        resp.tipo_despesa.set_id_selecionado("Fixa".to_string())
                    }
                    crate::dto::TipoDespesa::Variavel => {
                        resp.tipo_despesa.set_id_selecionado("Variavel".to_string())
                    }
                    crate::dto::TipoDespesa::Perda => {
                        resp.tipo_despesa.set_id_selecionado("Perda".to_string())
                    }
                    crate::dto::TipoDespesa::Vazio => {
                        resp.tipo_despesa.set_id_selecionado("Vazio".to_string())
                    }
                }
                resp.grupo.set_texto(gd.grupo);
            }
            crate::dto::TipoFluxo::Investimento => {
                resp.tipo.set_id_selecionado("Investimento".to_string())
            }
            crate::dto::TipoFluxo::Retorno => resp.tipo.set_id_selecionado("Retorno".to_string()),
            crate::dto::TipoFluxo::Transferencias => {
                resp.tipo.set_id_selecionado("Transferencias".to_string())
            }
            crate::dto::TipoFluxo::SemCategoria => {
                resp.tipo.set_id_selecionado("SemCategoria".to_string())
            }
        }

        resp
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Option<Categoria>> {
        while !matches!(self.status, Status::Sair(_)) {
            if let Err(erro) = terminal.draw(|frame| frame.render_widget(&mut self, frame.area())){
                log::error!("Erro ao desenhar tela EditarCategoria: {}", erro);
            };
            
            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            };
        }

        if let Status::Sair(meta) = self.status {
            return Ok(meta);
        }
        Ok(None)
    }

    pub fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Esc => self.status = Status::Sair(None),
            KeyCode::Tab => self.proximo_input(),
            KeyCode::BackTab => self.anterior_input(),
            KeyCode::F(5) => self.salvar(),
            _ => self.alterar_input(key, terminal),
        }
    }

    fn proximo_input(&mut self) {
        match self.status {
            Status::AltTipo => match self.tipo.get_id_selecionado().as_str() {
                "Receita" => self.status = Status::AltGrupo,
                "Despesa" => self.status = Status::AltTipoDespesa,
                "Investimento" | "Retorno" | "Transferencias" | "SemCategoria" => {
                    self.status = Status::AltNome
                }

                _ => {}
            },
            Status::AltTipoDespesa => {
                if !self.tipo_despesa.id_selecionado_eh(String::new()) {
                    self.status = Status::AltGrupo
                }
            }
            Status::AltGrupo => {
                if self.grupo.to_string().len() >= 3 {
                    self.status = Status::AltNome
                }
            }
            Status::AltNome => self.status = Status::AltTipo,

            Status::Sair(_) => {}
        }
    }

    fn anterior_input(&mut self) {
        match self.status {
            Status::AltNome => match self.tipo.get_id_selecionado().as_str() {
                "Receita" | "Despesa" => self.status = Status::AltGrupo,
                _ => self.status = Status::AltTipo,
            },
            Status::AltGrupo => match self.tipo.get_id_selecionado().as_str() {
                "Despesa" => self.status = Status::AltTipoDespesa,
                _ => self.status = Status::AltTipo,
            },
            Status::AltTipoDespesa => self.status = Status::AltTipo,

            Status::AltTipo | Status::Sair(_) => {}
        }
    }

    fn salvar(&mut self) {
        let mut categoria = Categoria {
            id: self.id.clone(),
            nome: self.nome.to_string(),
            tipo: match self.tipo.get_id_selecionado().as_str() {
                "Receita" => TipoFluxo::Receita(self.grupo.to_string()),
                "Despesa" => TipoFluxo::Despesa(GrupoDespesa {
                    grupo: self.grupo.to_string(),
                    tipo: match self.tipo_despesa.get_id_selecionado().as_str() {
                        "Fixa" => TipoDespesa::Fixa,
                        "Variavel" => TipoDespesa::Variavel,
                        "Perda" => TipoDespesa::Perda,
                        _ => TipoDespesa::Vazio,
                    },
                }),
                "Investimento" => TipoFluxo::Investimento,
                "Retorno" => TipoFluxo::Retorno,
                "Transferencias" => TipoFluxo::Transferencias,
                _ => TipoFluxo::SemCategoria,
            },
        };

        if categoria.id.is_empty() {
            categoria.gerar_id();
        }

        self.status = Status::Sair(Some(categoria));
    }

    fn alterar_input(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        match self.status {
            Status::AltTipo => {
                self.tipo.handle_key(key, terminal);
                self.tipo_despesa.set_id_selecionado(String::new());
                self.grupo.set_texto(String::new());
                self.nome.set_texto(String::new());
            }
            Status::AltTipoDespesa => {
                self.tipo_despesa.handle_key(key, terminal);
                self.grupo.set_texto(String::new());
                self.nome.set_texto(String::new());
            }
            Status::AltGrupo => {
                self.grupo.handle_key(key);
                self.nome.set_texto(String::new());
            }
            Status::AltNome => self.nome.handle_key(key),

            Status::Sair(_) => {}
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let mut mostrar_nome = false;

        let [linha1, linha2] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).areas(area);

        let [col1, col2, col3] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(linha1);

        self.tipo.render(self.status == Status::AltTipo, col1, buf);

        match self.tipo.get_id_selecionado().as_str() {
            "Receita" => {
                self.grupo
                    .render(self.status == Status::AltGrupo, col2, buf);
                mostrar_nome = self.grupo.to_string().len() >= 3;
            }
            "Despesa" => {
                self.tipo_despesa
                    .render(self.status == Status::AltTipoDespesa, col2, buf);

                if !self.tipo_despesa.id_selecionado_eh(String::new()) {
                    self.grupo
                        .render(self.status == Status::AltGrupo, col3, buf);
                    mostrar_nome = self.grupo.to_string().len() >= 3;
                }
            }
            "Investimento" | "Retorno" | "Transferencias" | "SemCategoria" => {
                mostrar_nome = true;
            }
            _ => {}
        }

        if mostrar_nome {
            self.nome
                .render(self.status == Status::AltNome, linha2, buf);
        }
    }
}
