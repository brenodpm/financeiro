# Decisões Arquiteturais — Migração para aplicação Desktop

Status: approved
Owner: product
Last updated: 2026-06-23

## Inputs
- `doc/product/migracao-desktop/vision.md`
- `doc/product/migracao-desktop/epicos.md`
- Débito #13: Decisões de arquitetura bloqueando requisitos de importação OFX

## 1. Framework GUI

**Decisão:** Tauri com frontend HTML/CSS/JavaScript
**Justificativa:** 
- Mantém stack Rust no backend conforme vision existente
- Permite uso de tecnologias web familiares para UI
- Melhor performance que Electron com menor footprint de memória
- Suporte nativo para distribuição cross-platform
- API segura para comunicação frontend/backend via comandos Tauri

**Comunicação Frontend/Backend:**
- Backend Rust expõe comandos Tauri (`#[tauri::command]`)
- Frontend JavaScript invoca via `window.__TAURI__.invoke()`
- Comunicação assíncrona com serialização JSON automática
- Sem necessidade de API REST ou IPC complexo

## 2. Comparação de Duplicatas

**Decisão:** Chave primária composta no banco SQLite
**Justificativa:**
- Utiliza constraints de UNIQUE do SQLite para detecção automática
- Não depende de hash SHA1 que pode ter colisões
- Performance superior em consultas de duplicata
- Regras de negócio explícitas no schema do banco

**Critérios de Duplicata:**
- Mesma conta + mesmo valor + mesma data + mesmo memo/descrição
- Verificação tanto contra banco persistido quanto lote de importação atual
- Aplicável entre arquivos diferentes (duplicata global)

## 3. Suporte OFX

**Decisão:** Suporte completo OFX 1.x e 2.x com auto-detecção de encoding
**Especificação:**
- OFX 1.x (formato SGML legacy)  
- OFX 2.x (formato XML moderno)
- Auto-detecção de encoding: chardet-rs para detectar ISO-8859-1, UTF-8, Windows-1252
- Fallback gracioso para UTF-8 em caso de detecção inconclusiva

## 4. Limitação de Preview

**Decisão:** Preview virtualizado com limite de 1000 transações visíveis
**Comportamento:**
- Carrega arquivo completo na memória para validação
- Exibe apenas primeiras 1000 transações no preview
- Scroll virtual para arquivos maiores
- Contadores: "Mostrando 1000 de 5000 transações"
- Importação processa arquivo completo independente do preview

## Impactos nos Artefatos

Estas decisões desbloqueiam:
- Criação de requisitos técnicos específicos para story #10
- Definição de critérios de aceitação testáveis
- Especificação de contratos de API Tauri para importação OFX
- Documentação de limites de performance e memória
