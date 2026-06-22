# Vision — Migração para aplicação Desktop

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- Issue: `.pipe/boards/epic/analise-negocio/3-migracao_para_aplicacao_desktop.md`
- Contexto atual: `docs/context.md`
- Débitos técnicos: `docs/debitos-tecnicos.md`

## Problema
A aplicação financeira atual funciona exclusivamente no terminal (TUI), criando uma barreira de adoção significativa para usuários que preferem interfaces gráficas modernas. Esta limitação restringe o público-alvo e impede a expansão da base de usuários.

## Solução
Migrar completamente a aplicação para uma versão desktop moderna usando Rust com interface HTML/CSS/JavaScript, mantendo todas as funcionalidades atuais enquanto resolve os débitos técnicos existentes e adiciona capacidades de distribuição via lojas de aplicativos Linux.

## Público-alvo
- Usuários de sistemas Linux que necessitam organizar suas finanças pessoais
- Pessoas que importam dados bancários via arquivos OFX
- Usuários que preferem interfaces gráficas modernas a terminais
- Indivíduos que gerenciam dívidas, metas financeiras e contracheques

## Proposta de valor
- Interface gráfica intuitiva e moderna substituindo o terminal
- Eliminação de débitos técnicos que causam instabilidade
- Base de dados local robusta (SQLite) em vez de arquivos JSON
- Configurações personalizáveis via arquivos
- Dashboard integrado na própria aplicação
- Distribuição facilitada via lojas de aplicativos
- Internacionalização desde o início

## Métricas de sucesso
- Aplicação desktop completamente funcional com todas as features atuais
- Zero crashes relacionados aos débitos técnicos identificados
- Disponibilização em pelo menos 2 lojas de aplicativos Linux
- Suporte completo a i18n (português e inglês inicialmente)
- Migração automática de dados da versão TUI para desktop
