# Mapeamento de Dependências — Sistema de Metas Financeiras

Status: approved
Owner: requirements
Last updated: 2026-06-22

## Dependências de Histórias (Story 9)

| Dependência | Board | Status | Motivo |
|---|---|---|---|
| Story 4 - Fundação da aplicação desktop | story/requisitos | **BLOQUEANTE** | Necessário: estrutura base da app, configurações, logging, suporte a múltiplos idiomas |
| Story 5 - Persistência robusta com SQLite | story/requisitos | **BLOQUEANTE** | Necessário: schema SQLite para armazenar metas, transações, backup automático |
| Story 6 - Dashboard principal integrado | story/requisitos | **BLOQUEANTE** | Necessário: UI da dashboard para exibir widget de metas, alertas, projeções |
| Story 8 - Visualização e categorização de lançamentos | story/requisitos | **BLOQUEANTE** | Necessário: categoria "Economia" e seus subcategorias existirem, lançamentos categorizados disponíveis |

## Regras de Negócio Dependentes

| RN | Dependência de Histórias | Detalhes |
|---|---|---|
| RN-001 a RN-008 | Story 4 (Fundação) | Internacionalização para nomes de metas e mensagens |
| RN-003, RN-005, RN-008 | Story 5 (SQLite) | Esquema com tabelas: `metas`, `meta_progresso_historico`, relacionamentos |
| RN-006 (Alertas) | Story 6 (Dashboard) | Widget de alertas, notificações integradas ao sistema |
| RN-003, RN-005 | Story 8 (Categorização) | Categoria pai "Entrada >> Economia" e subcategorias devem existir |

## Requisitos Não-Funcionais Dependentes

| Categoria | Dependência | Detalhes |
|---|---|---|
| Performance | Story 5 (SQLite) | Índices em `categoria_economia_id`, `data_conclusao`, `ativa` |
| Segurança | Story 5 (SQLite) | Prepared statements, constraints, foreign keys |
| Acessibilidade | Story 4 (Fundação) | Sistema de internacionalização, paleta de cores acessível |
| Observabilidade | Story 4 (Fundação) | Logger configurado (log4rs) |
| Compatibilidade | Story 4 (Fundação) | Path handlers, suporte a locale |

## Ordem de Execução Sugerida

1. **Story 4** — Fundação da aplicação desktop
2. **Story 5** — Persistência robusta com SQLite (depende de 4)
3. **Story 8** — Visualização e categorização de lançamentos (depende de 4, 5)
4. **Story 6** — Dashboard principal integrado (depende de 4, 5)
5. **Story 9** — Sistema de metas financeiras (depende de 4, 5, 6, 8) **← AQUI**

## Critérios de "Ready" para Story 9

✅ Story 4 deve estar em coluna `validacao-negocial` ou além (requisitos definidos)
✅ Story 5 deve estar em coluna `validacao-negocial` ou além (schema metas definido)
✅ Story 6 deve estar em coluna `validacao-negocial` ou além (layout dashboard definido)
✅ Story 8 deve estar em coluna `validacao-negocial` ou além (categorias definidas)
✅ Arquivo `docs/requirements/sistema-metas-financeiras/` completo (este documento)

## Artefatos que Story 9 Produz para Downstream

| Artefato | Consumidor | Uso |
|---|---|---|
| `docs/requirements/sistema-metas-financeiras/business-rules.md` | UX, Arquitetura, QA | Definição de regras, testes de aceitação |
| `docs/requirements/sistema-metas-financeiras/non-functional-requirements.md` | Arquitetura, QA | Performance, segurança, escalabilidade |
| `docs/requirements/sistema-metas-financeiras/acceptance-criteria.md` | QA, Dev | Testes de integração, validação de comportamento |
| Schema SQLite (vem em Story 5, referencia Story 9) | Dev | Implementação de persistência |
| Widget de metas no dashboard (referencia Story 6) | Dev | Integração visual |

## Riscos e Mitigações

| Risco | Probabilidade | Impacto | Mitigação |
|---|---|---|---|
| Categoria "Economia" não criada em Story 8 | MÉDIA | ALTO | RN-005 deve validar existência, criar padrão se não houver |
| Performance de cálculo com 1000+ metas | BAIXA | MÉDIO | Executar agregação em background, cache em memória |
| Inconsistência entre progresso em memória e SQLite | MÉDIA | MÉDIO | Sincronização a cada 5s, validação em startup |
| Prazo impossível (data passada) não detectado | ALTA | BAIXO | Validação em CA-010, logar warning, UI redireciona |
