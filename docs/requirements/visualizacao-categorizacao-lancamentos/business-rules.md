# Regras de Negócio — Visualização e Categorização de Lançamentos

Status: approved
Owner: requirements
Last updated: 2026-06-23

## Inputs
- Story: Visualização e categorização de lançamentos
- Entrevista com usuário (2026-06-23)

## RN-001 — Escopo de Lançamentos em Visualização

**Descrição:** A tela de visualização e categorização mostra exclusivamente lançamentos não categorizados (aqueles em `nao-cat.json`).

**Contexto:** Lançamentos já categorizados serão gerenciados em épico separado.

**Exceções:** Nenhuma.

## RN-002 — Ordenação Padrão de Lançamentos

**Descrição:** Lançamentos não categorizados são ordenados por:
1. Quantidade de repetição da mesma descrição (decrescente)
2. Data de lançamento (mais recente primeiro)

**Contexto:** Agrupa lançamentos similares para eficiência na categorização em lote.

**Exceções:** Nenhuma.

## RN-003 — Matching de Padrão para Categorização

**Descrição:** Categorização automática usa busca por `contains` em descrição do lançamento, não regex literal. Uma categoria é aplicada quando o padrão (trecho) da descrição existe no lançamento.

**Contexto:** Simplifica a regra sem perder funcionalidade de agrupamento.

**Exceções:** Nenhuma.

## RN-004 — Persistência de Regras

**Descrição:** Quando o usuário confirma uma regra de categorização:
- O padrão (trecho) é armazenado persistentemente
- Todos os lançamentos não categorizados são processados através do categorizador
- Lançamentos que dão match com a regra entram na tela de revisão

**Contexto:** Permite reutilização de regras e auditoria.

**Exceções:** Nenhuma.

## RN-005 — Categorização Manual com Edição Posterior

**Descrição:** Um lançamento categorizado pode ter sua categoria alterada posteriormente:
- Ao alterar categoria de um lançamento individualizado, a nova regra é persistida imediatamente
- A categoria é aplicada diretamente ao lançamento sem depender de novo match
- A tela de revisão reflete a mudança

**Contexto:** Permite correções sem necessidade de revisar fluxo completo.

**Exceções:** Nenhuma.

## RN-006 — Persistência Final e Ciclo

**Descrição:** Após confirmação final:
- Todos os lançamentos categorizados são persistidos em `lancamentos.json` com categoria aplicada
- Lançamentos são removidos de `nao-cat.json`
- Usuário retorna à tela principal de categorização
- Se não houver mais lançamentos não categorizados, a tela de categorização fecha automaticamente

**Contexto:** Fecha ciclo de categorização e permite validação de conclusão.

**Exceções:** Nenhuma.

## RN-007 — Vinculação de Regra ao Lançamento

**Descrição:** Cada lançamento categorizado armazena referência à regra que o categorizou (padrão + categoria).

**Contexto:** Auditoria e rastreamento de origem da categorização.

**Exceções:** Nenhuma.

## RN-008 — Ausência de Filtros Durante Categorização

**Descrição:** Enquanto houver lançamentos não categorizados, nenhum filtro é disponibilizado na tela. Todos os lançamentos não categorizados aparecem na lista.

**Contexto:** Força o usuário a categorizar todos antes de outros usos da tela.

**Exceções:** Nenhuma.
