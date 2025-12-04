use color_eyre::Result;
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
    dto::{Categoria, Lancamento, LazyFn, OptionalLazyFn, Regra},
    estilo::{
        alternate_colors, principal_comandos, principal_titulo, GERAL_BG, LISTA_BORDA_ESTILO,
        LISTA_SELECIONADO_ESTILO,
    },
    widget::{alerta_wgt::Alerta, categoria_wgt::EditarCategoria},
};

pub struct ListaCategoria {
    sair: bool,
    categorias: Vec<Categoria>,
    state: ListState,
}

impl Default for ListaCategoria {
    fn default() -> Self {
        Self {
            sair: false,
            categorias: Categoria::listar(),
            state: Default::default(),
        }
    }
}

impl Widget for &mut ListaCategoria {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Lista de Categorias", titulo, buf);
        principal_comandos(
            vec![
                "↓↑ (mover)",
                "N (novo)",
                "ENTER (selecionar)",
                "ESC (sair)",
                "DEL (remover)",
            ],
            rodape,
            buf,
        );
        self.render_categorias(corpo, buf);
    }
}

impl ListaCategoria {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.state.select_first();
        while !self.sair {
            if let Err(erro) = terminal.draw(|frame| frame.render_widget(&mut self, frame.area())) {
                log::error!("Erro ao desenhar tela Lista de Categorias: {}", erro);
            };

            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            };
        }
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.sair = true,
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Char('n') | KeyCode::Char('N') => self.nova_categoria(terminal),
            KeyCode::Right | KeyCode::Enter => self.alterar_categoria(terminal),
            KeyCode::Delete => self.deletar(terminal),
            _ => {}
        }
    }

    fn deletar(&mut self, terminal: &mut DefaultTerminal) {
        if let Some(i) = self.state.selected() {
            let categoria = self.categorias[i].clone();
            let mut aviso: Vec<String> = Vec::new();

            aviso.push("Você tem certeza que gostaria de deletar a categoria:".to_string());
            aviso.push(String::new());
            aviso.push(categoria.to_string());
            aviso.push(String::new());

            let lancamentos: Vec<Lancamento> = Lancamento::lancamentos_listar()
                .into_iter()
                .filter(|l| l.categoria.id() == categoria.id)
                .collect();
            if lancamentos.len() == 1 {
                aviso.push("1 lançamento será reclassificado".to_string());
            } else if lancamentos.len() > 1 {
                aviso.push(format!(
                    "{} lançamentos serão reclassificados",
                    lancamentos.len()
                ));
            }

            let regras: Vec<Regra> = Regra::listar()
                .into_iter()
                .filter(|l| l.categoria.id() == categoria.id)
                .collect();
            if regras.len() == 1 {
                aviso.push("1 regras serão removidas".to_string());
            } else if regras.len() > 1 {
                aviso.push(format!("{} regras serão removidas", regras.len()));
            }

            if let Ok(resp) = Alerta::atencao(aviso).run(terminal) {
                if resp {
                    lancamentos.iter().for_each(|l| {
                        l.lancamentos_recategorizar();
                    });

                    categoria.deletar();
                    self.categorias = Categoria::listar();
                }
            }
        }
    }

    fn select_next(&mut self) {
        self.state.select_next();
    }

    fn select_previous(&mut self) {
        self.state.select_previous();
    }

    fn nova_categoria(&mut self, terminal: &mut DefaultTerminal) {
        match EditarCategoria::new().run(terminal) {
            Ok(op) => match op {
                Some(categoria) => {
                    categoria.salvar();
                    self.categorias = Categoria::listar();
                }
                None => {}
            },
            Err(erro) => log::error!("Erro ao criar nova categoria: {}", erro),
        }
    }

    fn alterar_categoria(&mut self, terminal: &mut DefaultTerminal) {
        if let Some(i) = self.state.selected() {
            let categoria = self.categorias[i].clone();
            match EditarCategoria::set(categoria).run(terminal) {
                Ok(op) => match op {
                    Some(categoria) => {
                        categoria.salvar();
                        self.categorias = Categoria::listar();
                    }
                    None => {}
                },
                Err(erro) => log::error!("Erro ao editar categoria: {}", erro),
            }
        }
    }

    fn render_categorias(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Categorias").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(LISTA_BORDA_ESTILO)
            .bg(GERAL_BG);

        let items: Vec<ListItem> = self
            .categorias
            .iter()
            .enumerate()
            .map(|(i, todo_item)| ListItem::from(todo_item).bg(alternate_colors(i)))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(LISTA_SELECIONADO_ESTILO)
            .highlight_symbol("▶ ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }
}
