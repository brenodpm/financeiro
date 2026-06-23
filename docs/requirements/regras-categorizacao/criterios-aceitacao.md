# Critérios de Aceitação — Gestão de Regras de Categorização

## CA-001 — Criação de regra com critério simples

**Dado que** estou na tela de categorização e tenho permissão de criar regra
**Quando** seleciono um critério simples (e.g., "descrição contém 'X'")
**Então** o sistema gera regex válida correspondente e a salva associada ao fluxo da transação

**Notas técnicas:**
- Operadores suportados: contém, começa com, termina com, igual a
- Regex gerada deve ser compilável e validada antes de persistência
- Associação ao fluxo (entrada/saída) é derivada da transação em contexto

---

## CA-002 — Preview de categorização

**Dado que** criei ou estou editando uma regra
**Quando** visualizo a tela de preview
**Então** vejo as 10 transações mais recentes do fluxo com indicação visual de quais serão categorizadas por esta regra

**Notas técnicas:**
- Mostrar apenas transações não ainda categorizadas (ou marcadas como "a recategorizar")
- Ordenação por data decrescente (mais recentes primeiro)
- Destacar em cada transação se padrão combina e qual categoria seria atribuída

---

## CA-003 — Ordenação por especificidade

**Dado que** tenho múltiplas regras ativas para o mesmo fluxo
**Quando** o sistema aplica regras a uma transação
**Então** testa em ordem decrescente de tamanho da regex (maiores primeiro)

**Notas técnicas:**
- Cálculo de tamanho é feito na string da regex compilada
- Primeira regex a combinar ganha; não continua testando
- Ordem é dinâmica (pode mudar se regex forem editadas)

---

## CA-004 — Detecção de conflito: regex duplicada

**Dado que** tentei salvar uma regra com regex idêntica a outra para o mesmo fluxo
**Quando** click em "Salvar"
**Então** popup é apresentado listando as regras conflitantes, solicitando qual manter; as não selecionadas são deletadas

**Notas técnicas:**
- Comparação: string exata da regex (case-sensitive)
- Popup deve exibir: regex, categoria destino, data criação de cada regra
- Confirmação é obrigatória; cancelar aborta salvamento

---

## CA-005 — Listagem de regras

**Dado que** acesso tela de gerenciamento de regras
**Quando** carrego a página
**Então** vejo todas as regras ativas, filtradas por fluxo, com opções de editar ou excluir

**Notas técnicas:**
- Editar: abre preview e permite alterar categoria destino (regex não é editável, apenas vista)
- Excluir: requer confirmação
- Ordenação padrão: tamanho de regex (maior primeiro, respeitando ordenação de aplicação)

---

## CA-006 — Sugestão de regra automática

**Dado que** o sistema detecta padrão em transações
**Quando** analisa lançamentos recentes
**Então** sugere nova regra baseada em descritores repetidos

**Notas técnicas:**
- Análise: padrão de texto recorrente em campo "descrição"
- Sugestão é não-vinculante; usuário decide se aceita
- Exibir na tela de categorização como "sugestão" com opção de usar ou ignorar

---

## CA-007 — Isolamento por fluxo

**Dado que** tenho regras em ambos os fluxos (entrada e saída)
**Quando** aplico regra a transação de entrada
**Então** apenas regras do fluxo entrada são testadas

**Notas técnicas:**
- Validação em nível de lógica de aplicação: antes de testar regra, verificar que fluxo corresponde
- Testes devem cobrir cenário de transação entrada com regra saída e vice-versa
