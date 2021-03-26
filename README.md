# TinkerLang

TinkerLang is a tool that allows you to quickly prototype programming languages, using state of the art tools. Generate an easily embeddable, fast incremental parser with [tree-sitter][tree-sitter] and compile into [LLVM IR][llvm] to get great performance while spending as little time as possible on it.

**NOTICE:** TinkerLang is *only* compatible for Linux currently. It is compatible with [Docker][docker].

## Using the Docker Image

At this time, the docker image isn't polished, but it is usable. Simply pull the docker image from the [docker packages list][docker-packages]. If you get an error pulling, you may need to [authenticate with GitHub][pull-github-cr].

Then, run the docker image. Mount `a.out` to get the output binary, and you should be able to run that in an docker container that bases off of `ubuntu:focal` (or smaller others). Pass in the lowerer, input, and tree-sitter binary to their correct places.

```
docker run --rm \
    -v ~/.tree-sitter/:/root/.tree-sitter/ \
    -v $PWD/input:/input \
    -v $PWD/lowerer:/lowerer \
    -v $PWD/a.out:/a.out \
        docker.pkg.github.com/sirjosh3917/tinkerlang/latest:main \
    --input input --parser javascript --lowerer lowerer
```

## Getting Started

First, download the latest TinkerLang binary from the [Nightly build on Github Actions][nightly-build].

To use TinkerLang, first you'll need to [generate a parser with tree-sitter][generate-a-parser-with-tree-sitter]. We may provide our own resources on generating parsers soon, but for now please refer to [tree-sitter's documentation][generate-a-parser-with-tree-sitter].

Once you've generated the parser, use `--parser <name>` to tell TinkerLang to use your parser. The compiled parser *must* be located at `~/.tree-sitter/bin/<name>.so` at this time.

Then, write a lowerer for your parser. The lowerer is executed with QuickJS, and is given access to `tree` to represent that AST, and `context` to create pseudo LLVM IR. See [Writing a Lowerer][#writing-a-lowerer] for more information. Use `--lowerer <path to js file>` to pass the lowerer to TinkerLang.

Once both steps are completed, you can pass TinkerLang an input file to be parsed with tree sitter via `--input <path to input file>`. The resultant command should look similar to the following

```
$ ./tinkerlang --input code.js --parser javascript --lowerer example-lowerer.js
```

## Writing a Lowerer

The lowerer is the part of a compiler which takes an AST, and outputs some kind of flat intermediate assembly. In TinkerLang's case, that'd be taking the input from `tree-sittter`, and converting it into pseudo LLVM IR, which is then turned into actual LLVM IR and finally into a binary.

The lowerer has access to a few variables, notably:

- `console.log`: for printing to the console to assist in debugging
- `tree`: for accessing the AST
- `toValue`: for getting the string value of a node
- `context`: for generating pseudo LLVM IR
- `bool`, `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`: pseudo LLVM IR types

**Example Lowerer**

A picture speaks a thousand words, and so does code.

```js
const add = context.method("add", u64, [u64, u64])
    .block("entry")
        .ld_param(0, 0)
        .ld_param(1, 1)
        .add(2, 0, 1)
        .ret(2);

const main = context.method("main", i32, [])
    .block("entry")
        .ld_const(0, u64, 1)
        .ld_const(1, u64, 2)
        // NOTE: you can pass methods OR blocks to functions that take methods
        .call(2, add, [0, 1])
        .truncate(3, i32, 2)
        .ret(3);

context.setMain(main);
```

### `console.log`

Documentation is available [on the web][console.log]. There are some differences with how it's implemented, but generally try to pass in strings.

### `tree`

`tree` is the root node of the generated tree-sitter tree. Its type definition is as follows:

```ts
interface Node {
    type: string;
    value: NodeValue;
    children: Node[]
}
```

Do note that the actual value of `value` is *implementation defined*. Do not rely on it in any way - instead, pass it to `toValue()` if you want values from it.

### `toValue`

`toValue` takes in a `NodeValue`, and outputs a string containing the contents of the AST node. Its type definition is as follows:

```ts
declare function toValue(value: NodeValue): string;
```

### `context`

`context` is an API that provides helper methods for building pseudo LLVM IR. It closely interacts with the compiler, and provides abstractions for building the pseudo LLVM IR. As this project is currently heavily WIP, the best reference you'll find on its types is to [read the code yourself][primer.js].

[tree-sitter]: https://tree-sitter.github.io/tree-sitter/
[llvm]: https://llvm.org/
[docker]: https://www.docker.com/
<<<<<<< Updated upstream
[pull-github-cr]: https://docs.github.com/en/packages/guides/pushing-and-pulling-docker-images
[docker-packages]: https://github.com/SirJosh3917/tinkerlang/packages
=======
<<<<<<< Updated upstream
=======
[pull-github-cr]: https://docs.github.com/en/packages/guides/configuring-docker-for-use-with-github-packages
[docker-packages]: https://github.com/SirJosh3917/tinkerlang/packages
>>>>>>> Stashed changes
>>>>>>> Stashed changes
[generate-a-parser-with-tree-sitter]: https://tree-sitter.github.io/tree-sitter/creating-parsers
[nightly-build]: https://github.com/SirJosh3917/tinkerlang/actions/workflows/build.yml
[primer.js]: https://github.com/SirJosh3917/tinkerlang/tree/main/src/ir/primer.js
[console.log]: https://developer.mozilla.org/en-US/docs/Web/API/Console/log
