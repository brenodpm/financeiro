# Problem Space — Migração para aplicação Desktop

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- Documentação atual do projeto: `docs/context.md`
- Débitos técnicos mapeados: `docs/debitos-tecnicos.md`
- Análise da base de código atual

## Contexto
A aplicação financeira existe como uma ferramenta TUI (Terminal User Interface) desenvolvida em Rust, que importa dados OFX bancários, categoriza lançamentos por regras regex, gerencia dívidas e metas, e gera dashboards HTML/JS estáticos. A aplicação utiliza arquivos JSON para persistência e possui uma arquitetura funcional mas com limitações significativas.

## Problemas

### Barreira de Adoção
- Interface de terminal é intimidante para usuários não técnicos
- Curva de aprendizado alta para navegação TUI
- Impossibilidade de distribuição via lojas de aplicativos tradicionais

### Instabilidade Técnica
- 8 débitos técnicos críticos (🔴) que causam crashes em uso normal
- Múltiplos `.unwrap()` e `.expect()` que podem derrubar a aplicação
- Falta de tratamento de erros em importação OFX e manipulação de dados

### Limitações de Persistência
- Arquivos JSON podem corromper durante interrupções
- Ausência de transações e integridade referencial
- Sistema `OptionalLazy<T>` complexo para simular foreign keys

### Experiência do Usuário Fragmentada
- Dashboard gerado como arquivos estáticos separados da aplicação
- Necessidade de alternar entre TUI e navegador web
- Configurações dispersas em múltiplos arquivos JSON

## Impacto
- Baixa adoção por usuários não técnicos
- Risco de perda de dados devido a crashes frequentes
- Manutenibilidade comprometida por débitos técnicos
- Impossibilidade de crescimento da base de usuários
- Experiência de usuário inconsistente e fragmentada

## Oportunidade
O mercado de aplicações desktop Linux está crescendo, especialmente para ferramentas de produtividade pessoal. A migração permitirá:
- Alcançar um público muito maior através de interfaces gráficas
- Resolver definitivamente os problemas de estabilidade
- Criar uma experiência unificada e profissional
- Estabelecer base sólida para funcionalidades futuras
- Posicionar a aplicação em lojas de software tradicionais
