# US — Dashboard principal integrado

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- `doc/product/migracao-desktop/epicos.md`
- Dashboard HTML atual da versão TUI

## Descrição
Como usuário de gestão financeira
Quero acessar um dashboard visual integrado na aplicação
Para visualizar minha situação financeira sem alternar entre aplicações

## Regras de negócio
- Dashboard deve ser a tela inicial da aplicação
- Informações devem atualizar em tempo real
- Gráficos devem ser interativos e responsivos
- Menu principal deve aparecer sob demanda no rodapé
- Dados devem carregar em menos de 2 segundos

## Critérios de aceitação
- Dado que abro a aplicação, quando carrega, então exibe dashboard como página inicial
- Dado que visualizo gastos, quando clico em categoria, então expande detalhes dos lançamentos
- Dado que altero dados, quando salvo, então dashboard atualiza automaticamente
- Dado que clico no rodapé, quando menu aparece, então posso navegar para outras funcionalidades
- Dado que redimensiono janela, quando altero tamanho, então layout se adapta responsivamente

## Não objetivos
- Configuração avançada de gráficos
- Exportação de relatórios
- Integração com bancos externos
