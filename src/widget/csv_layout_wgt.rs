use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Widget},
    DefaultTerminal,
};

use crate::{
    componentes::{
        check_wgt::Check,
        input_wgt::Input,
        lista_suspensa::{ItemListaSuspensa, ListaSuspensa},
    },
    dto::{Banco, CsvLayout},
    estilo::{alternate_colors, principal_comandos, principal_titulo, GERAL_TEXT_FG, LISTA_BORDA_ESTILO},
    repository::csv_layout_repy,
};

use ratatui::style::Stylize;

#[derive(PartialEq)]
enum Foco {
    Nome,
    Banco,
    Delimitador,
    ColConta,
    ColData,
    ColDescricao,
    ColValor,
    FormatoData,
    InverterSinal,
    Conta,
    Salvo,
    Cancelado,
}

pub struct EditarCsvLayout {
    layout: CsvLayout,
    foco: Foco,
    nome: Input,
    delimitador: Input,
    col_conta: Input,
    col_data: Input,
    col_descricao: Input,
    col_valor: Input,
    formato_data: Input,
    inverter_sinal: Check,
    dropdown_banco: ListaSuspensa,
    dropdown_conta: ListaSuspensa,
    bancos: Vec<Banco>,
    preview_raw: Vec<String>,
}

impl EditarCsvLayout {
    pub fn new(arquivo: &str) -> Self {
        Self::from(arquivo, CsvLayout::new(""))
    }

