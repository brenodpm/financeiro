# US — Importação OFX com interface gráfica

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- `doc/product/migracao-desktop/epicos.md`
- Débitos técnicos relacionados ao parsing OFX

## Descrição
Como usuário que recebe extratos bancários
Quero importar arquivos OFX através de interface visual
Para carregar transações sem usar linha de comando

## Regras de negócio
- Drag-and-drop deve funcionar para seleção de arquivos
- Preview dos dados deve aparecer antes da confirmação
- Duplicatas devem ser detectadas e sinalizadas
- Erros de parsing devem ser tratados graciosamente
- Progresso da importação deve ser visível

## Critérios de aceitação
- Dado que arrasto arquivo OFX, quando solto na área, então carrega preview dos lançamentos
- Dado que arquivo tem formato inválido, quando tento importar, então exibe erro específico sem crash
- Dado que existem duplicatas, quando importo, então destaca e permite escolher ação
- Dado que importação está processando, quando aguardo, então vejo barra de progresso
- Dado que importação completa, quando finaliza, então mostra resumo dos lançamentos adicionados

## Não objetivos
- Correção manual de arquivos OFX corrompidos
- Importação de outros formatos além de OFX
- Configuração de mapeamento de campos customizados
