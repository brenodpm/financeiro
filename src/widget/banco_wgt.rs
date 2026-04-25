use color_eyre::eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget,
    },
    DefaultTerminal,
};

use crate::{
    componentes::input_wgt::Input,
    dto::{Banco, Conta},
    estilo::{
        alternate_colors, principal_comandos, principal_titulo, GERAL_BG, GERAL_TEXT_FG,
        LISTA_BORDA_ESTILO, LISTA_SELECIONADO_ESTILO,
    },
};

// ─── Tela 1: Lista de Bancos ─────────────────────────────────────────────────

pub struct ListaBancos {
    sair: bool,
    bancos: Vec<Banco>,
    state: ListState,
}

impl Default for ListaBancos {
    fn default() -> Self {
        Self { sair: false, bancos: Banco::listar(), state: Default::default() }
    }
}

impl Widget for &mut ListaBancos {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);
        principal_titulo("Bancos e Contas", titulo, buf);
        principal_comandos(
            vec!["↓↑ mover", "N novo", "ENTER editar", "DEL remover", "ESC sair"],
            rodape,
            buf,
        );
        render_lista_bancos(&self.bancos, corpo, buf, &mut self.state, true);
    }
}

impl ListaBancos {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.state.select_first();
        while !self.sair {
            if let Err(e) = terminal.draw(|f| f.render_widget(&mut self, f.area())) {
                log::error!("Erro ao desenhar ListaBancos: {}", e);
            }
            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.sair = true,
            KeyCode::Down => self.state.select_next(),
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Char('n') | KeyCode::Char('N') => {
                if let Ok(Some(banco)) = EditarBanco::new().run(terminal) {
                    Banco::salvar(banco);
                    self.bancos = Banco::listar();
                }
            }
            KeyCode::Enter | KeyCode::Right => {
                if let Some(i) = self.state.selected() {
                    let banco = self.bancos[i].clone();
                    if let Ok(Some(banco)) = EditarBanco::set(banco).run(terminal) {
                        let mut lista = self.bancos.clone();
                        lista[i] = banco;
                        Banco::salvar_tudo(lista);
                        self.bancos = Banco::listar();
                    }
                }
            }
            KeyCode::Delete => {
                if let Some(i) = self.state.selected() {
                    self.bancos.remove(i);
                    Banco::salvar_tudo(self.bancos.clone());
                    self.state.select_first();
                }
            }
            _ => {}
        }
    }
}

fn render_lista_bancos(
    bancos: &[Banco],
    area: Rect,
    buf: &mut Buffer,
    state: &mut ListState,
    ativo: bool,
) {
    let borda = if ativo { LISTA_BORDA_ESTILO } else { LISTA_BORDA_ESTILO.dim() };
    let block = Block::new()
        .title(Line::raw("Bancos").centered())
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .border_style(borda)
        .bg(GERAL_BG);

    let items: Vec<ListItem> = bancos
        .iter()
        .enumerate()
        .map(|(i, b)| {
            let texto = format!(" {} [{}] ({} conta(s))", b.nome, b.id, b.contas.len());
            ListItem::new(Line::styled(texto, GERAL_TEXT_FG)).bg(alternate_colors(i))
        })
        .collect();

    StatefulWidget::render(
        List::new(items)
            .block(block)
            .highlight_style(LISTA_SELECIONADO_ESTILO)
            .highlight_symbol("▶")
            .highlight_spacing(HighlightSpacing::Always),
        area,
        buf,
        state,
    );
}

// ─── Tela 2: Editar Banco (com lista de contas) ───────────────────────────────

#[derive(PartialEq)]
enum StatusBanco {
    AltNome,
    AltId,
    AltContas,
    Sair(Option<Banco>),
}

pub struct EditarBanco {
    status: StatusBanco,
    banco: Banco,
    nome: Input,
    id: Input,
    conta_state: ListState,
}

impl EditarBanco {
    pub fn new() -> Self {
        Self {
            status: StatusBanco::AltNome,
            banco: Banco::default(),
            nome: Input::new_texto("Nome do banco", String::new()),
            id: Input::new_texto("ID do banco", String::new()),
            conta_state: Default::default(),
        }
    }

