# Requisitos Funcionais — Visualização e Categorização de Lançamentos

Status: approved
Owner: requirements
Last updated: 2026-06-23

## Inputs
- Story: Visualização e categorização de lançamentos
- Entrevista com usuário (2026-06-23)

## RF-001 — Visualização de Lançamentos Não Categorizados

**Descrição:** Sistema lista todos os lançamentos não categorizados com agrupamento inteligente.

**Critérios de Aceitação:**

1. Dado que existem lançamentos não categorizados na base
   Quando a tela de categorização é aberta
   Então exibe lista com lançamentos agrupados por descrição e ordenados por repetição (descendente) e data (mais recente)

2. Dado que um lançamento foi categorizado e persistido
   Quando a tela de categorização é aberta novamente
   Então o lançamento categorizado não aparece na lista

## RF-002 — Seleção e Criação de Regra de Categorização

**Descrição:** Usuário seleciona um lançamento (ou grupo) e abre interface de criação de regra.

**Critérios de Aceitação:**

1. Dado que visualizo lista de lançamentos não categorizados
   Quando clico em um lançamento ou agrupamento
   Então abre modal com campo de padrão pré-preenchido com descrição completa do lançamento

2. Dado que tenho modal de criação de regra aberto
   Quando edito o padrão (reduz para um trecho significativo)
   Então o sistema permite entrada livre de texto

3. Dado que tenho padrão e categoria selecionados
   Quando confirmo a regra
   Então regra é persistida e a tela principal exibe indicação de que a categoria será aplicada após confirmação final

## RF-003 — Revisão de Lançamentos Categorizados

**Descrição:** Após criar regra, sistema mostra tela de revisão com lançamentos que deram match.

**Critérios de Aceitação:**

1. Dado que criei uma regra de categorização
   Quando confirmo a regra
   Então sistema processa todos lançamentos não categorizados com a regra
   E exibe tela de revisão mostrando apenas lançamentos que deram match com a regra

2. Dado que visualizo tela de revisão
   Então cada lançamento é exibido individualmente com categoria que será aplicada

## RF-004 — Edição de Categoria Individual

**Descrição:** Usuário pode alterar categoria de lançamento individualizado na revisão.

**Critérios de Aceitação:**

1. Dado que visualizo lançamento na tela de revisão
   Quando clico em alterar categoria
   Então abre novamente modal de criação de regra com padrão pré-preenchido

2. Dado que altero categoria no modal
   Quando confirmo
   Então nova regra é persistida imediatamente e categoria é aplicada diretamente ao lançamento (sem necessidade de novo match)

## RF-005 — Persistência Final e Encerramento

**Descrição:** Usuário confirma categorizações finais e retorna à tela principal.

**Critérios de Aceitação:**

1. Dado que visualizo tela de revisão com lançamentos categorizados
   Quando clico em confirmar final
   Então todos os lançamentos são persistidos em `lancamentos.json` com suas respectivas categorias
   E regras criadas são armazenadas para reutilização

2. Dado que confirmei categorizações finais e não há mais lançamentos não categorizados
   Quando a persistência completa
   Então tela de categorização fecha automaticamente
   E retorna à tela anterior do aplicativo

3. Dado que confirmei categorizações finais mas ainda existem lançamentos não categorizados
   Quando a persistência completa
   Então retorna à tela principal de categorização com lançamentos pendentes

## RF-006 — Vinculação de Regra ao Lançamento

**Descrição:** Cada lançamento armazena referência à regra que o categorizou.

**Critérios de Aceitação:**

1. Dado que um lançamento foi categorizado por uma regra
   Quando visualizo dados persistidos do lançamento
   Então o lançamento contém referência ao padrão e categoria que o categorizou

## RF-007 — Visualização de Origem de Categorização

**Descrição:** Interface indica qual regra categorizou cada lançamento.

**Critérios de Aceitação:**

1. Dado que visualizo um lançamento categorizado na tela de revisão
   Quando procuro informações sobre categorização
   Então o sistema exibe qual padrão/categoria foi aplicada

## RF-008 — Ausência de Filtros Inicial

**Descrição:** Enquanto houver lançamentos não categorizados, nenhum filtro é ativo.

**Critérios de Aceitação:**

1. Dado que existem lançamentos não categorizados
   Quando acesso tela de categorização
   Então nenhuma opção de filtro é exibida

2. Dado que todos os lançamentos foram categorizados
   Quando tela se fecha e reabre
   Então futuras interações podem incluir filtros (fora do escopo desta story)
