# US — Importação OFX com interface gráfica

Status: draft
Owner: product
Last updated: 2025-06-22

## Inputs
- `doc/product/migracao-desktop/epicos.md`
- `doc/product/migracao-desktop/arquitetura.md`
- Débitos técnicos relacionados ao parsing OFX

## Descrição
Como usuário que recebe extratos bancários
Quero importar arquivos OFX através de interface visual
Para carregar transações sem usar linha de comando

## Regras de negócio
- Drag-and-drop deve funcionar para seleção de arquivos
- Preview dos dados deve aparecer antes da confirmação (máximo 1000 transações visíveis)
- Duplicatas devem ser detectadas via chave composta: conta + valor + data + descrição
- Suporte a OFX 1.x (SGML) e 2.x (XML) com auto-detecção de encoding
- Erros de parsing devem ser tratados graciosamente
- Progresso da importação deve ser visível
- Comparação de duplicatas contra banco SQLite e lote atual

## Critérios de aceitação
- Dado que arrasto arquivo OFX, quando solto na área, então carrega preview dos primeiros 1000 lançamentos
- Dado que arquivo tem mais de 1000 transações, quando visualizo preview, então exibe contador "Mostrando 1000 de X transações"
- Dado que arquivo tem formato OFX 1.x ou 2.x, quando importo, então detecta formato automaticamente
- Dado que arquivo tem encoding ISO-8859-1 ou UTF-8, quando carrego, então detecta encoding automaticamente
- Dado que arquivo tem formato inválido, quando tento importar, então exibe erro específico sem crash
- Dado que existem duplicatas por chave composta, quando importo, então destaca e permite escolher ação
- Dado que importação está processando arquivo grande, quando aguardo, então vejo barra de progresso
- Dado que importação completa, quando finaliza, então mostra resumo dos lançamentos adicionados vs rejeitados

## Não objetivos
- Correção manual de arquivos OFX corrompidos
- Importação de outros formatos além de OFX
- Configuração de mapeamento de campos customizados