    pub fn from(arquivo: &str, l: CsvLayout) -> Self {
        let preview_raw = ler_preview(arquivo);
        let bancos = Banco::listar();
        let dropdown_banco = montar_dropdown_bancos(&bancos, &l.banco);
        let dropdown_conta = montar_dropdown_contas(&bancos, &l.banco, &l.conta);

        Self {
            foco: Foco::Nome,
            nome: Input::new_texto("Nome", l.nome.clone()),
            delimitador: Input::new_texto("Delimitador", l.delimitador.to_string()),
            col_conta: Input::new_texto("Coluna conta (nº)", l.col_conta.map(|c| (c + 1).to_string()).unwrap_or_default()),
            col_data: Input::new_inteiro("Coluna data", (l.col_data + 1) as i32),
            col_descricao: Input::new_inteiro("Coluna descrição", (l.col_descricao + 1) as i32),
            col_valor: Input::new_inteiro("Coluna valor", (l.col_valor + 1) as i32),
            formato_data: Input::new_texto("Formato data", l.formato_data.clone()),
            inverter_sinal: Check::new("Inverter sinal", l.inverter_sinal),
            dropdown_banco,
            dropdown_conta,
            bancos,
            preview_raw,
            layout: l,
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<Option<CsvLayout>> {
        loop {
            terminal.draw(|f| f.render_widget(&mut self, f.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key, terminal);
            }
            match self.foco {
                Foco::Salvo => return Ok(Some(self.layout)),
                Foco::Cancelado => return Ok(None),
                _ => {}
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent, terminal: &mut DefaultTerminal) {
        if key.kind != KeyEventKind::Press { return; }
        if key.code == KeyCode::Esc   { self.foco = Foco::Cancelado; return; }
        if key.code == KeyCode::F(10) { self.salvar(); return; }

        match key.code {
            KeyCode::Tab | KeyCode::Right    => { self.proximo(); return; }
            KeyCode::BackTab | KeyCode::Left  => { self.anterior(); return; }
            _ => {}
        }

        match self.foco {
            Foco::Nome            => self.nome.handle_key(key),
            Foco::Banco           => {
                self.dropdown_banco.handle_key(key, terminal);
                // atualiza contas ao mudar banco
                let banco_id = self.dropdown_banco.get_id_selecionado();
                self.dropdown_conta = montar_dropdown_contas(&self.bancos, &banco_id, "");
            }
            Foco::Delimitador     => self.delimitador.handle_key(key),
            Foco::ColConta        => self.col_conta.handle_key(key),
            Foco::ColData         => self.col_data.handle_key(key),
            Foco::ColDescricao    => self.col_descricao.handle_key(key),
            Foco::ColValor        => self.col_valor.handle_key(key),
            Foco::FormatoData     => self.formato_data.handle_key(key),
            Foco::InverterSinal   => self.inverter_sinal.handle_key(key),
            Foco::Conta           => self.dropdown_conta.handle_key(key, terminal),
            _ => {}
        }
    }

    fn col_conta_preenchida(&self) -> bool {
        !self.col_conta.to_string().is_empty()
    }

    fn proximo(&mut self) {
        self.foco = match self.foco {
            Foco::Nome            => Foco::Banco,
            Foco::Banco           => Foco::Delimitador,
            Foco::Delimitador     => Foco::ColConta,
            Foco::ColConta        => Foco::ColData,
            Foco::ColData         => Foco::ColDescricao,
            Foco::ColDescricao    => Foco::ColValor,
            Foco::ColValor        => Foco::FormatoData,
            Foco::FormatoData     => Foco::InverterSinal,
            Foco::InverterSinal   => {
                if self.col_conta_preenchida() { Foco::InverterSinal } else { Foco::Conta }
            }
            _ => Foco::Conta,
        };
    }

    fn anterior(&mut self) {
        self.foco = match self.foco {
            Foco::Banco           => Foco::Nome,
            Foco::Delimitador     => Foco::Banco,
            Foco::ColConta        => Foco::Delimitador,
            Foco::ColData         => Foco::ColConta,
            Foco::ColDescricao    => Foco::ColData,
            Foco::ColValor        => Foco::ColDescricao,
            Foco::FormatoData     => Foco::ColValor,
            Foco::InverterSinal   => Foco::FormatoData,
            Foco::Conta           => Foco::InverterSinal,
            _ => Foco::Nome,
        };
    }

    fn salvar(&mut self) {
        let nome = self.nome.to_string();
        log::debug!("salvar: nome={:?}", nome);
        if nome.is_empty() { return; }

        let banco_id = self.dropdown_banco.get_id_selecionado();
        log::debug!("salvar: banco_id={:?}", banco_id);
        if banco_id.is_empty() { return; }

        let col_conta_val = {
            let s = self.col_conta.to_string();
            if s.is_empty() { None } else { s.parse::<usize>().ok().map(|c| c.saturating_sub(1)) }
        };

        let conta_id = if col_conta_val.is_none() {
            let c = self.dropdown_conta.get_id_selecionado();
            log::debug!("salvar: conta_id={:?}", c);
            c
        } else {
            String::new()
        };

        let delim = self.delimitador.to_string().chars().next().unwrap_or(';');
        self.layout = CsvLayout {
            id: nome.to_lowercase().replace(' ', "_"),
            nome,
            delimitador: delim,
            col_data: (self.col_data.to_i32().max(1) - 1) as usize,
            col_descricao: (self.col_descricao.to_i32().max(1) - 1) as usize,
            col_valor: (self.col_valor.to_i32().max(1) - 1) as usize,
            formato_data: self.formato_data.to_string(),
            banco: banco_id,
            conta: conta_id,
            col_conta: col_conta_val,
            inverter_sinal: self.inverter_sinal.get_checked(),
        };
        csv_layout_repy::salvar(self.layout.clone());
        self.foco = Foco::Salvo;
    }
}

fn ler_preview(arquivo: &str) -> Vec<String> {
    use chardet::detect;
    use encoding_rs::Encoding;
    use std::{fs::File, io::Read};

    let mut file = match File::open(arquivo) {
        Ok(f) => f,
        Err(_) => return vec![],
    };
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let (enc_name, _, _) = detect(&buffer);
    let encoding = Encoding::for_label(enc_name.as_bytes()).unwrap_or(encoding_rs::UTF_8);
    let (cow, _, _) = encoding.decode(&buffer);
    cow.replace("\r\n", "\n")
        .replace('\r', "\n")
        .split('\n')
        .take(20)
        .enumerate()
        .map(|(i, linha)| format!("{:>2}: {}", i + 1, linha))
        .collect()
}

fn montar_dropdown_bancos(bancos: &[Banco], selecionado: &str) -> ListaSuspensa {
    let itens: Vec<ItemListaSuspensa> = bancos
        .iter()
        .map(|b| ItemListaSuspensa::new2(&b.id, &b.nome))
        .collect();
    let mut dd = ListaSuspensa::new("Banco", itens, false);
    if !selecionado.is_empty() {
        dd.set_id_selecionado(selecionado.to_string());
    }
    dd
}

fn montar_dropdown_contas(bancos: &[Banco], banco_id: &str, selecionado: &str) -> ListaSuspensa {
    let contas: Vec<ItemListaSuspensa> = bancos
        .iter()
        .find(|b| b.id == banco_id)
        .map(|b| b.contas.iter().map(|c| ItemListaSuspensa::new2(&c.id, &c.nome)).collect())
        .unwrap_or_default();
    let mut dd = ListaSuspensa::new("Conta", contas, false);
    if !selecionado.is_empty() {
        dd.set_id_selecionado(selecionado.to_string());
    }
    dd
}

impl Widget for &mut EditarCsvLayout {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [titulo, corpo, rodape] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        principal_titulo("Novo Layout CSV", titulo, buf);
        principal_comandos(vec!["↓↑/TAB (navegar)", "F10 (salvar)", "ESC (cancelar)"], rodape, buf);

        let col_conta_preenchida = self.col_conta_preenchida();

        let linhas_count = if col_conta_preenchida { 3 } else { 4 };
        let mut linhas_constraints = vec![
            Constraint::Length(3), // l1
            Constraint::Length(3), // l2
            Constraint::Length(3), // l3
        ];
        if !col_conta_preenchida {
            linhas_constraints.push(Constraint::Length(3)); // l4 conta
        }
        linhas_constraints.push(Constraint::Fill(1)); // preview

        let areas_v = Layout::vertical(linhas_constraints).split(corpo);

        let l1 = areas_v[0];
        let l2 = areas_v[1];
        let l3 = areas_v[2];
        let preview_area = areas_v[linhas_count];

        // linha 1: Nome | Banco | Delimitador
        let [a_nome, a_banco, a_delim] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Length(14),
        ])
        .areas(l1);

        // linha 2: Col conta | Col data | Col descrição | Col valor
        let [a_cconta, a_cdata, a_cdesc, a_cvalor] =
            Layout::horizontal([Constraint::Fill(1); 4]).areas(l2);

        // linha 3: Formato data | Inverter sinal
        let [a_fmt, a_inv] =
            Layout::horizontal([Constraint::Length(20), Constraint::Fill(1)]).areas(l3);

        self.nome.render(self.foco == Foco::Nome, a_nome, buf);
        self.dropdown_banco.render(self.foco == Foco::Banco, a_banco, buf);
        self.delimitador.render(self.foco == Foco::Delimitador, a_delim, buf);
        self.col_conta.render(self.foco == Foco::ColConta, a_cconta, buf);
        self.col_data.render(self.foco == Foco::ColData, a_cdata, buf);
        self.col_descricao.render(self.foco == Foco::ColDescricao, a_cdesc, buf);
        self.col_valor.render(self.foco == Foco::ColValor, a_cvalor, buf);
        self.formato_data.render(self.foco == Foco::FormatoData, a_fmt, buf);
        self.inverter_sinal.render(self.foco == Foco::InverterSinal, a_inv, buf);

        // linha 4 (condicional): Conta
        if !col_conta_preenchida {
            self.dropdown_conta.render(self.foco == Foco::Conta, areas_v[3], buf);
        }

        let items: Vec<ListItem> = self.preview_raw
            .iter()
            .enumerate()
            .map(|(i, l)| ListItem::new(Line::styled(l.clone(), GERAL_TEXT_FG)).bg(alternate_colors(i)))
            .collect();

        List::new(items)
            .block(
                Block::default()
                    .title(Line::raw("Prévia do arquivo (linhas brutas)").left_aligned())
                    .borders(Borders::TOP)
                    .border_style(LISTA_BORDA_ESTILO),
            )
            .render(preview_area, buf);
    }
}
