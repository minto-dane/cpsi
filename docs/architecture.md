# アーキテクチャ概要

CPS エコシステムは、ビルドレシピからインストール済みシステムまでを明確に分離します。

```text
.cpsb
 ↓
cpsbuild
 ↓
.clos
 ↓
Repository
 ↓
cpsi
 ↓
Installed System
```

## コンポーネント

| コンポーネント | 役割 |
| --- | --- |
| `.cpsb` | パッケージビルドレシピ |
| `cpsbuild` | パッケージ生成 |
| `.clos` | 配布用パッケージ |
| Repository | パッケージ配布 |
| `cpsi` | パッケージ管理 |
| Installed System | インストール済み環境 |

## 責務分離

`.cpsb` と `cpsbuild` はパッケージ作成を担当します。

`.clos`、Repository、`cpsi` はパッケージ配布とインストールを担当します。

利用者は通常 `.cpsb` を直接取得せず、リポジトリから `.clos` を取得して `cpsi` で管理します。
