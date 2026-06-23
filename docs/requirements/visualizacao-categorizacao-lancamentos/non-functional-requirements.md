# Requisitos Não-Funcionais — Visualização e Categorização de Lançamentos

Status: approved
Owner: requirements
Last updated: 2026-06-23

## Inputs
- Story: Visualização e categorização de lançamentos
- Entrevista com usuário (2026-06-23)
- Context.md do projeto

## Performance

- Ordenação de lançamentos não categorizados (por repetição + data) deve completar em tempo real (< 500ms) para até 10.000 lançamentos
- Filtro por padrão (contains) deve executar em tempo real (< 500ms)
- Persistência de regras deve completar em < 1s (I/O)
- Re-processamento de lançamentos após nova regra deve completar em < 2s para 10.000 lançamentos

## Segurança

- Todas as operações de persistência devem incluir backup ou transação atômica (rollback em caso de falha)
- Regras de categorização não devem expor dados sensíveis em logs ou histórico
- Alterações de categoria devem ser auditáveis (quem alterou, quando)

## Escalabilidade

- Sistema deve suportar até 100.000 lançamentos não categorizados sem degradação perceptível de UX
- Armazenamento de regras deve ser eficiente (índice por padrão para busca O(1) ou O(log n))
- Estrutura deve permitir extensão para múltiplas contas bancárias sem redesenho

## Disponibilidade

- Tela de categorização deve estar disponível enquanto houver lançamentos não categorizados
- Falha de persistência deve notificar usuário e permitir retry sem perda de dados
- Recuperação de estado (volta à tela após interrupção) deve restaurar lançamentos e regras já processadas
