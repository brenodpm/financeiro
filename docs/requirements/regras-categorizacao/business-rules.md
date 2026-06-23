# Regras de Negócio — Gestão de Regras de Categorização

Status: approved
Owner: requirements
Last updated: 2026-06-23

## Inputs
- Issue story: 11-gestao_de_regras_de_categorizacao
- Histórico de clarificações com usuário (2026-06-23)

## RN-001 — Separação de regras por fluxo

**Descrição:** Regras aplicam-se exclusivamente ao seu fluxo (entrada ou saída). Uma regra de categorização de saída nunca aplica a um lançamento de entrada, e vice-versa.

**Contexto:** Sistema processa transações em dois fluxos distintos; regras devem ser isoladas por fluxo para evitar categorização cruzada.

**Exceções:** Nenhuma.

## RN-002 — Ordenação de regras por especificidade

**Descrição:** Regras são testadas ordenadas pelo tamanho da string regex em ordem decrescente (maior para menor). Regex maiores são consideradas mais específicas; menores, mais generalistas.

**Contexto:** Garante que padrões mais específicos sejam avaliados primeiro, evitando categorização genérica desnecessária.

**Exceções:** Nenhuma.

## RN-003 — Escopo de análise: descrição da transação

**Descrição:** Regras operam exclusivamente sobre o campo de descrição da transação. Nenhum outro campo (valor, tipo, data, conta) é usado na avaliação de regras.

**Contexto:** Simplifica lógica de categorização e reduz combinatória de padrões.

**Exceções:** Nenhuma.

## RN-004 — Detecção e resolução de conflitos por regex duplicada

**Descrição:** Se duas ou mais regras compartilham a mesma string regex para o mesmo fluxo, um popup de resolução é apresentado ao usuário, solicitando que escolha qual regra manter. As regras não selecionadas são excluídas.

**Contexto:** Evita duplicação e ambiguidade na aplicação de regras.

**Exceções:** Nenhuma.

## RN-005 — Preview com as 10 transações mais recentes

**Descrição:** Ao revisar uma regra, o preview mostra o resultado de categorização nas 10 transações mais recentes do fluxo correspondente.

**Contexto:** Oferece feedback visual prático sem carregar grandes volumes de dados.

**Exceções:** Se houver menos de 10 transações no fluxo, mostra todas as disponíveis.

## RN-006 — Regex gerada exclusivamente na tela de categorização

**Descrição:** A conversão de critérios simples em regex ocorre apenas na tela de categorização de transações. A tela de listagem/edição de regras exibe regex já geradas e permite exclusão, mas não criação.

**Contexto:** Centraliza lógica de geração e garante consistência.

**Exceções:** Nenhuma.
