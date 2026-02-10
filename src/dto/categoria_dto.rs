use std::{
    collections::HashMap,
    fmt::{self, Formatter, Result},
};

use serde::{Deserialize, Serialize};

use crate::dto::TipoDespesa;

use super::{
    gerar_sha1, lazy::LazyFn, optional_lazy::OptionalLazyFn, GrupoDespesa, Lazy, OptionalLazy,
    TipoFluxo, Unico,
};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Categoria {
    pub id: String,
    pub nome: String,
    pub tipo: TipoFluxo,
}

impl LazyFn<Categoria> for Lazy<Categoria> {
    fn id(&self) -> String {
        match self {
            Lazy::Id(id) => id.clone(),
            Lazy::Some(cat) => cat.id.clone(),
        }
    }

    fn some(&self) -> Categoria {
        match self {
            Lazy::Id(id) => Categoria {
                id: id.clone(),
                nome: String::new(),
                tipo: TipoFluxo::SemCategoria,
            },
            Lazy::Some(cat) => cat.clone(),
        }
    }
}

impl OptionalLazyFn<Categoria> for OptionalLazy<Categoria> {
    fn id(&self) -> String {
        match self {
            OptionalLazy::Id(id) => id.clone(),
            OptionalLazy::Some(cat) => cat.id.clone(),
            OptionalLazy::None => String::new(),
        }
    }

    fn some(&self) -> Option<Categoria> {
        match self {
            OptionalLazy::Id(id) => Some(Categoria {
                id: id.clone(),
                nome: String::new(),
                tipo: TipoFluxo::SemCategoria,
            }),
            OptionalLazy::Some(cat) => Some(cat.clone()),
            OptionalLazy::None => None,
        }
    }
}

impl Unico for Categoria {
    fn gerar_id(&mut self) {
        let mut itens: Vec<String> = Vec::new();

        itens.push(self.nome.clone());
        itens.push(self.tipo.to_string());

        self.id = gerar_sha1(itens.join(":"));
    }
}

impl fmt::Display for Categoria {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{}{}", self.tipo, self.nome))
    }
}

impl Categoria {
    pub fn lista_padrao() -> Vec<Categoria> {
        let mut resp: Vec<Categoria> = Vec::new();

        despesa(&mut resp, "Açougues", "Abastecimento", "Variavel");
        despesa(&mut resp, "Padarias", "Abastecimento", "Variavel");
        despesa(&mut resp, "Peixarias", "Abastecimento", "Variavel");
        despesa(&mut resp, "Supermercados", "Abastecimento", "Variavel");
        despesa(&mut resp, "Verdurarias", "Abastecimento", "Variavel");
        despesa(
            &mut resp,
            "Cosméticos e perfumarias",
            "Bem estar",
            "Variavel",
        );
        despesa(&mut resp, "Tratamentos estéticos", "Bem estar", "Variavel");
        despesa(&mut resp, "Vestuário", "Bem estar", "Variavel");
        despesa(&mut resp, "Materiais escolar", "Educação", "Variavel");
        despesa(&mut resp, "Bares", "Lazer", "Variavel");
        despesa(&mut resp, "Eventos", "Lazer", "Variavel");
        despesa(&mut resp, "Hospedagens", "Lazer", "Variavel");
        despesa(&mut resp, "Lanches", "Lazer", "Variavel");
        despesa(&mut resp, "Restaurantes", "Lazer", "Variavel");
        despesa(&mut resp, "Ferramentas", "Moradia", "Variavel");
        despesa(
            &mut resp,
            "Móveis e eletrodomésticos",
            "Moradia",
            "Variavel",
        );
        despesa(&mut resp, "Obras e manutenções", "Moradia", "Variavel");
        despesa(&mut resp, "Medicamentos", "Saúde", "Variavel");
        despesa(&mut resp, "Consultas", "Saúde", "Variavel");
        despesa(&mut resp, "Planos de saúde", "Saúde", "Variavel");
        despesa(&mut resp, "Cartão de crédito", "Transações", "Variavel");
        despesa(&mut resp, "Doações", "Transações", "Variavel");
        despesa(&mut resp, "Saques", "Transações", "Variavel");
        despesa(&mut resp, "Combustível", "Transporte", "Variavel");
        despesa(&mut resp, "Estacionamentos", "Transporte", "Variavel");
        despesa(&mut resp, "Manutenção veicular", "Transporte", "Variavel");
        despesa(&mut resp, "Pedágios e translados", "Transporte", "Variavel");
        despesa(&mut resp, "Passagens", "Transporte", "Variavel");
        despesa(&mut resp, "Taxas", "Tributos", "Variavel");

        despesa(&mut resp, "Mensalidades", "Educação", "Fixa");
        despesa(&mut resp, "Assinaturas", "Lazer", "Fixa");
        despesa(&mut resp, "Condomínio", "Moradia", "Fixa");
        despesa(&mut resp, "Energia elétrica", "Moradia", "Fixa");
        despesa(&mut resp, "Financiamentos imobiliários", "Moradia", "Fixa");
        despesa(&mut resp, "Internet", "Moradia", "Fixa");
        despesa(&mut resp, "Parcela imóvel", "Moradia", "Fixa");
        despesa(&mut resp, "Telefonia", "Moradia", "Fixa");
        despesa(&mut resp, "Esporte", "Saúde", "Fixa");
        despesa(&mut resp, "Seguro veicular", "Transporte", "Fixa");
        despesa(&mut resp, "INSS", "Tributos", "Fixa");
        despesa(&mut resp, "IPTU", "Tributos", "Fixa");
        despesa(&mut resp, "IPVA", "Tributos", "Fixa");
        despesa(&mut resp, "IR", "Tributos", "Fixa");
        despesa(&mut resp, "FGTS", "Tributos", "Fixa");
        despesa(&mut resp, "Licenciamento Veicular", "Tributos", "Fixa");

        despesa(&mut resp, "Taxas e Juros", "Transações", "Perda");
        despesa(&mut resp, "Empréstimos", "Transações", "Perda");
        despesa(&mut resp, "Multas", "Transações", "Perda");
        despesa(&mut resp, "IOF", "Tributos", "Perda");

        receita(&mut resp, "Salário", "Trabalho");
        receita(&mut resp, "Férias", "Trabalho");

        retorno(&mut resp, "Cashback");
        retorno(&mut resp, "Poupança");
        retorno(&mut resp, "Restituição");
        retorno(&mut resp, "Ações");
        retorno(&mut resp, "Grupo de investimento");

        investimento(&mut resp, "Poupança");
        investimento(&mut resp, "Ações");
        investimento(&mut resp, "Grupo de investimento");

        transferencias(&mut resp, "Transferência entre contas");
        transferencias(&mut resp, "Empréstimos familiar");

        resp.push(Categoria::new("", TipoFluxo::SemCategoria));
        resp
    }

