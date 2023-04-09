# Typst.ts

Typst.ts allows you to independently run the Typst compiler and exporter (renderer) in your browser.

You can:

+ locally run the compilation via `typst-ts-cli` to get a precompiled document,
  + or use `typst-precompiler` to build your backend programmatically.
+ build your frontend using the lightweight TypeScript library `typst.ts`.
+ send the precompiled document to your readers' browsers and render it as HTML elements.

The Typst.ts application is designed to be fast due to the following reasons:
+ Precompiled documents are much smaller than their PDF equivalents.
  + For example, a compressed precompiled document is only 35KB while its corresponding PDF is 342KB.
+ The renderer has a small code size.
+ Typst itself has great performance.

### CLI

Run Compiler Example:

```shell
typst-ts-cli compile --workspace "fuzzers/corpora/math" --entry "fuzzers/corpora/math/math.typ"
```

Help:

```shell
$ typst-ts-cli --help
The cli for typst.ts.

Usage: typst-ts-cli <COMMAND>

Commands:
  compile  Run precompiler. [aliases: c]
  help     Print this message or the help of the given subcommand(s)
```

Compile Help:

```shell
$ typst-ts-cli compile --help
Run precompiler.

Usage: typst-ts-cli compile [OPTIONS] --entry <ENTRY>

Compile options:
  -w, --workspace <WORKSPACE>  Path to typst workspace [default: .]
  -e, --entry <ENTRY>          Entry file
      --format <FORMAT>        Output formats
  -o, --output <OUTPUT>        Output to directory, default in the same directory as the entry file [default: ]
```

### Precompiler

The precompiler is capable of producing PDF and artifact outputs from a Typst project.

### Renderer

The renderer accepts an input in artifact format and renders the document as HTML elements.
