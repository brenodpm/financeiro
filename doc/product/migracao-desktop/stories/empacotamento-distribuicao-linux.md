# US — Empacotamento para distribuição Linux

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- `doc/product/migracao-desktop/epicos.md`

## Descrição
Como usuário Linux
Quero instalar a aplicação através de lojas de aplicativos
Para ter instalação e atualizações simplificadas

## Regras de negócio
- Deve gerar pacotes para Snap Store e Flathub minimamente
- Instalação deve ser isolada sem conflitos de dependências
- Aplicação deve funcionar offline após instalação
- Atualizações devem preservar dados do usuário
- Ícone e metadados devem seguir padrões das lojas

## Critérios de aceitação
- Dado que busco na Snap Store, quando procuro "financeiro", então encontro aplicação listada
- Dado que instalo via snap, quando executo, então abre sem dependências adicionais
- Dado que uso Flathub, quando instalo, então ícone aparece no menu de aplicações
- Dado que nova versão disponível, quando atualizo, então preserva todos os dados locais
- Dado que desinstalo, quando removo, então limpa arquivos mas oferece backup de dados

## Não objetivos
- Suporte para outros sistemas operacionais
- Distribuição via repositórios de distribuições específicas
- Auto-update integrado na aplicação
