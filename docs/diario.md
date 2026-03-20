# Diário de Desenvolvimento

## 2026-03-20

### Análise inicial do projeto
Fiz uma revisão completa da arquitetura com o Kiro.

**Pontos fortes identificados:**
- Separação em camadas bem definida (dto / repository / widget / calc / componentes)
- `OptionalLazy<T>` e `Lazy<T>` são soluções elegantes para referências lazy no JSON
- IDs SHA1 determinísticos evitam duplicatas na importação OFX
- Detecção de encoding (chardet) resolve problema real de OFX bancários brasileiros
- Dashboard HTML/JS estático: simples e funcional

**Débitos técnicos identificados:**
- `unwrap()`/`expect()` no parser OFX — OFX malformado derruba o programa
- Sem backup antes de sobrescrever JSON — risco de corrupção
- Parser OFX manual e frágil (linha a linha, vulnerável a variações de formato)
- Lógica de persistência misturada nos DTOs (ex: `Lancamento::from_ofx()`)
- Zero testes automatizados
- Nomenclatura com sufixos não convencionais (`_repy`, `_wgt`, `_dto`)

**Decisão:** Projeto está ok para uso pessoal. Antes de publicar/migrar para desktop, priorizar: tratamento de erros no OFX, backup de JSON e testes em `calc/`.

### Organização da documentação
Criados `docs/context.md`, `docs/debitos-tecnicos.md` e `docs/diario.md`.

`debitos-tecnicos.md` organizado por módulo com três prioridades:
- 🔴 Imediato (risco de crash/perda de dados)
- 🟡 Relevante (qualidade/manutenibilidade)
- 🔵 Na migração (endereçar ao migrar para desktop/SQLite)

Itens 🔴 mais críticos: unwraps no OFX parser, sem escrita atômica no JSON, `.unwrap()` em `DadosDivida::primeira()/ultima()`, `.unwrap()` na data do contracheque.

Decisão registrada: migrar persistência para SQLite no futuro.