    fn new(nome: &str, tipo: TipoFluxo) -> Self {
        Self {
            id: String::new(),
            nome: nome.to_string(),
            tipo: tipo,
        }
    }
}

fn despesa(array: &mut Vec<Categoria>, nome: &str, grupo: &str, sub_grupo: &str) {
    array.push(Categoria::new(
        nome,
        TipoFluxo::Despesa(GrupoDespesa::new(grupo, sub_grupo)),
    ));
}

fn receita(array: &mut Vec<Categoria>, nome: &str, grupo: &str) {
    array.push(Categoria::new(nome, TipoFluxo::Receita(grupo.to_string())));
}

fn investimento(array: &mut Vec<Categoria>, nome: &str) {
    array.push(Categoria::new(nome, TipoFluxo::Investimento));
}

fn retorno(array: &mut Vec<Categoria>, nome: &str) {
    array.push(Categoria::new(nome, TipoFluxo::Retorno));
}

fn transferencias(array: &mut Vec<Categoria>, nome: &str) {
    array.push(Categoria::new(nome, TipoFluxo::Transferencias));
}

#[derive(Clone)]
pub struct CategoriaMap(HashMap<String, CategoriaMap>);
impl CategoriaMap {
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    pub fn push(&mut self, array: Vec<&str>) {
        if !array[0].is_empty() {
            if !self.0.contains_key(array[0]) {
                self.0
                    .insert(array[0].to_string(), CategoriaMap(HashMap::new()));
            }
            if array.len() > 1 {
                if let Some(v) = self.0.get_mut(array[0]) {
                    v.push(array[1..].to_vec());
                }
            }
        }
    }
}

pub trait CategoriaMapa {
    fn agrupar(&self) -> CategoriaMap;
}

impl CategoriaMapa for Vec<Categoria> {
    fn agrupar(&self) -> CategoriaMap {
        let mut mapa = CategoriaMap(HashMap::new());

        for cat in self {
            let cat_str = cat.to_string();
            let array = cat_str.split(";").collect::<Vec<_>>();
            mapa.push(array);
        }

        mapa
    }
}
