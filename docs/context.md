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