    pub fn set(banco: Banco) -> Self {
        let mut s = Self::new();
        s.nome = Input::new_texto("Nome do banco", banco.nome.clone());
        s.id = Input::new_texto("ID do banco", banco.id.clone());
        s.banco = banco;
        s.conta_state.select_first();
        s
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Option<Banco>> {
        while !matches!(self.status, StatusBanco::Sair(_)) {
            if let Err(e) = terminal.draw(|f| f.render_widget(&mut self, f.area())) {
                log::error!("Erro ao desenhar EditarBanco: {}", e);
            }
            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            }
        }
        if let StatusBanco::Sair(banco) = self.status {
            return Ok(banco);
        }
        Ok(None)
    }

    fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.status = StatusBanco::Sair(None),
            KeyCode::Tab => self.proximo(),
            KeyCode::BackTab => self.anterior(),
            KeyCode::F(5) => self.salvar(),
            _ => self.alterar_input(key, terminal),
        }
    }

    fn alterar_input(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        match self.status {
            StatusBanco::AltNome => self.nome.handle_key(key),
            StatusBanco::AltId => self.id.handle_key(key),
            StatusBanco::AltContas => match key.code {
                KeyCode::Down => self.conta_state.select_next(),
                KeyCode::Up => self.conta_state.select_previous(),
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    if let Ok(Some(conta)) = EditarConta::new().run(terminal) {
                        self.banco.contas.push(conta);
                    }
                }
                KeyCode::Enter | KeyCode::Right => {
                    if let Some(i) = self.conta_state.selected() {
                        if let Some(conta) = self.banco.contas.get(i).cloned() {
                            if let Ok(Some(conta)) = EditarConta::set(conta).run(terminal) {
                                self.banco.contas[i] = conta;
                            }
                        }
                    }
                }
                KeyCode::Delete => {
                    if let Some(i) = self.conta_state.selected() {
                        if i < self.banco.contas.len() {
                            self.banco.contas.remove(i);
                            self.conta_state.select_first();
                        }
                    }
                }
                _ => {}
            },
            StatusBanco::Sair(_) => {}
        }
    }

    fn proximo(&mut self) {
        self.status = match self.status {
            StatusBanco::AltNome => StatusBanco::AltId,
            StatusBanco::AltId => StatusBanco::AltContas,
            StatusBanco::AltContas => StatusBanco::AltNome,
            StatusBanco::Sair(_) => return,
        };
    }

    fn anterior(&mut self) {
        self.status = match self.status {
            StatusBanco::AltNome => StatusBanco::AltContas,
            StatusBanco::AltId => StatusBanco::AltNome,
            StatusBanco::AltContas => StatusBanco::AltId,
            StatusBanco::Sair(_) => return,
        };
    }

    fn salvar(&mut self) {
        let mut banco = self.banco.clone();
        banco.nome = self.nome.to_string();
        let id_str = self.id.to_string();
        banco.id = if id_str.is_empty() { gerar_id(&banco.nome) } else { id_str };
        self.status = StatusBanco::Sair(Some(banco));
    }
}

impl Widget for &mut EditarBanco {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let titulo_str = if self.banco.id.is_empty() { "Novo Banco" } else { "Editar Banco" };
        principal_titulo(titulo_str, titulo, buf);
        principal_comandos(
            vec!["TAB próximo", "N nova conta", "ENTER editar conta", "DEL remover", "F5 salvar", "ESC sair"],
            rodape,
            buf,
        );

        let [linha_campos, linha_contas] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(corpo);

        let [area_nome, area_id] =
            Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).areas(linha_campos);
        self.nome.render(self.status == StatusBanco::AltNome, area_nome, buf);
        self.id.render(self.status == StatusBanco::AltId, area_id, buf);

        render_lista_contas(
            &self.banco.contas,
            linha_contas,
            buf,
            &mut self.conta_state,
            self.status == StatusBanco::AltContas,
        );
    }
}

// ─── Tela 3: Editar Conta ─────────────────────────────────────────────────────

