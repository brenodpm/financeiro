# Contexto do Projeto

## Visão Geral
Organizador financeiro pessoal em Rust com TUI (ratatui). Importa arquivos OFX bancários, categoriza lançamentos por regras regex, gerencia dívidas, metas, contracheques e gera dashboard HTML/JS.

## Stack
- **UI**: ratatui (TUI terminal)
- **Persistência**: JSON flat files em `~/.financeiro/`
- **Serialização**: serde_json
- **Logging**: log4rs + log
- **Error handling**: color-eyre
- **Encoding**: chardet + encoding_rs (para OFX em ISO-8859)
- **Hashing**: sha1 + hex (IDs determinísticos)

## Estrutura de Módulos
```
src/
  app.rs           # Máquina de estados da aplicação (enum Etapa)
  main.rs          # Entrypoint: importa OFX, inicia TUI
  estilo.rs        # Estilos visuais ratatui
  config_log.rs    # Configuração de logging
  dto/             # Modelos de dados
  repository/      # Persistência (JSON files)
  widget/          # Telas TUI
  calc/            # Lógica de cálculo/agregação
  componentes/     # Widgets reutilizáveis (input, lista suspensa, checkbox)
dashfiles/         # Dashboard HTML/JS/CSS estático
docs/              # Documentação e diário de desenvolvimento
```

## Fluxo Principal
1. Ao iniciar: lê OFXs de `~/Downloads/importar/`, move para `~/Downloads/importado/`
2. Lançamentos novos vão para `nao-cat.json` (pendentes de categorização)
3. Usuário categoriza via TUI → regras salvas em `regras.json`
4. Lançamentos categorizados vão para `lancamentos.json`
5. Dashboard gerado como HTML com dados em JS

## Conceitos-chave
- **OptionalLazy<T>**: enum `None | Id(String) | Some(T)` — salva só o ID no JSON, carrega o objeto quando necessário
- **Lazy<T>**: igual mas sem None — para relações obrigatórias
- **Unico trait**: define `gerar_id()` — IDs são SHA1 determinísticos dos dados
- **Regra**: regex + fluxo (Entrada/Saída) + categoria — aplicada automaticamente na categorização

## Dashboard (`dashfiles/`)
- Gerado via TUI → opção "Gerar Gráfico" no menu
- Dados escritos como arquivos JS em `~/Financeiro/data/`
- HTML/CSS/JS copiados de `dashfiles/` (embutidos no binário via `include_dir`)
- Gráficos usam amCharts v4 (CDN)

### Gráficos disponíveis
| Arquivo JS | Tipo | Dados |
|---|---|---|
| `script/resumo.js` | Cards de resumo | `data/resumo.js` |
| `script/gasto-por-conta.js` | Pie chart | `data/gasto_por_conta.js` |
| `script/gasto-por-categoria.js` | Pie charts (um por grupo) | `data/gasto_por_categoria.js` |
| `script/gasto-por-categoria-ano.js` | Stacked Bar horizontal (um por grupo) | `data/gasto_por_categoria_ano.js` |
| `script/dividas.js` | Gráfico de dívidas | `data/dividas.js` |

### Estrutura do JSON `gasto_por_categoria_ano`
```json
[
  {
    "grupo": "Saídas >> Despesa",
    "valores": [
      { "categoria": "Fixa", "04/2025": 1060.0, "05/2025": 2292.0 },
      { "categoria": "Variavel", "04/2025": 8800.0 }
    ]
  }
]
```

### Hierarquia de categorias no cálculo anual
`TipoFluxo::to_string()` gera ex: `"Despesa; Fixa; Moradia; Energia elétrica"`

`calc_gasto_por_categoria_ano` percorre essa hierarquia com `grupo` (nível atual) e `grupo_pai` (nível anterior, sem acumular o caminho completo):
- i=0: grupo_pai=`""`, grupo=`"Saídas"` → chave `"Saídas"`, cat `"Despesa"`
- i=1: grupo_pai=`"Saídas >> "`, grupo=`"Despesa"` → chave `"Saídas >> Despesa"`, cat `"Fixa"`
- i=2: grupo_pai=`"Despesa >> "`, grupo=`"Fixa"` → chave `"Despesa >> Fixa"`, cat `"Moradia"`

Cada nível acumula a soma de todos os lançamentos filhos. A `ordem` gerada por `gerar_ordem_categorias` determina quais grupos viram gráficos (apenas grupos com mais de 1 categoria filho).
