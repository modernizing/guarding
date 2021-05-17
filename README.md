# Guarding

[![Build](https://github.com/inherd/guarding/actions/workflows/build.yml/badge.svg)](https://github.com/inherd/guarding/actions/workflows/build.yml)

> Guarding is a guardians for code, architecture, layered. Using git hooks and DSL for design guard rules.

todo:

 - [ ] git hooks for staged files
 - [x] guarding DSL design
 - [x] setup projects
 - [x] parser
    - [x] tree-sitter
 - [ ] languages
    - [ ] Java normal
    - [ ] Java ArchUnit generator
    - [ ] JavaScript / TypeScript
    - [ ] Rust

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

