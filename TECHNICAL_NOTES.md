# Notas técnicas

## Princípios
- Port fiel ao HBM's Nuclear Tech; não criar um mod nuclear genérico.
- Não inferir a função de um asset apenas pelo nome do arquivo.
- Separar inventário de arquivos do catálogo de objetos do jogo.
- Preservar identificadores e nomes originais quando possível.
- Documentar adaptações exigidas pelo Horizon.

## Validações pendentes
- Confirmar versão/revisão do Horizon alvo.
- Confirmar formatos e esquemas atuais de `.hmod` e `.hnmod`.
- Confirmar build, carregamento e depuração.
- Confirmar APIs de itens, blocos, entidades, receitas, energia, eventos,
  renderização e áudio.
- Confirmar licença dos recursos do HBM.

## Responsabilidades
- `ntm-core`: lógica independente do jogo.
- `ntm-native`: integração com APIs nativas.
- `ntm-resourcepack`: recursos e metadados.
- `docs`: mapeamento, decisões e limitações.

## Decisão inicial
Criar scaffold Rust sem inventar APIs do Horizon.
