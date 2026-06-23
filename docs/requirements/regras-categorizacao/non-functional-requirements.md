# Requisitos Não-Funcionais — Gestão de Regras de Categorização

Status: approved
Owner: requirements
Last updated: 2026-06-23

## Inputs
- Issue story: 11-gestao_de_regras_de_categorizacao
- Histórico de clarificações com usuário (2026-06-23)

## Performance

- **Preview em tempo real**: Processamento de até 10 transações com aplicação de regras deve completar em <500ms
- **Listagem de regras**: Exibição de todas as regras ativas para um fluxo deve carregar em <200ms
- **Geração de regex**: Conversão de critério simples deve completar em <100ms
- **Detecção de conflitos**: Verificação de duplicatas ao salvar regra deve completar em <300ms

## Segurança

- **Validação de regex**: Toda regex gerada deve ser validada antes de persistência para evitar padrões inválidos ou ReDOS (ReDoS attacks)
- **Isolamento de fluxos**: Regras de um fluxo nunca devem ser aplicadas a outro fluxo; isso deve ser garantido em nível de persistência e lógica

## Escalabilidade

- **Limite de regras por fluxo**: Sistema deve suportar até 1.000 regras ativas por fluxo sem degradação de performance
- **Processamento em lote**: Aplicação de regras a múltiplas transações (e.g., re-categorização) deve executar em batches para evitar bloqueio

## Disponibilidade

- **Falha em geração de regex**: Se regex falhar validação, transação não é categorizada automaticamente; usuário recebe erro claro e opção de retry ou correção manual
- **Consistência de conflitos**: Resolução de conflitos deve garantir que apenas uma regra permaneça; se houver falha, nenhuma regra é deletada até confirmação explícita
