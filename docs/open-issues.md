# 未確定仕様

この文書は CPSI v0.1 Draft 時点で未確定の仕様をまとめます。

## `remove`

コマンド:

```bash
cpsi remove foo
```

未決定:

- remove script
- orphan package 処理

## `upgrade`

コマンド:

```bash
cpsi upgrade
```

詳細仕様は未決定です。

## conflicts

候補:

```toml
conflicts = [
    "vim"
]
```

現時点では未採用です。

## repository signature

候補:

```text
Packages.zst.minisig
```

現時点では未採用です。

## file conflict policy

例:

```text
pkgA -> /usr/bin/foo
pkgB -> /usr/bin/foo
```

処理方針は未決定です。
