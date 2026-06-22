# US — Visualização e categorização de lançamentos

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- `doc/product/migracao-desktop/epicos.md`
- Funcionalidades de categorização da versão TUI

## Descrição
Como usuário que importa dados bancários
Quero visualizar e categorizar lançamentos em interface gráfica
Para organizar minhas transações de forma eficiente

## Regras de negócio
- Lista deve permitir ordenação por qualquer coluna
- Filtros devem funcionar em tempo real
- Categorização manual deve sobrescrever automática
- Mudanças devem ser salvas automaticamente
- Interface deve mostrar status de categorização

## Critérios de aceitação
- Dado que visualizo lançamentos, quando ordeno por coluna, então lista reorganiza imediatamente
- Dado que filtro por período, quando seleciono datas, então mostra apenas lançamentos do período
- Dado que categorizo manualmente, quando seleciono categoria, então salva automaticamente
- Dado que lançamento tem regra automática, quando visualizo, então indica origem da categorização
- Dado que busco por descrição, quando digito, então filtra em tempo real

## Não objetivos
- Edição de valores ou datas dos lançamentos
- Criação de lançamentos manuais
- Configuração de regras de categorização (será outra story)
