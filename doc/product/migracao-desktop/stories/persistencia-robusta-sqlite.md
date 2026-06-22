# US — Persistência robusta com SQLite

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- `doc/product/migracao-desktop/epicos.md`
- Débitos técnicos relacionados à persistência JSON

## Descrição
Como usuário da aplicação TUI atual
Quero que meus dados sejam migrados automaticamente para SQLite
Para ter persistência confiável sem risco de corrupção

## Regras de negócio
- Migração deve ser automática na primeira execução da versão desktop
- Dados originais JSON devem ser preservados como backup
- Schema SQLite deve suportar todas as entidades atuais
- Transações devem garantir integridade referencial
- Sistema deve detectar e recuperar de corrupções

## Critérios de aceitação
- Dado que tenho dados na versão TUI, quando executo primeira vez desktop, então migra automaticamente todos os dados
- Dado que migração falha parcialmente, quando retento, então continua do ponto de parada
- Dado que salvo alteração, quando ocorre interrupção, então rollback mantém consistência
- Dado que consulto dados migrados, quando comparo com originais, então todos os registros estão presentes
- Dado que backup automático ativa, quando executa, então cria cópia SQLite em local configurável

## Não objetivos
- Suporte para outros bancos além de SQLite
- Interface de administração de banco
- Funcionalidades de auditoria avançada
