use std::fmt::{self, Formatter, Result};

use super::{gerar_sha1, DtoIdentificado, SubVec, TipoFluxo};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Categoria {
    pub id: String,
    pub nome: String,
    pub tipo: TipoFluxo,
}

impl DtoIdentificado for Categoria {
    fn gerar_id(&mut self) {
        let mut itens: Vec<String> = Vec::new();

        itens.push(self.nome.clone());
        itens.push(self.tipo.to_string());

        self.id = gerar_sha1(itens.join(":"));
    }
}

impl From<String> for Categoria {
    #[inline]
    fn from(value: String) -> Categoria {
        let values: Vec<String> = value.split(';').map(String::from).collect();
        Categoria::from(values)
    }
}

impl From<Vec<String>> for Categoria {
    #[inline]
    fn from(value: Vec<String>) -> Categoria {
        Categoria {
            id: value[0].clone(),
            nome: value[1].clone(),
            tipo: TipoFluxo::from(value.sub_vec().sub_vec()),
        }
    }
}

impl fmt::Display for Categoria {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_fmt(format_args!("{} {}", self.tipo, self.nome))
    }
}

impl Categoria {
    #[inline]
    pub fn to_line(&self) -> String {
        let mut resp: Vec<String> = Vec::new();

        resp.push(self.id.clone());
        resp.push(self.nome.clone());
        resp.push(self.tipo.to_line());

        resp.join(";")
    }
    
    pub fn lista_padrao() -> Vec<Categoria> {
        let mut resp: Vec<Categoria> = Vec::new();

        despesa(&mut resp, "Açougues", "Abastecimento", "Variavel");
        despesa(&mut resp, "Padarias", "Abastecimento", "Variavel");
        despesa(&mut resp, "Peixarias", "Abastecimento", "Variavel");
        despesa(&mut resp, "Supermercados", "Abastecimento", "Variavel");
        despesa(&mut resp, "Verdurarias", "Abastecimento", "Variavel");
        despesa(&mut resp, "Cosméticos e perfumarias", "Bem estar", "Variavel");
        despesa(&mut resp, "Tratamentos estéticos", "Bem estar", "Variavel");
        despesa(&mut resp, "Vestuário", "Bem estar", "Variavel");
        despesa(&mut resp, "Materiais escolar", "Educação", "Variavel");
        despesa(&mut resp, "Bares", "Lazer", "Variavel");
        despesa(&mut resp, "Eventos", "Lazer", "Variavel");
        despesa(&mut resp, "Hospedagens", "Lazer", "Variavel");
        despesa(&mut resp, "Lanches", "Lazer", "Variavel");
        despesa(&mut resp, "Restaurantes", "Lazer", "Variavel");
        despesa(&mut resp, "Ferramentas", "Moradia", "Variavel");
        despesa(&mut resp, "Móveis e eletrodomésticos", "Moradia", "Variavel");
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

        resp
    }
}

fn despesa(array: &mut Vec<Categoria>, nome: &str, grupo: &str, despesa: &str) {
    array.push(Categoria::from(
        format!(";{nome};Despesa;{grupo};{despesa}").to_string(),
    ));
}

fn receita(array: &mut Vec<Categoria>, nome: &str, grupo: &str) {
    array.push(Categoria::from(
        format!(";{nome};Receita;{grupo}").to_string(),
    ));
}

fn investimento(array: &mut Vec<Categoria>, nome: &str) {
    array.push(Categoria::from(
        format!(";{nome};Investimento").to_string(),
    ));
}

fn retorno(array: &mut Vec<Categoria>, nome: &str) {
    array.push(Categoria::from(
        format!(";{nome};Retorno").to_string(),
    ));
}

fn transferencias(array: &mut Vec<Categoria>, nome: &str) {
    array.push(Categoria::from(
        format!(";{nome};Transferencias").to_string(),
    ));
}