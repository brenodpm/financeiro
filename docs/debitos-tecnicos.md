# Débitos Técnicos

## Legenda
- 🔴 **Imediato** — risco de crash ou perda de dados em uso normal
- 🟡 **Relevante** — melhoria de qualidade/manutenibilidade, fazer quando possível
- 🔵 **Na migração** — endereçar ao migrar para desktop/SQLite

---

## Gerais

### Persistência
- 🔴 Sem backup antes de sobrescrever JSON — corrupção de dados se o processo morrer durante escrita: escrita atômica via `.tmp` + rename implementada; falta avaliar cenários com transações multi-arquivo
- ~~🔴 `arq_escrever` sobrescreve diretamente sem arquivo temporário + rename atômico~~ — resolvido
- 🔵 Migrar toda persistência para SQLite (`rusqlite`) — substituir `file_repy`, todos os `_repy` e o padrão `OptionalLazy<T>`/`Lazy<T>` por foreign keys reais

### Camada DTO
- 🟡 Lógica de persistência misturada nos DTOs (`Lancamento::from_ofx()`, `Divida::atualizar()`, `Banco::salvar()`) — viola separação de responsabilidades; deveria ficar nos `_repy`
- 🟡 Nomenclatura com sufixos não convencionais em Rust (`_repy`, `_wgt`, `_dto`) — dificulta navegação em IDEs e `cargo doc`
- 🟡 IDs gerados por SHA1 dos dados — alterar nome/tipo de categoria ou dívida gera novo ID, orphanando lançamentos que referenciam o ID antigo

### Testes
- 🟡 Zero testes automatizados — funções de `calc/` e lógica de `repository/` são facilmente testáveis unitariamente
- 🔵 Ao migrar, adicionar testes de integração para importação OFX e persistência

---

## Específicos

### Importação OFX (`ofx_repy`)
- 🔴 `valor.parse().unwrap()` — OFX com valor malformado derruba o programa
- 🔴 `NaiveDate::parse_from_str(...).unwrap()` — data malformada derruba o programa
- 🔴 `mover_para_importado` usa `.expect()` — falha silencia o erro e o arquivo pode ser reimportado
- 🔴 Indexação manual de string com `&linha[1..]` e `&linha[..pos-1]` — pode entrar em pânico com caracteres multibyte (OFX em UTF-8 com acentos)
- 🟡 Parser linha a linha é frágil — OFX tem variantes SGML e XML; considerar crate `ofx` ou parser próprio mais robusto
- 🟡 `FITID` (ID único do OFX) ignorado — o ID é gerado por SHA1 dos dados, o que pode colidir em lançamentos idênticos no mesmo dia

### Categorizador (`categorizador_wgt`)
- 🟡 `buscar_itens()` chama `Lancamento::checar_ja_importados()` que lê e reescreve o arquivo — efeito colateral escondido numa função de leitura
- 🟡 Regras aplicadas por `contains()` simples, não é regex de verdade apesar do campo se chamar `regex`

### Dívidas (`divida_wgt` / `lista_dividas_wgt` / `divida_dto`)
- 🔴 `DadosDivida::primeira()` e `ultima()` usam `.unwrap()` — pânico se a lista de parcelas estiver vazia
- 🟡 `Divida::atualizar()` chamada no `main` sem tratamento de erro visível

### Contracheque (`contracheque_wgt`)
- 🔴 `self.data_pagamento.to_naivedate().unwrap()` em `salvar()` — data inválida derruba o programa
- 🟡 `buscar_conta()` cria banco "Salario" hardcoded — deveria ser configurável
- 🟡 Lançamentos do contracheque entram como não-categorizados mesmo tendo nome conhecido — poderia aplicar regras automaticamente

### Metas (`meta_wgt`)
- 🟡 `meta_filtro` recarrega lista de bancos/categorias a cada render quando o tipo muda — deveria ser feito só uma vez na mudança
- 🟡 Typo: `"marior que"` em vez de `"maior que"` na lista de métricas

### Dashboard (`gerador_dash` / `dash_repy`)
- 🟡 `atualizar_base()` deleta e recria o diretório inteiro a cada geração — desnecessariamente destrutivo
- 🟡 Dashboard gerado como JS com `var nome = ...` (variável global) — frágil; preferir JSON puro com `fetch()`
- 🔵 Ao migrar para desktop, substituir dashboard HTML/JS por gráficos nativos

### Categorias (`categoria_wgt` / `categoria_dto`)
- 🟡 `lista_padrao()` tem ~60 categorias hardcoded com strings mágicas (`"Variavel"`, `"Fixa"`) — usar os enums `TipoDespesa` diretamente

### Configurações (`config_wgt`)
- 🟡 `salvar()` chamado a cada troca de campo (Tab/BackTab) — salva estado incompleto desnecessariamente
- 🟡 `endividamento_max` recalculado automaticamente como 40% do salário sem aviso ao usuário
