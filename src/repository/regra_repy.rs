use std::vec;

use crate::dto::{Categoria, FluxoRegra, Lazy, LazyFn, Regra, TipoFluxo};

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
    pub fn listar() -> Vec<Regra> {
        let cats = Categoria::listar();
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
        let mut atuais = Regra::listar();

        novas.into_iter().for_each(|n| {
            if !atuais.iter().any(|a| a.id == n.id) {
                atuais.push(n.clone());
            }
        });

        salvar(atuais);
    }

    pub fn nova(nova: Regra) {
        let mut atuais = Regra::listar();

        if !atuais.iter().any(|a| a.id == nova.id) {
            atuais.push(nova.clone());
        }

        salvar(atuais);
    }

    pub fn remover(&self) {
        let regras: Vec<Regra> = Regra::listar()
            .into_iter()
            .filter(|r| r.id != self.id)
            .collect();
        salvar(regras);
    }

    pub fn remover_sem_categoria() {
        let regras = Regra::listar();

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
