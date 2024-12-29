```plain-text
compactador/
├── src/
│   ├── bin/
│   │   └── cli.rs          # CLI principal
│   ├── lib/
│   │   ├── huffman.rs      # Implementação do algoritmo de Huffman
│   │   ├── rle.rs          # Implementação do Run-Length Encoding (RLE)
│   │   ├── bwt.rs          # Implementação da Transformação Burrows-Wheeler
│   │   ├── compression.rs  # Coordenação das etapas de compactação
│   │   ├── decompression.rs # Coordenação das etapas de descompactação
│   │   └── utils.rs        # Funções auxiliares (e.g., manipulação de bits)
│   └── main.rs             # Entrada do programa
├── tests/
│   ├── huffman_tests.rs    # Testes para Huffman
│   ├── rle_tests.rs        # Testes para RLE
│   ├── bwt_tests.rs        # Testes para BWT
│   └── integration_tests.rs # Testes de integração
├── Cargo.toml              # Configuração do projeto Cargo
└── README.md               # Instruções e detalhes do projeto
```

## Funções e estruturas principais

- `compress`

  - Huffman ⇒ BWT + RLE
  - BWT + RLE ⇒ Huffman

- `decompress`

  - Huffman ⇒ BWT + RLE
  - BWT + RLE ⇒ Huffman

### Huffman

- `huff_compress`:

  - `build_tree`
  - `build_table(tree)`
  - Adicionar tabela como metadado no arquivo comprimido
  - Compactar o arquivo, substituindo cada caractere pelo código binário na tabela.
  - Pseudo-Caracter Fim de Arquivo (EOF)

- `huff_decompress`:

- `build_tree`:

  - Árvore de codificação Huffman.

- `build_table(tree)`:

  - Tabela de codificação Huffman.
