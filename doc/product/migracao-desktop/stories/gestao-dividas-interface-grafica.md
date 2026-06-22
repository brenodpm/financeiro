# US — Gestão de dívidas com interface gráfica

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- `doc/product/migracao-desktop/epicos.md`
- Funcionalidade de dívidas da versão TUI

## Descrição
Como usuário que controla dívidas
Quero gerenciar meus compromissos financeiros em interface visual
Para acompanhar pagamentos e planejamento de quitação

## Regras de negócio
- Deve calcular juros compostos automaticamente
- Simulações de pagamento devem ser interativas
- Alertas devem avisar sobre vencimentos próximos
- Histórico de pagamentos deve ser mantido
- Interface deve mostrar progresso de quitação

## Critérios de aceitação
- Dado que cadastro dívida, quando informo valor e condições, então calcula automaticamente evolução
- Dado que simulo pagamento, quando altero valor, então atualiza projeção em tempo real
- Dado que vencimento se aproxima, quando restam 3 dias, então exibe alerta visual
- Dado que registro pagamento, quando confirmo, então atualiza saldo e histórico
- Dado que visualizo dívida, quando vejo detalhes, então mostra gráfico de evolução

## Não objetivos
- Integração com bancos para débito automático
- Negociação automática de dívidas
- Suporte a moedas estrangeiras
