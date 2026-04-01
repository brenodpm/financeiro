use crate::dto::Orientacao;

/// Calcula a prioridade de uma orientação com base no desvio em relação a uma referência.
/// Quanto maior o desvio, menor o número retornado (mais urgente).
/// Exemplos: desvio=350, referencia=400 → prioridade 13 (urgente)
///           desvio=75,  referencia=300 → prioridade 75 (informativo)
pub fn prioridade_por_desvio(desvio: f64, referencia: f64) -> u8 {
    if referencia == 0.0 {
        return 0;
    }
    let percentual = (desvio / referencia * 100.0).clamp(0.0, 100.0);
    (100.0 - percentual) as u8
}

pub fn ordenar(orientacoes: &mut Vec<Orientacao>) {
    orientacoes.sort_by_key(|o| o.prioridade);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::Orientacao;

    fn orientacao(prioridade: u8) -> Orientacao {
        Orientacao { prioridade, icone: "".to_string(), texto: "".to_string() }
    }

    #[test]
    fn ordenar_por_prioridade() {
        let mut o = vec![orientacao(50), orientacao(10), orientacao(30)];
        ordenar(&mut o);
        assert_eq!(o[0].prioridade, 10);
        assert_eq!(o[1].prioridade, 30);
        assert_eq!(o[2].prioridade, 50);
    }

    #[test]
    fn vetor_vazio_nao_falha() {
        let mut o: Vec<Orientacao> = vec![];
        ordenar(&mut o);
        assert_eq!(o.len(), 0);
    }

    #[test]
    fn desvio_alto_gera_prioridade_urgente() {
        assert!(prioridade_por_desvio(350.0, 400.0) <= 20);
    }

    #[test]
    fn desvio_baixo_gera_prioridade_informativa() {
        assert!(prioridade_por_desvio(75.0, 300.0) > 40);
    }

    #[test]
    fn desvio_zero_gera_prioridade_maxima_baixa() {
        assert_eq!(prioridade_por_desvio(0.0, 300.0), 100);
    }

    #[test]
    fn referencia_zero_nao_falha() {
        assert_eq!(prioridade_por_desvio(100.0, 0.0), 0);
    }
}
