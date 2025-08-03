use crate::dto::{Categoria, FluxoRegra, Lazy, LazyFn, Regra, TipoFluxo};

use super::file_repy::{arq_escrever, arq_ler};

const FIN: &str = ".financeiro";
const REGRAS: &str = "regras.json";

pub trait Buscar {
    fn buscar(&self, descricao: &String, fluxo: FluxoRegra) -> Option<Regra>;
}

impl Buscar for Vec<Regra> {
    fn buscar(&self, descricao: &String, fluxo: FluxoRegra) -> Option<Regra> {
        self.into_iter()
            .find(|item| item.fluxo == fluxo && descricao.contains(&item.regex))
            .map(|item| item.clone())
    }
}

impl Regra {
    pub fn listar() -> Vec<Regra> {
        let cats = Categoria::listar();
        let mut json: String = arq_ler(FIN, REGRAS).collect();
        if json.is_empty() {
            json = "[]".to_string();
        }
        let mut resp: Vec<Regra> = serde_json::from_str(&json).unwrap();

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

    pub fn adicionar(regras: &mut Vec<Regra>) {
        let mut atuais = Regra::listar();
        atuais.append(regras);

        salvar(atuais);
    }

    pub fn nova(regra: Regra) {
        let mut atuais = Regra::listar();
        atuais.push(regra);

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
}

fn salvar(mut regras: Vec<Regra>) {
    regras.sort_by(|a, b| b.regex.len().cmp(&a.regex.len()));

    for regra in regras.iter_mut() {
        if let Lazy::Some(t) = regra.categoria.clone() {
            regra.categoria = Lazy::Id(t.id);
        }
    }

    arq_escrever(FIN, REGRAS, serde_json::to_string(&regras).unwrap());
}
