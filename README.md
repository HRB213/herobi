![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)
![Language: Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![Last Commit](https://img.shields.io/github/last-commit/HRB213/herobi)
[![Coverage Status](https://coveralls.io/repos/github/HRB213/herobi/badge.svg?branch=main)](https://coveralls.io/github/HRB213/herobi?branch=main)

# herobi

ディレクトリ内の構成を素早く把握するためのCLIツール

## Description

大量のファイルが存在するディレクトリでは、単なる一覧表示だけでは構成を把握しづらい場合があります。

**herobi** は、指定したディレクトリ内のファイルやディレクトリを一覧表示するだけでなく、ファイルを種類ごとに分類し、ディレクトリごとの使用容量や全体の構成を可視化することで、ディレクトリ構成を一目で把握できるCLIツールです。

## Features

- 指定したディレクトリ内のファイル・ディレクトリ一覧表示
- ファイルの種類ごとの自動分類
- ディレクトリごとの使用容量表示
- ディレクトリ全体の件数・容量サマリー表示

## Installation

### Homebrew

```bash
brew install HRB213/homebrew-tap/herobi
```

### Docker

```bash
docker pull hrb213/herobi
```

## Homebrew Usage

```text
herobi [OPTIONS] [PATH]
```

指定したディレクトリ内の構成を表示します。

`PATH` を省略した場合は、カレントディレクトリを対象とします。

### Arguments

```text
PATH    対象とするディレクトリのパス（省略可能）
```

### Options

```text
-c, --category    ファイルを種類ごとに分類して表示する
-s, --size        ディレクトリごとの使用容量を表示する
-m, --summary     ファイル数・ディレクトリ数・合計容量を表示する
-h, --help        ヘルプを表示する
-V, --version     バージョン情報を表示する
```

## Examples

### 一覧表示

```bash
herobi
```

```text
Cargo.toml
README.md
docs
src
target
```

### ファイルを種類ごとに分類

```bash
herobi -c
```

```text
Directories
-----------
docs
src
target

Rust
----
main.rs
gencomp.rs

Markdown
---------
README.md

Other
-----
Cargo.toml
LICENSE
```

### ディレクトリごとの使用容量

```bash
herobi -s
```

```text
Directory Size
--------------
docs        24 KB
src         38 KB
target      2.1 MB
```

### ディレクトリ全体のサマリー

```bash
herobi -m
```

```text
Summary
-------
Directories : 3
Files       : 8
Total Size  : 2.2 MB
```

### すべての機能を表示

```bash
herobi -c -s -m
```

```text
Directories
-----------
docs
src
target

Rust
----
main.rs
gencomp.rs

Markdown
---------
README.md

Other
-----
Cargo.toml
LICENSE

Directory Size
--------------
docs        24 KB
src         38 KB
target      2.1 MB

Summary
-------
Directories : 3
Files       : 8
Total Size  : 2.2 MB
```

## Docker Usage

Docker版では、対象とするディレクトリをコンテナへマウントして実行します。

```bash
docker run --rm -v "$PWD":/work -w /work hrb213/herobi [OPTIONS] [PATH]
```