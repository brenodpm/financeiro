# Regras de Negócio — Sistema de Metas Financeiras

Status: approved
Owner: requirements
Last updated: 2026-06-22

## Inputs
- `.pipe/boards/story/requisitos/9-sistema_de_metas_financeiras.md`
- `repo/docs/context.md` (arquitetura do projeto)

## RN-001 — Composição de meta

**Descrição:** Uma meta é composta por: nome, categoria de economia, valor objetivo total, data de conclusão e (opcional) valor mensal planejado.

**Contexto:** Define a estrutura mínima para qualquer meta criada. A meta fixa o destino da poupança.

**Exceções:** Nenhuma — todos os campos obrigatórios devem ser preenchidos antes de salvar.

---

## RN-002 — Cálculo de valor mensal

**Descrição:** Quando valor mensal não é informado, calcular: `valor_mensal = valor_objetivo / meses_restantes`. Se `meses_restantes <= 0`, usar 1 mês (meta com prazo hoje).

**Contexto:** Permite planejamento automático quando usuário não detalha o ritmo mensal.

**Exceções:** Se usuário informar valor mensal manualmente, não recalcular automaticamente (permitir override).

---

## RN-003 — Rastreamento de progresso

**Descrição:** Progresso da meta = soma de lançamentos categorizados como economia dentro da categoria da meta, desde criação até hoje. Progresso não volta para trás, mesmo se lançamentos forem deletados (progresso acumulado).

**Contexto:** Metas precisam de status em tempo real para avisos e projeções.

**Exceções:** Lançamentos marcados como "revertidos" ou "anulados" não contam no progresso (serão identificados por flag de status).

---

## RN-004 — Projeção de conclusão

**Descrição:** Projeção = `progresso_atual + (valor_mensal_planejado * meses_até_data_conclusao)`. Se projeção >= valor_objetivo, meta será concluída no prazo. Caso contrário, calcular quantos meses adicionais serão necessários.

**Contexto:** Ajuda usuário a visualizar se ritmo atual é suficiente.

**Exceções:** Se meses_até_conclusão < 0 (data já passou), meta está atrasada e projeção assume prazos impossíveis (flag de alerta).

---

## RN-005 — Categorias de economia

**Descrição:** Uma meta deve estar vinculada a exatamente uma subcategoria de "Entrada >> Economia" (ex: "Entrada >> Economia >> Fundo de Emergência").

**Contexto:** Permite cálculo automático de progresso: agrupa todos os lançamentos dessa categoria.

**Exceções:** Categoria deve pré-existir em `regras.json` e estar classificada como economia.

---

## RN-006 — Alertas por desvio

**Descrição:** Alerta dispara quando:
- Progresso < (valor_objetivo * 0.5) com 30 dias até prazo (alerta amarelo)
- Progresso < (valor_objetivo * 0.25) com 30 dias até prazo (alerta vermelho)
- Progresso = valor_objetivo antes da data (meta atingida)

**Contexto:** Intensifica avisos conforme prazo se aproxima, prioriza comportamento de economia.

**Exceções:** Alertas não se repetem no mesmo dia (uma única notificação por dia por meta).

---

## RN-007 — Múltiplas metas simultâneas

**Descrição:** Usuário pode criar quantas metas quiser sem limite. Dashboard mostra todas ordenadas por: data de conclusão (mais próximas primeiro), depois por prioridade manual (se houver).

**Contexto:** Planos financeiros complexos requerem múltiplos objetivos paralelos.

**Exceções:** Nenhuma — apenas performance com 1000+ metas monitoradas (não esperado em uso típico).

---

## RN-008 — Persistência de metas

**Descrição:** Todas as metas são salvas em SQLite com campos: `id`, `nome`, `categoria_economia_id`, `valor_objetivo`, `data_conclusao`, `valor_mensal_planejado` (opcional), `progresso_acumulado`, `data_criacao`, `data_ultima_atualizacao`. Relatório de metas exportável em JSON/CSV para análise externa.

**Contexto:** Garante durabilidade entre sessões e permite auditoria.

**Exceções:** Soft-delete apenas (flag `ativa = false`) — histórico preservado para rastreabilidade.
