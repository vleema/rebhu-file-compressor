# My file compactor (rebhu)

### **Fase 1: Planejamento e Preparação do Ambiente**

**Período: 28/12/2024 a 30/12/2024**

- **Definir escopo**:
  - Compactador CLI em Rust.
  - Interface gráfica (GUI) em Java para interagir com o CLI.
  - Implementação dos algoritmos de compressão: Huffman, RLE e BWT.
- **Escolher ferramentas e bibliotecas**:
  - Crates: `serde`, `bit-vec`, `clap`.
- **Planejar estrutura do projeto**:
  - Separar módulos (Rust):
    - Gerenciamento de arquivos.
    - Algoritmos de compressão/descompressão.
    - Interface CLI.
  - Interface GUI (Java):
    - Seleção de arquivos.
    - Botões para compactar/descompactar.
    - Logs e taxa de compressão.
- **Configurar ambiente**:
  - Criar repositório no Git.
  - Configurar compilador Rust.
  - **Ambiente Java**:
    - Framework GUI (JavaFX)

---

### **Fase 2: Implementação do Compactador CLI**

**Período: 31/12/2024 a 06/01/2025**

1. **Implementação Inicial** (31/12 a 02/01):
   - **Compactação Huffman**:
     - Construir árvore de Huffman.
     - Codificar entrada usando Huffman.
   - **Descompactação**:
     - Reconstruir a tabela e descompactar o arquivo.
2. **Integração de BWT e RLE** (02/01 a 03/01):

   - **Transformação BWT**:
     - Implementar algoritmo BWT e sua inversa.
   - **Codificação RLE**:
     - Adicionar compressão baseada em repetição.
   - **Integrar Huffman + BWT + RLE**:
     - Definir a ordem ideal das operações para melhor compressão.

3. **Funcionalidades Adicionais e Validações** (04/01 a 05/01):
   - Gerar cabeçalhos para descompactação.
   - Implementar pseudo-caractere EOF para evitar problemas com padding.
   - Validar com arquivos de texto e binários.

---

### Fase 3: Desenvolvimento da Interface Gráfica (GUI)

1. **Prototipação da GUI** (07/01 a 08/01):
   - Layout básico:
     - Botões: "Selecionar Arquivos", "Compactar", "Descompactar".
     - Área para exibir mensagens e taxa de compressão.
     - Definir e implementar integração com o CLI via execução de comandos Rust.
2. **Implementação da GUI funcional** (09/01 a 10/01):
   - Adicionar validações de entrada e exibição de logs.
   - Testar integração com o compactador CLI em Rust.
3. **Refinamento e testes da GUI** (11/01 a 12/01):
   - Melhorar layout e usabilidade
   - Garantir que os erros sejam exibidos corretamente.

### **Fase 4: Testes e Documentação**

**Período: 13/01/2025 a 15/01/2025**

- **Testes Integrados** (09/01 a 10/01):
  - Validar compressão e descompressão com arquivos pequenos e grandes.
  - Verificar taxa de compressão e comportamento com diferentes formatos.
- **Documentação** (13/01 a 14/01):
  - Escrever README com instruções de uso.
  - Documentar código e funcionalidades.

---

### **Fase 5: Entrega Final**

**Período: 15/01/2025 a 16/01/2025**

- Revisar todos os componentes e corrigir bugs.
- Submeter o projeto ou disponibilizá-lo em repositório público.

---