#[derive(PartialEq)]
enum StatusConta {
    AltNome,
    AltId,
    Sair(Option<Conta>),
}

pub struct EditarConta {
    status: StatusConta,
    conta: Conta,
    nome: Input,
    id: Input,
}

impl EditarConta {
    pub fn new() -> Self {
        Self {
            status: StatusConta::AltNome,
            conta: Conta { id: String::new(), nome: String::new() },
            nome: Input::new_texto("Nome da conta", String::new()),
            id: Input::new_texto("ID da conta", String::new()),
        }
    }

    pub fn set(conta: Conta) -> Self {
        let mut s = Self::new();
        s.nome = Input::new_texto("Nome da conta", conta.nome.clone());
        s.id = Input::new_texto("ID da conta", conta.id.clone());
        s.conta = conta;
        s
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Option<Conta>> {
        while !matches!(self.status, StatusConta::Sair(_)) {
            if let Err(e) = terminal.draw(|f| f.render_widget(&mut self, f.area())) {
                log::error!("Erro ao desenhar EditarConta: {}", e);
            }
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            }
        }
        if let StatusConta::Sair(conta) = self.status {
            return Ok(conta);
        }
        Ok(None)
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.status = StatusConta::Sair(None),
            KeyCode::Tab => self.proximo(),
            KeyCode::BackTab => self.anterior(),
            KeyCode::F(5) => self.salvar(),
            _ => match self.status {
                StatusConta::AltNome => self.nome.handle_key(key),
                StatusConta::AltId => self.id.handle_key(key),
                StatusConta::Sair(_) => {}
            },
        }
    }

    fn proximo(&mut self) {
        self.status = match self.status {
            StatusConta::AltNome => StatusConta::AltId,
            StatusConta::AltId => StatusConta::AltNome,
            StatusConta::Sair(_) => return,
        };
    }

    fn anterior(&mut self) {
        self.proximo();
    }

    fn salvar(&mut self) {
        let nome = self.nome.to_string();
        if nome.is_empty() {
            return;
        }
        let id_str = self.id.to_string();
        let id = if id_str.is_empty() { gerar_id(&nome) } else { id_str };
        self.status = StatusConta::Sair(Some(Conta { id, nome }));
    }
}

impl Widget for &mut EditarConta {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(1),
        ])
        .areas(area);

        let titulo_str = if self.conta.id.is_empty() { "Nova Conta" } else { "Editar Conta" };
        principal_titulo(titulo_str, titulo, buf);
        principal_comandos(vec!["TAB próximo", "F5 salvar", "ESC sair"], rodape, buf);

        let [area_nome, area_id] =
            Layout::horizontal([Constraint::Fill(3), Constraint::Fill(1)]).areas(corpo);
        self.nome.render(self.status == StatusConta::AltNome, area_nome, buf);
        self.id.render(self.status == StatusConta::AltId, area_id, buf);
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn render_lista_contas(
    contas: &[Conta],
    area: Rect,
    buf: &mut Buffer,
    state: &mut ListState,
    ativo: bool,
) {
    let borda = if ativo { LISTA_BORDA_ESTILO } else { LISTA_BORDA_ESTILO.dim() };
    let block = Block::new()
        .title(Line::raw("Contas").centered())
        .borders(Borders::TOP)
        .border_set(symbols::border::EMPTY)
        .border_style(borda)
        .bg(GERAL_BG);

    let items: Vec<ListItem> = contas
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let texto = format!(" {} [{}]", c.nome, c.id);
            ListItem::new(Line::styled(texto, GERAL_TEXT_FG)).bg(alternate_colors(i))
        })
        .collect();

    StatefulWidget::render(
        List::new(items)
            .block(block)
            .highlight_style(LISTA_SELECIONADO_ESTILO)
            .highlight_symbol("▶")
            .highlight_spacing(HighlightSpacing::Always),
        area,
        buf,
        state,
    );
}

fn gerar_id(nome: &str) -> String {
    use sha1::{Digest, Sha1};
    let mut h = Sha1::new();
    h.update(nome.trim().to_lowercase());
    hex::encode(h.finalize())
}
