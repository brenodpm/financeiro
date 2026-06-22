# US — Gestão de regras de categorização

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- `doc/product/migracao-desktop/epicos.md`
- Sistema de regras regex da versão TUI

## Descrição
Como usuário que categoriza transações
Quero configurar regras automáticas através de interface visual
Para automatizar categorização sem conhecer regex

## Regras de negócio
- Interface deve permitir criar regras sem conhecimento técnico
- Preview deve mostrar quais lançamentos serão afetados
- Regras devem ter prioridade configurável
- Sistema deve sugerir regras baseado em padrões
- Conflitos entre regras devem ser sinalizados

## Critérios de aceitação
- Dado que crio regra, quando defino critério simples, então gera regex automaticamente
- Dado que configuro regra, quando vejo preview, então mostra lançamentos que serão categorizados
- Dado que tenho múltiplas regras, quando reordeno prioridade, então aplica na sequência correta
- Dado que sistema detecta padrão, quando analisa lançamentos, então sugere nova regra
- Dado que regras conflitam, quando salvo, então alerta sobre sobreposição

## Não objetivos
- Editor de regex avançado
- Machine learning para categorização
- Sincronização de regras entre dispositivos
