# Critérios de Aceitação Refinados — Sistema de Metas Financeiras

Status: approved
Owner: requirements
Last updated: 2026-06-22

## CA-001 — Criação de meta com cálculo automático

**Descrição:** Quando usuário cria meta com valor e prazo, sistema calcula valor mensal necessário automaticamente.

```gherkin
Dado que estou na tela de criar meta
E preenchido: nome="Viagem", valor_objetivo="3000", data_conclusao="2026-12-31", categoria="Entrada >> Economia >> Viagem"
E NÃO preenchido o campo "valor_mensal_planejado"
Quando clico em "Salvar"
Então meta é criada com sucesso
E sistema calcula: valor_mensal = 3000 / (meses_até_conclusão)
E valor_mensal calculado é exibido na tela de edição
```

**Regra aplicável:** RN-002 (Cálculo de valor mensal)

---

## CA-002 — Atualização de progresso em tempo real

**Descrição:** Quando novo lançamento é categorizado como economia (categoria vinculada a uma meta), progresso da meta atualiza automaticamente.

```gherkin
Dado que existe meta ativa "Fundo de Emergência" com valor_objetivo=5000, progresso=0
E novo lançamento é categorizado: valor=500, categoria="Entrada >> Economia >> Fundo de Emergência"
Quando lançamento é salvo
Então meta "Fundo de Emergência" atualiza progresso para 500
E atualização ocorre em menos de 100ms (sem delay perceptível na UI)
E log registra: "Meta #123 progresso atualizado: 0 → 500 (lançamento #456)"
```

**Regra aplicável:** RN-003 (Rastreamento de progresso), RN-005 (Categorias de economia)

---

## CA-003 — Projeção de conclusão e alerta amarelo

**Descrição:** Quando faltam 30 dias para o prazo de uma meta e progresso < 50% do objetivo, sistema exibe alerta amarelo.

```gherkin
Dado que hoje é "2026-11-22"
E existe meta "Viagem" com valor_objetivo=3000, progresso=1200, data_conclusao="2026-12-22"
Quando carrego o dashboard
Então vejo alerta AMARELO para "Viagem"
E alerta exibe: "⏱ Viagem: 40% alcançado (1200/3000). Faltam 30 dias. Aumente para R$60/dia."
E projeção exibe: "Projeção: R$2.200 se mantiver ritmo (faltarão R$800)"
```

**Regra aplicável:** RN-004 (Projeção de conclusão), RN-006 (Alertas por desvio)

---

## CA-004 — Alerta vermelho por atraso crítico

**Descrição:** Quando faltam 30 dias para prazo e progresso < 25% do objetivo, alerta muda para vermelho.

```gherkin
Dado que hoje é "2026-12-01"
E existe meta "Emergência" com valor_objetivo=2000, progresso=300, data_conclusao="2026-12-31"
Quando carrego o dashboard
Então vejo alerta VERMELHO para "Emergência"
E ícone exibe: "⚠ Emergência: 15% alcançado (300/2000). Atraso iminente."
E sugestão exibe: "Recomendação: aumentar valor mensal para R$170/dia ou estender prazo"
```

**Regra aplicável:** RN-006 (Alertas por desvio)

---

## CA-005 — Meta concluída antes do prazo

**Descrição:** Quando progresso >= valor_objetivo antes da data de conclusão, meta é marcada como concluída e alerta sinaliza sucesso.

```gherkin
Dado que hoje é "2026-11-15"
E existe meta "Fundo" com valor_objetivo=2000, progresso=1999, data_conclusao="2026-12-31"
E novo lançamento é categorizado: valor=50 para categoria da meta
Quando lançamento é salvo
Então progresso da meta atualiza para 2049
E sistema marca meta como "concluída"
E dashboard exibe: "✓ Fundo: 102% alcançado. Meta atingida em 46 dias!"
E notificação enviada (ex: log) celebrando conclusão
```

**Regra aplicável:** RN-003, RN-006

---

## CA-006 — Edição de meta com override manual

**Descrição:** Usuário pode alterar valor mensal planejado manualmente, sobrescrevendo cálculo automático.

```gherkin
Dado que existe meta "Viagem" com valor_mensal=100 (calculado automaticamente)
E clico em "Editar"
Quando altero valor_mensal para "150" manualmente
E clico em "Salvar"
Então meta é atualizada com valor_mensal=150
E próximas projeções usam 150 (não recalculam automaticamente)
```

**Regra aplicável:** RN-002

---

## CA-007 — Dashboard de múltiplas metas

**Descrição:** Dashboard exibe todas as metas ordenadas por data de conclusão (mais próximas primeiro), com indicadores visuais de progresso e alertas.

```gherkin
Dado que existem 3 metas ativas:
  | Nome          | Prazo       | Progresso | Objetivo |
  | Viagem        | 2026-12-22  | 1500      | 3000     |
  | Emergência    | 2026-12-31  | 300       | 2000     |
  | Casa          | 2027-06-30  | 5000      | 10000    |
Quando abro dashboard de metas
Então metas aparecem nesta ordem: Viagem, Emergência, Casa
E cada meta exibe: barra de progresso, percentual, alertas (se houver)
E usuário pode clicar para expandir detalhes (projeção, histórico)
```

**Regra aplicável:** RN-007 (Múltiplas metas simultâneas)

---

## CA-008 — Persistência e recuperação

**Descrição:** Todas as metas são persisted em SQLite. Ao reabrir aplicação, todas as metas e seus progressos estão intactos.

```gherkin
Dado que criei 3 metas e apliquei lançamentos
Quando fecho a aplicação e reabroa
Então todas as 3 metas reaparecem com progressos corretos
E não há perda de dados
E se banco foi corrompido, aplicação oferece: restaurar do backup OU reiniciar
```

**Regra aplicável:** RN-008 (Persistência de metas)

---

## CA-009 — Não repetir alertas no mesmo dia

**Descrição:** Se alerta foi disparado hoje, não dispara novamente até próximo dia.

```gherkin
Dado que alerta amarelo foi disparado hoje para "Viagem" às 09:00
E agora são 17:00 (mesmo dia)
Quando recarrego dashboard
Então alerta ainda aparece visualmente, mas notificação não dispara novamente
E log registra: "Alerta 'Viagem' suprimido: já disparado hoje"
```

**Regra aplicável:** RN-006

---

## CA-010 — Validação de entrada

**Descrição:** Sistema valida e rejeita entradas inválidas com mensagens claras.

```gherkin
Dado que estou criando meta
Quando preenchido com dados inválidos:
  | Campo                    | Valor           | Resultado esperado              |
  | valor_objetivo           | -100            | Erro: "Valor deve ser positivo" |
  | valor_objetivo           | 0               | Erro: "Valor deve ser > 0"      |
  | data_conclusao           | data passada    | Erro: "Data deve ser futura"    |
  | categoria                | não existe      | Erro: "Categoria não encontrada"|
  | valor_mensal_planejado   | > valor_objetivo| Aviso: "Acelerará conclusão"   |
Então validação bloqueia salvamento até correção
```

**Regra aplicável:** Segurança (validação de entrada)
