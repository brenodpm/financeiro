use std::vec;

use crate::dto::{Categoria, FluxoRegra, Lancamento, Lazy, LazyFn, OptionalLazyFn, Regra, TipoFluxo};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const REGRAS: &str = "regras.json";

pub trait Buscar {
    fn buscar(&self, descricao: &String, fluxo: FluxoRegra) -> Option<Regra>;
}

impl Buscar for Vec<Regra> {
    fn buscar(&self, descricao: &String, fluxo: FluxoRegra) -> Option<Regra> {
        let mut resp: Option<Regra> = None;

        for i in 0..self.len() {
            if self[i].fluxo != fluxo {
                continue;
            }
            if descricao.contains(self[i].regex.to_lowercase().as_str()) {
                resp = Some(self[i].clone());
                break;
            }
        }

        resp
    }
}

impl Regra {
    pub fn listar_lazy() -> Vec<Regra> {
        let mut json: String = arq_ler(FIN, REGRAS).collect();
        if json.is_empty() {
            json = "[]".to_string();
        }
        let mut resp: Vec<Regra> = match serde_json::from_str(&json) {
            Ok(vec) => vec,
            Err(erro) => {
                log::error!("Erro ao ler regras: {}", erro);
                vec![]
            }
        };

        resp
    }
    pub fn listar_full() -> Vec<Regra> {
        let mut resp = Regra::listar_lazy();

        let cats = Categoria::listar();
        resp.iter_mut().for_each(|r| {
            r.categoria = Lazy::Some(
                cats.iter()
                    .find(|c| c.id == r.categoria.id())
                    .unwrap()
                    .clone(),
            )
        });

        resp
    }

    pub fn adicionar(novas: &mut Vec<Regra>) {
        let mut atuais = Regra::listar_full();

        novas.into_iter().for_each(|n| {
            if !atuais.iter().any(|a| a.id == n.id) {
                atuais.push(n.clone());
            }
        });

        salvar(atuais);
    }

    pub fn nova(nova: Regra) {
        let mut atuais = Regra::listar_full();

        if !atuais.iter().any(|a| a.id == nova.id) {
            atuais.push(nova.clone());
        }

        salvar(atuais);
    }

    pub fn remover(&self) {
        let regras: Vec<Regra> = Regra::listar_full()
            .into_iter()
            .filter(|r| r.id != self.id)
            .collect();
        salvar(regras);
    }

    pub fn remover_sem_categoria() {
        let regras = Regra::listar_full();

        regras.iter().for_each(|r| {
            if let Lazy::Some(cat) = r.categoria.clone() {
                match cat.tipo {
                    TipoFluxo::SemCategoria => r.remover(),
                    _ => {}
                }
            }
        });
    }

    pub fn salvar_lista(itens: &Vec<Regra>) {
        salvar(itens.clone());
    }

    pub fn expurgo() {
        let mut regras = Regra::listar_lazy();
        let lancamentos = Lancamento::lancamentos_listar();

        regras = remover_regras_duplicadas(regras);
        regras = remover_regras_sem_categoria(regras);
        regras = remover_regras_em_desuso(regras, lancamentos);

        Regra::salvar_lista(&regras);
    }
}

fn remover_regras_em_desuso(atual: Vec<Regra>, lancamentos: Vec<Lancamento>) -> Vec<Regra> {
    let mut resp: Vec<Regra> = Vec::new();
    atual.iter().for_each(|r| {
        lancamentos.iter().any(|l| l.regra.id() == r.id).then(|| {
            if !resp.iter().any(|m| m.id == r.id || r.regex == m.regex) {
                resp.push(r.clone());
            }
        });
    });

    if resp.len() != atual.len() {
        log::info!(
            "{} regra{} por desuso",
            atual.len() - resp.len(),
            if atual.len() - resp.len() == 1 {
                " foi removida"
            } else {
                "s foram removidas"
            }
        );
    }

    resp
}

fn remover_regras_duplicadas(atual: Vec<Regra>) -> Vec<Regra> {
    let mut resp: Vec<Regra> = Vec::new();

    atual.iter().for_each(|n| {
        if !resp
            .iter()
            .any(|a| a.id == n.id || (a.fluxo == n.fluxo && a.regex == n.regex))
        {
            resp.push(n.clone());
        }
    });

    if resp.len() != atual.len() {
        log::info!(
            "{} regra{} por duplicidade",
            atual.len() - resp.len(),
            if atual.len() - resp.len() == 1 {
                " foi removida"
            } else {
                "s foram removidas"
            }
        );
    }

    resp
}

fn remover_regras_sem_categoria(atual: Vec<Regra>) -> Vec<Regra> {
    let categorias = Categoria::listar();
    let mut resp: Vec<Regra> = Vec::new();
    atual.iter().for_each(|r| {
        categorias
            .iter()
            .any(|c| c.id == r.categoria.some().id)
            .then(|| {
                resp.push(r.clone());
            });
    });

    if resp.len() != atual.len() {
        log::info!(
            "{} regra{} por falta de categoria",
            atual.len() - resp.len(),
            if atual.len() - resp.len() == 1 {
                " foi removida"
            } else {
                "s foram removidas"
            }
        );
    }

    resp
}

fn salvar(mut regras: Vec<Regra>) {
    regras.sort_by(|a, b| b.regex.len().cmp(&a.regex.len()));

    for regra in regras.iter_mut() {
        if let Lazy::Some(t) = regra.categoria.clone() {
            regra.categoria = Lazy::Id(t.id);
        }
    }

    match serde_json::to_string_pretty(&regras) {
        Ok(json) => arq_escrever(FIN, REGRAS, json),
        Err(erro) => log::error!("Erro ao salvar regras: {}", erro),
    };
}
