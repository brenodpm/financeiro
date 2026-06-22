# US — Sistema de metas financeiras

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- `doc/product/migracao-desktop/epicos.md`
- Funcionalidade de metas da versão TUI

## Descrição
Como usuário planejador financeiro
Quero definir e acompanhar metas de economia
Para alcançar objetivos financeiros específicos

## Regras de negócio
- Metas devem ter prazo e valor objetivo
- Progresso deve ser calculado automaticamente baseado em lançamentos
- Projeções devem considerar histórico de economia
- Alertas devem avisar sobre desvios do planejado
- Interface deve mostrar múltiplas metas simultaneamente

## Critérios de aceitação
- Dado que crio meta, quando defino valor e prazo, então calcula valor mensal necessário
- Dado que tenho lançamentos categorizados como economia, quando visualizo meta, então atualiza progresso automaticamente
- Dado que estou atrasado na meta, quando analiso, então sugere ajustes no valor mensal
- Dado que visualizo dashboard de metas, quando vejo todas, então comparo prioridades facilmente
- Dado que meta está próxima do prazo, quando faltam 30 dias, então intensifica alertas

## Não objetivos
- Investimentos automatizados
- Conselhos financeiros personalizados
- Integração com bancos para transferências automáticas
