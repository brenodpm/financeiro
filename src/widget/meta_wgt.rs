use color_eyre::eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::
        Stylize
    ,
    widgets::{
        Paragraph, Widget,
    },
    DefaultTerminal,
};

use crate::dto::{Banco, Categoria, Meta, Unico};

use super::{
    check_wgt::Check,
    input_wgt::Input,
    lista_suspensa::{ItemListaSuspensa, ListaSuspensa},
};

#[derive(PartialEq)]
enum Status {
    AltNome,
    AltAtivo,
    AltDescricao,

    AltTipoMeta,
    AltFiltroMeta,

    AltFluxo,
    AltMetrica,
    AltPeriodo,
    AltValor,

    Sair(Option<Meta>),
}

pub struct EditarMeta {
    status: Status,
    id_meta: String,

    nome: Input,
    ativo: Check,
    desc: Input,

    meta_tipo: ListaSuspensa,
    meta_filtro: ListaSuspensa,

    fluxo: ListaSuspensa,
    metrica: ListaSuspensa,
    periodo: ListaSuspensa,
    valor: Input
}

impl Widget for &mut EditarMeta {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        EditarMeta::render_header(header_area, buf);
        EditarMeta::render_footer(footer_area, buf);
        self.render(main_area, buf)
    }
}

impl EditarMeta {
    pub fn new() -> Self {
        let tipos_metas = vec!["Conta", "Categoria"];

        let fluxos = vec!["entrada", "saída"];

        let metrica = vec![ "menor que", "marior que"];

        let periodos = vec![
            "mensal",
            "anual",
            "ult. 30 dias",
            "ult. 60 dias",
            "ult. 90 dias",
            "ult. 365 dias",
        ];

        Self {
            status: Status::AltNome,
            id_meta: String::new(),

            nome: Input::new_texto("Nome", "".to_string()),
            ativo: Check::new("Ativo", true),
            desc: Input::new_multilinhas("Descrição", "".to_string()),

            meta_tipo: ListaSuspensa::new_string("Tipo", tipos_metas),
            meta_filtro: ListaSuspensa::new("Filtro", Vec::new()),

            fluxo: ListaSuspensa::new_string("Fluxo", fluxos),
            metrica: ListaSuspensa::new_string("Métrica", metrica),
            periodo: ListaSuspensa::new_string("Período", periodos),
            valor: Input::new_monetario("Valor", 0.0f64)
        }
    }

