# Diário de Desenvolvimento

## 2026-03-20

### Análise inicial do projeto
Fiz uma revisão completa da arquitetura com o Kiro.

**Pontos fortes identificados:**
- Separação em camadas bem definida (dto / repository / widget / calc / componentes)
- `OptionalLazy<T>` e `Lazy<T>` são soluções elegantes para referências lazy no JSON
- IDs SHA1 determinísticos evitam duplicatas na importação OFX
- Detecção de encoding (chardet) resolve problema real de OFX bancários brasileiros
- Dashboard HTML/JS estático: simples e funcional

**Débitos técnicos identificados:**
- `unwrap()`/`expect()` no parser OFX — OFX malformado derruba o programa
- Sem backup antes de sobrescrever JSON — risco de corrupção
- Parser OFX manual e frágil (linha a linha, vulnerável a variações de formato)
- Lógica de persistência misturada nos DTOs (ex: `Lancamento::from_ofx()`)
- Zero testes automatizados
- Nomenclatura com sufixos não convencionais (`_repy`, `_wgt`, `_dto`)

**Decisão:** Projeto está ok para uso pessoal. Antes de publicar/migrar para desktop, priorizar: tratamento de erros no OFX, backup de JSON e testes em `calc/`.

### Organização da documentação
Criados `docs/context.md`, `docs/debitos-tecnicos.md` e `docs/diario.md`.

`debitos-tecnicos.md` organizado por módulo com três prioridades:
- 🔴 Imediato (risco de crash/perda de dados)
- 🟡 Relevante (qualidade/manutenibilidade)
- 🔵 Na migração (endereçar ao migrar para desktop/SQLite)

Itens 🔴 mais críticos: unwraps no OFX parser, sem escrita atômica no JSON, `.unwrap()` em `DadosDivida::primeira()/ultima()`, `.unwrap()` na data do contracheque.

Decisão registrada: migrar persistência para SQLite no futuro.

### Gráfico de gastos por categoria ano (concluído)
Implementado o gráfico de Stacked Bar horizontal (amCharts v4) para gastos por categoria nos últimos 12 meses.

**Mudanças realizadas:**
- `dash_gasto_por_categoria_ano_dto.rs`: `valores` mudou de `HashMap<String, ...>` para `Vec<DashGastoPorCategoriaAnoValores>` com campo `categoria: String` no mesmo nível dos meses — necessário para o amCharts consumir o array diretamente
- `calc_gasto_por_categoria_ano.rs`: corrigidos dois bugs:
  1. `gerar_agrupamento` não fazia `.trim()` nas partes do split, causando chaves com espaço que não batiam com a `ordem`
  2. `grupo_pai` acumulava o `nome_grupo` formatado em vez do `grupo` atual, quebrando a hierarquia a partir do 3º nível
- Criado `dashfiles/script/gasto-por-categoria-ano.js`: gera um gráfico por entrada do array, meses detectados dinamicamente das chaves
- `dashfiles/index.html`: adicionado divisor `lb-gastos-ano`, scripts de data e script do novo gráfico

**Versão estável gerada após estas correções. Próximo foco: débitos técnicos.**
