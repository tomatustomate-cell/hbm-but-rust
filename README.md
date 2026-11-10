# HBM's Nuclear Tech — port para Horizon

Projeto de port fiel do HBM's Nuclear Tech (NTM) para Horizon.

## Objetivo
Preservar, sempre que a API e os formatos do Horizon permitirem, materiais, minérios, itens, blocos, máquinas, estruturas, receitas, energia, reatores, armas, entidades, efeitos, texturas, modelos, sons e interface.

Create, Mekanism, Immersive Engineering, IndustrialCraft 2 e Applied Energistics 2 podem servir como referências técnicas, mas não substituem a identidade ou o comportamento do HBM.

## Estado inicial
Este pacote contém somente a estrutura de desenvolvimento. Nenhum conteúdo do HBM foi convertido ou declarado funcional. A integração depende da inspeção dos recursos originais e da confirmação das APIs e formatos atuais do Horizon.

## Estrutura
- `ntm-core/`: lógica Rust independente do jogo.
- `ntm-native/`: futura integração nativa com Horizon.
- `ntm-resourcepack/`: área reservada para recursos.
- `docs/`: inventário, decisões e plano.
- `tests/`: testes de regressão.

## Verificação local
Requisitos: Rust e Cargo.
```sh
cargo check --workspace
cargo test --workspace
```
Isso verifica apenas os crates Rust; não comprova integração nem funcionamento dentro do jogo.

## Regras
1. Verificar o comportamento original antes de implementar.
2. Não inventar receitas, funções ou dependências.
3. Preservar nomes e identidade visual quando possível.
4. Documentar adaptações exigidas pelo Horizon.
5. Não declarar recursos concluídos sem testes.
6. Não redistribuir recursos proprietários sem verificar direitos e licença.

Consulte `docs/PORTING_PLAN.md` e `docs/TECHNICAL_NOTES.md`.
