# Guarding

[![Build](https://github.com/inherd/guarding/actions/workflows/build.yml/badge.svg)](https://github.com/inherd/guarding/actions/workflows/build.yml)
[![crates.io](https://meritbadge.herokuapp.com/guarding)](https://crates.io/crates/guarding)
[![docs.rs](https://docs.rs/guarding/badge.svg)](https://docs.rs/guarding/)
[![license](https://img.shields.io/crates/l/guarding)](https://github.com/inherd/guarding/blob/master/LICENSE)

> Guarding is a guardians for code, architecture, layered. Using git hooks and DSL for design guard rules.

todo:

 - [x] guarding DSL design
 - [x] setup projects
 - [x] parser
    - [x] tree-sitter
 - [x] parser
 - [x] interface / trait supports
 - [ ] languages
    - [x] Java
    - [x] JavaScript
    - [ ] TypeScript
    - [ ] Rust

Others:

 - [ ] ArchUnit code generator?
 - [ ] git hooks for staged files (low-priority， some-languages has custom git hooks)

## Usage

1. install

```
cargo install guarding
```

2. create `guarding.guarding` file

```
package(".")::file.len should < 200;
package(".")::file.len should > 50;
```

3. run 

```
guarding .
```

## Development

workflow:

1. parsing guarding rules
2. parsing source code to models
3. capture rule with models

DSL capture logic:

1. filter models from `rule_level` with `rule_scope`
2. run expression
3. run assert

Queries Samples: [https://github.com/nvim-treesitter/nvim-treesitter/tree/master/queries](https://github.com/nvim-treesitter/nvim-treesitter/tree/master/queries)

## Document

### Guarding - Class or Struct function-name

for Java, JavaScript

```
# 类::名 包含 "Controller";
# 中文分词：("..myapp..") 类名称中包含 "Controller"
class("..myapp..")::function.name should contains("Model");
# or
class("..myapp..")::function.name contains("");
```

for Rust and Golang

```
struct("..myapp..")::function.name should contains("Model");
# or
struct("..myapp..")::function.name contains("");
```

License
---

This code is distributed under the MIT license. See `LICENSE` in this directory.

