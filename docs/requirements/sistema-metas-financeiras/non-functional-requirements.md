# Requisitos Não-Funcionais — Sistema de Metas Financeiras

Status: approved
Owner: requirements
Last updated: 2026-06-22

## Inputs
- `.pipe/boards/story/requisitos/9-sistema_de_metas_financeiras.md`
- `repo/docs/context.md` (stack Rust + ratatui/tauri)

## Performance

- **Carregamento de metas**: Listar todas as metas (até 1000) deve completar em < 500ms (SQLite em disco local).
- **Cálculo de progresso**: Agregação de lançamentos por categoria deve rodar em background, sem bloquear UI.
- **Atualização de progresso**: Quando novo lançamento é categorizado, meta correspondente atualiza em < 100ms (cache em memória).
- **Dashboard de metas**: Renderização com 20+ metas simultâneas deve ser fluida (> 30 FPS em máquinas com 4GB RAM).

## Segurança

- **Isolamento de dados**: Cada usuário (configurado em `config.toml`) vê apenas suas metas (chave de segregação: `user_id` ou diretório local).
- **Validação de entrada**: Valores monetários rejeitar valores negativos, nulos ou maiores que `i64::MAX`.
- **Injeção SQL**: Usar prepared statements (serde_rusqlite ou similar) para todas as queries.
- **Integridade referencial**: Foreign key `categoria_economia_id` obrigatória — não permitir órfãs.

## Escalabilidade

- **Metas por usuário**: Suportar até 1000 metas ativas sem degradação perceptível.
- **Histórico de metas**: Permitir arquivo de metas concluídas sem impactar queries de metas ativas (particionamento lógico).
- **Exportação de dados**: CSV/JSON com até 10.000 registros (metas + progresso histórico) deve executar em < 2s.

## Disponibilidade

- **Recuperação de falhas**: Se banco SQLite corrupto, aplicação oferece opção: restaurar do último backup ou reinicializar.
- **Backup automático**: Backup diário do arquivo SQLite em diretório configurável (`~/.financeiro/backups/`).
- **Resiliência em modo offline**: Se persistência falhar, aplicação armazena em memória e tenta salvar a cada intervalo (ex: a cada 30s).

## Observabilidade

- **Logging**: Operações CRUD de metas registradas em nível `INFO` (criação, atualização, conclusão). Cálculos de progresso em nível `DEBUG`.
- **Erro de cálculo**: Se projeção não convergir (cenário impossível), logar warning e avisar usuário.
- **Auditoria**: Cada atualização de progresso registra timestamp e origem (lançamento ID, usuário) para rastreabilidade.

## Acessibilidade

- **Internacionalização**: Todos os rótulos, mensagens e unidades monetárias devem suportar português (pt-BR) e inglês (en-US) conforme configuração.
- **Contraste visual**: Alertas devem usar combinação de cor + ícone (não só cor) — vermelho + ⚠, amarelo + ⏱.
- **Navegação por teclado**: Todas as operações (criar, editar, deletar meta) acessíveis sem mouse.

## Compatibilidade

- **Sistemas operacionais**: Linux (foco inicial), com preparação para macOS e Windows (path handlers, separadores de arquivo).
- **Versão do SQLite**: Compatível com SQLite 3.30+ (disponível em Ubuntu 20.04+).
- **Encoding**: Aceitar entrada monetária em locale local (ex: "1.234,56" em pt-BR, "1,234.56" em en-US).