    pub fn set(meta: Meta)-> Self{
        let mut resp = Self::new();

        resp.id_meta = meta.id;
        resp.nome.set_texto(meta.nome);
        resp.desc.set_texto(meta.desc);
        resp.meta_tipo.set_id_selecionado(meta.tipo_meta);
        resp.meta_filtro.set_id_selecionado(meta.filtro);
        resp.metrica.set_id_selecionado(meta.metrica);
        resp.fluxo.set_id_selecionado(meta.fluxo);
        resp.periodo.set_id_selecionado(meta.periodo);
        resp.valor.set_monetario(meta.valor);

        resp
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Option<Meta>> {
        while !matches!(self.status, Status::Sair(_)) {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            };
        }

        if let Status::Sair(meta) = self.status {
            return Ok(meta);
        }
        Ok(None)
    }

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Financeiro")
            .bold()
            .centered()
            .render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ mover, → selecionar categoria, ESC sair")
            .centered()
            .render(area, buf);
    }

    pub fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Esc => self.status = Status::Sair(None),
            KeyCode::Tab => self.proximo_input(),
            KeyCode::BackTab => self.anterior_input(),
            KeyCode::Insert => self.salvar(),
            _ => self.alterar_input(key, terminal),
        }
    }

    fn salvar(&mut self) {
        let mut meta = Meta {
            id: self.id_meta.clone(),
            nome: self.nome.to_string(),
            desc: self.desc.to_string(),
            tipo_meta: self.meta_tipo.get_id_selecionado(),
            filtro: self.meta_filtro.get_id_selecionado(),
            metrica: self.metrica.get_id_selecionado(),
            fluxo: self.fluxo.get_id_selecionado(),
            periodo: self.periodo.get_id_selecionado(),
            valor: self.valor.to_f64(),
        };

        if meta.id.is_empty() {
            meta.gerar_id();
        }

        self.status = Status::Sair(Some(meta));
    }

    fn proximo_input(&mut self) {
        match self.status {
            Status::AltNome => self.status = Status::AltAtivo,
            Status::AltAtivo => self.status = Status::AltDescricao,
            Status::AltDescricao => self.status = Status::AltTipoMeta,
            Status::AltTipoMeta => self.status = Status::AltFiltroMeta,
            Status::AltFiltroMeta => self.status = Status::AltFluxo,
            Status::AltFluxo => self.status = Status::AltMetrica,
            Status::AltMetrica => self.status = Status::AltPeriodo,
            Status::AltPeriodo => self.status = Status::AltValor,
            Status::AltValor => self.status = Status::AltNome,

            Status::Sair(_) => {}
        }
    }

    fn anterior_input(&mut self) {
        match self.status {
            Status::AltNome => self.status = Status::AltValor,
            Status::AltAtivo => self.status = Status::AltNome,
            Status::AltDescricao => self.status = Status::AltAtivo,
            Status::AltTipoMeta => self.status = Status::AltDescricao,
            Status::AltFiltroMeta => self.status = Status::AltTipoMeta,
            Status::AltFluxo => self.status = Status::AltFiltroMeta,
            Status::AltMetrica => self.status = Status::AltFluxo,
            Status::AltPeriodo => self.status = Status::AltMetrica,
            Status::AltValor => self.status = Status::AltPeriodo,

            Status::Sair(_) => {}
        }
    }

    fn alterar_input(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        match self.status {
            Status::AltNome => match key.code {
                KeyCode::Down => self.status = Status::AltDescricao,
                KeyCode::Right => self.status = Status::AltAtivo,
                _ => self.nome.handle_key(key),
            },
            Status::AltAtivo => self.ativo.handle_key(key),
            Status::AltDescricao => self.desc.handle_key(key),
            Status::AltFluxo => self.fluxo.handle_key(key, terminal),
            Status::AltTipoMeta => self.meta_tipo.handle_key(key, terminal),
            Status::AltFiltroMeta => self.meta_filtro.handle_key(key, terminal),
            Status::AltMetrica => self.metrica.handle_key(key, terminal),
            Status::AltPeriodo => self.periodo.handle_key(key, terminal),
            Status::AltValor => self.valor.handle_key(key),
            Status::Sair(_) => {}
        }
    }

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        let [linha_1, linha_2, linha_3, linha_4] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(6),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .areas(area);

        self.render_linha_1(buf, linha_1);

        self.desc
            .render(self.status == Status::AltDescricao, linha_2, buf);

        self.render_linha_3(buf, linha_3);
        self.render_linha_4(buf, linha_4);
    }

    fn render_linha_1(&mut self, buf: &mut Buffer, linha_1: Rect) {
        let [nome, ativo] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(linha_1);

        self.nome.render(self.status == Status::AltNome, nome, buf);
        self.ativo
            .render(self.status == Status::AltAtivo, ativo, buf);
    }

    fn render_linha_3(&mut self, buf: &mut Buffer, linha_3: Rect) {
        let [tipo_meta, filtro] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(3)]).areas(linha_3);

        self.meta_tipo
            .render(self.status == Status::AltTipoMeta, tipo_meta, buf);

        if self.meta_tipo.texto_selecionado_eh("") {
            if self.meta_filtro.nome != "Filtro" {
                self.meta_filtro.nome = "Filtro".to_string();
                self.meta_filtro.set_lista(Vec::new());
            }
        } else if self.meta_tipo.texto_selecionado_eh("Conta") {
            if self.meta_filtro.nome != "Conta" {
                let mut itens: Vec<ItemListaSuspensa> = Vec::new();
                Banco::listar().iter().for_each(|b| {
                    b.contas.iter().for_each(|c| {
                        itens.push(ItemListaSuspensa {
                            id: format!("{}:{}", b.id, c.id),
                            texto: format!("{} - {}", b.nome, c.nome),
                        });
                    });
                });
                self.meta_filtro.nome = "Conta".to_string();
                self.meta_filtro.set_lista(itens);
            }
        } else if self.meta_tipo.texto_selecionado_eh("Categoria") {
            if self.meta_filtro.nome != "Categoria" {
                let itens: Vec<ItemListaSuspensa> = Categoria::listar()
                    .into_iter()
                    .map(|c| ItemListaSuspensa {
                        id: c.id.clone(),
                        texto: c.to_string(),
                    })
                    .collect();

                self.meta_filtro.nome = "Categoria".to_string();
                self.meta_filtro.set_lista(itens);
            }
        }

        self.meta_filtro
            .render(self.status == Status::AltFiltroMeta, filtro, buf);
    }

    fn render_linha_4(&mut self, buf: &mut Buffer, linha_4: Rect) {
        let [fluxo, metrica, periodo, valor] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .areas(linha_4);

        self.fluxo
            .render(self.status == Status::AltFluxo, fluxo, buf);

        self.metrica
            .render(self.status == Status::AltMetrica, metrica, buf);

        self.periodo
            .render(self.status == Status::AltPeriodo, periodo, buf);

        self.valor
            .render(self.status == Status::AltValor, valor, buf);
    }
}
