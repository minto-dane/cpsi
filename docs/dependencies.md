# 依存関係と Provides

CPSI は単純な依存記法とグラフ探索による依存解決を採用します。

## 依存関係の記法

```toml
depends = [
    "glibc>=2.42",
    "openssl>=3.5",
    "gtk4>=4.18"
]
```

## 対応演算子

```text
=
>
>=
<
<=
```

## 非対応の記法

現時点では以下をサポートしません。

```text
^
~
*
```

理由:

- 実装を単純に保つため

## Provides

例:

```toml
provides = [
    "vi"
]
```

候補が 1 つの場合は自動選択します。

```text
自動選択
```

候補が複数ある場合は利用者に問い合わせます。

```text
1. vim
2. neovim
3. busybox

選択してください:
```

## 依存解決アルゴリズム

依存解決はグラフ探索方式で行います。

例:

```text
A
├─ B
│  ├─ D
│  └─ E
└─ C
```

## 状態管理

探索中は以下の状態を保持します。

```text
visiting
visited
```

## 循環依存検出

例:

```text
A
↓
B
↓
C
↓
A
```

循環依存を発見した場合は、以下のエラーで終了します。

```text
dependency cycle detected
```
