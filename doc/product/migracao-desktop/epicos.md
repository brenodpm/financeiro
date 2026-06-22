# Épicos — Migração para aplicação Desktop

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- Vision: `doc/product/migracao-desktop/vision.md`
- Problem Space: `doc/product/migracao-desktop/problem-space.md`
- Análise de funcionalidades existentes

## Épico: Fundação da Aplicação Desktop

**Objetivo:** Estabelecer a base tecnológica e arquitetural para a aplicação desktop
**Escopo:** 
- Setup inicial de projeto desktop em Rust com framework web-view
- Estrutura de diretórios e organização de código
- Sistema de configuração via arquivos
- Logging e tratamento de erros robusto
- Setup de internacionalização (i18n)

**Fora de escopo:** 
- Funcionalidades de negócio específicas
- Migração de dados existentes
- Interface de usuário final

## Épico: Persistência e Migração de Dados

**Objetivo:** Implementar persistência robusta e migrar dados existentes
**Escopo:**
- Implementação de camada de dados com SQLite
- Schemas de banco de dados para todas as entidades
- Sistema de migração automática de dados JSON para SQLite
- Backup e recovery de dados
- Resolução de débitos técnicos relacionados à persistência

**Fora de escopo:**
- Interface de usuário
- Funcionalidades de importação OFX (mantém versão atual temporariamente)

## Épico: Interface de Usuário Core

**Objetivo:** Criar interface gráfica moderna para funcionalidades essenciais
**Escopo:**
- Dashboard principal integrado
- Navegação e menu principal
- Telas para visualização de lançamentos
- Interface para categorização manual
- Sistema de notificações e alertas

**Fora de escopo:**
- Funcionalidades avançadas (dívidas, metas, relatórios)
- Configurações avançadas
- Importação automatizada

## Épico: Gestão Financeira Avançada

**Objetivo:** Implementar funcionalidades completas de gestão financeira
**Escopo:**
- Gestão completa de dívidas com interface gráfica
- Sistema de metas financeiras
- Contracheques e renda variável
- Relatórios e análises avançadas
- Filtros e buscas sofisticadas

**Fora de escopo:**
- Funcionalidades experimentais
- Integrações externas

## Épico: Importação e Automação

**Objetivo:** Recriar sistema de importação com interface moderna
**Escopo:**
- Interface gráfica para importação OFX
- Sistema robusto de parsing OFX (resolver débitos críticos)
- Gestão de regras de categorização
- Importação de CSV personalizada
- Automação de tarefas recorrentes

**Fora de escopo:**
- Integração direta com bancos (APIs)
- Importação de outras fontes de dados

## Épico: Distribuição e Finalização

**Objetivo:** Preparar aplicação para distribuição e uso final
**Escopo:**
- Empacotamento para distribuição Linux
- Documentação completa de usuário
- Setup de distribuição via lojas de aplicativos
- Testes de aceitação final
- Migração definitiva de usuários da versão TUI

**Fora de escopo:**
- Suporte para outros sistemas operacionais
- Funcionalidades não planejadas nos épicos anteriores
