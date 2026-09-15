# Third-Party Legal Notices

HelpOfAi's `codebase-memory` feature downloads and coordinates an external binary engine (`codebase-memory-mcp`) created by DeusData. The binary is distributed under the MIT license, and incorporates several third-party dependencies. If you use `helpofai codebase install`, you are downloading artifacts distributed by DeusData under these licenses.

## codebase-memory-mcp
**License:** MIT  
**Author:** DeusData  
**Source:** https://github.com/DeusData/codebase-memory-mcp  

## Nomic Embeddings (Bundled in the binary)
**Name:** `nomic-embed-code` (via ggml/llama.cpp)  
**License:** Apache License 2.0  
**Author:** Nomic AI  
**Notice:** This binary contains quantized weights and inference code for Nomic AI's text embeddings.

## C/C++ Ecosystem Dependencies
The upstream `codebase-memory-mcp` binary is statically linked and redistributes the following libraries under their respective permissive licenses (MIT, BSD, Zlib, Public Domain):
- **SQLite3** (Public Domain): Database engine
- **yyjson** (MIT): JSON parsing
- **xxHash** (BSD): Hashing
- **zstd** (BSD/GPL subset): Compression
- **mimalloc** (MIT): Memory allocator
- **simplecpp** (MIT): C/C++ preprocessing
- **Tre** (2-Clause BSD): Regex engine
- **Tree-sitter** (MIT): Code parsing grammar engine structure
- **160+ Tree-sitter grammars** (MIT/ISC): Various language authors

For the full detailed breakdown and exact texts of these licenses, please run `helpofai codebase get_third_party` or view `THIRD_PARTY_NOTICES.md` inside the DeusData release archive.
