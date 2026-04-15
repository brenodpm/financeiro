use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, Widget, Wrap},
    DefaultTerminal,
};

use crate::{
    componentes::lista_suspensa::{ItemListaSuspensa, ListaSuspensa},
    dto::{CsvLayout, Lancamento},
    estilo::{
        alternate_colors, principal_comandos, principal_titulo, GERAL_BG, GERAL_TEXT_FG,
        LISTA_BORDA_ESTILO, LISTA_SELECIONADO_ESTILO,
    },
    repository::{csv_layout_repy, csv_repy},
};

use super::csv_layout_wgt::EditarCsvLayout;

pub struct ImportarCsv {
    arquivos: Vec<String>,
    idx_arquivo: usize,
    dropdown: ListaSuspensa,
    layouts: Vec<CsvLayout>,
    preview: Vec<Lancamento>,
    preview_state: ListState,
    preview_erro: Option<String>,
    status: Status,
}

enum Status {
    SelecionandoLayout,
    Concluido,
    Cancelado,
}

impl ImportarCsv {
    pub fn new(arquivos: Vec<String>) -> Self {
        let mut s = Self {
            arquivos,
            idx_arquivo: 0,
            dropdown: ListaSuspensa::new("Layout", vec![], false),
            layouts: vec![],
            preview: vec![],
            preview_state: ListState::default(),
            preview_erro: None,
            status: Status::SelecionandoLayout,
        };
        s.carregar_layouts();
        s
    }

    fn arquivo_atual(&self) -> &str {
        &self.arquivos[self.idx_arquivo]
    }

    fn nome_arquivo(&self) -> String {
        self.arquivo_atual()
            .split('/')
            .last()
            .unwrap_or("")
            .to_string()
    }

    fn carregar_layouts(&mut self) {
        self.layouts = csv_layout_repy::listar();
        let itens: Vec<ItemListaSuspensa> = self
            .layouts
            .iter()
            .map(|l| ItemListaSuspensa::new2(&l.id, &l.nome))
            .chain(std::iter::once(ItemListaSuspensa::new2(
                "__novo__",
                "+ Novo layout",
            )))
            .collect();
        self.dropdown.set_lista(itens);
        self.atualizar_preview(None);
    }

    fn atualizar_preview(&mut self, _terminal: Option<&mut DefaultTerminal>) {
        let id = self.dropdown.get_id_selecionado();
        if let Some(layout) = self.layouts.iter().find(|l| l.id == id) {
            let (lancamentos, erro) = csv_repy::preview(self.arquivo_atual(), layout, 10);
            self.preview = lancamentos;
            self.preview_erro = erro;
        } else {
            self.preview.clear();
            self.preview_erro = None;
        }
        self.preview_state.select_first();
    }

    fn importar_atual(&mut self) {
        let id = self.dropdown.get_id_selecionado();
        if let Some(layout) = self.layouts.iter().find(|l| l.id == id) {
            let lancamentos = csv_repy::importar(self.arquivo_atual(), layout);
            Lancamento::categorizar(&lancamentos);
            log::info!(
                "CSV {}: {} lançamentos importados",
                self.nome_arquivo(),
                lancamentos.len()
            );
        }
    }

    fn avancar_arquivo(&mut self) {
        self.idx_arquivo += 1;
        if self.idx_arquivo >= self.arquivos.len() {
            self.status = Status::Concluido;
        } else {
            self.atualizar_preview(None);
        }
    }

    pub fn importar(terminal: &mut DefaultTerminal) {
        let arquivos = csv_repy::listar_csvs();
        if arquivos.is_empty() {
            return;
        }
        ImportarCsv::new(arquivos)
            .run(terminal)
            .unwrap_or_else(|e| log::error!("Erro ao importar CSV: {e}"));
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        if self.arquivos.is_empty() {
            return Ok(());
        }

        loop {
            terminal.draw(|f| f.render_widget(&mut self, f.area()))?;

            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            }

            match self.status {
                Status::Concluido | Status::Cancelado => break,
                Status::SelecionandoLayout => {}
            }
        }
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Esc => self.status = Status::Cancelado,

            KeyCode::Enter => {
                let id = self.dropdown.get_id_selecionado();
                if id == "__novo__" {
                    self.abrir_novo_layout(terminal);
                } else if !id.is_empty() {
                    self.importar_atual();
                    self.avancar_arquivo();
                }
            }

            KeyCode::Char('n') | KeyCode::Char('N') => self.abrir_novo_layout(terminal),

            _ => {
                self.dropdown.handle_key(key, terminal);
                self.atualizar_preview(Some(terminal));
            }
        }
    }

    fn abrir_novo_layout(&mut self, terminal: &mut DefaultTerminal) {
        match EditarCsvLayout::new(self.arquivo_atual()).run(terminal) {
            Ok(Some(_)) => self.carregar_layouts(),
            Ok(None) => {}
            Err(e) => log::error!("Erro ao criar layout: {e}"),
        }
    }
}

impl Widget for &mut ImportarCsv {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let titulo_txt = format!(
            "Importar CSV  [{}/{}]  {}",
            self.idx_arquivo + 1,
            self.arquivos.len(),
            self.nome_arquivo()
        );
        principal_titulo(&titulo_txt, titulo, buf);
        principal_comandos(
            vec![
                "↓↑ (layout)",
                "ENTER (importar)",
                "N (novo layout)",
                "ESC (cancelar)",
            ],
            rodape,
            buf,
        );

        let [dropdown_area, preview_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(corpo);

        self.dropdown.render(true, dropdown_area, buf);
        self.render_preview(preview_area, buf);
    }
}

impl ImportarCsv {
    fn render_preview(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(Line::raw("Prévia (10 primeiros lançamentos)").centered())
            .borders(Borders::TOP)
            .border_style(LISTA_BORDA_ESTILO)
            .bg(GERAL_BG);

        if self.preview.is_empty() {
            let msg = self.preview_erro.as_deref()
                .unwrap_or("Selecione um layout para ver a prévia.");
            Paragraph::new(msg)
                .block(block)
                .fg(GERAL_TEXT_FG)
                .wrap(Wrap { trim: false })
                .render(area, buf);
            return;
        }

        let items: Vec<ListItem> = self
            .preview
            .iter()
            .enumerate()
            .map(|(i, l)| {
                ListItem::new(Line::styled(
                    format!(
                        "  {}  R$ {:>10.2}  {}",
                        l.data.format("%d/%m/%Y"),
                        l.valor,
                        l.descricao
                    ),
                    GERAL_TEXT_FG,
                ))
                .bg(alternate_colors(i))
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(LISTA_SELECIONADO_ESTILO);

        StatefulWidget::render(list, area, buf, &mut self.preview_state);
    }
}
