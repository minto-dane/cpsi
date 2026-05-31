# パッケージ形式

CPSI の配布用パッケージは `.clos` 拡張子を持ちます。

## 拡張子

```text
.clos
```

例:

```text
firefox-139.0-k1-x86_64.clos
glibc-2.42-k3-x86_64.clos
```

## ファイル名構造

形式:

```text
<name>-<version>-k<release>-<arch>.clos
```

例:

```text
firefox-139.0-k1-x86_64.clos
```

| 項目 | 説明 |
| --- | --- |
| `name` | パッケージ名 |
| `version` | 上流バージョン |
| `release` | ディストリビューション修正版番号 |
| `arch` | 対象アーキテクチャ |

## 内部構造

```text
/
├── .pkg/
│   ├── info
│   └── scripts/
│       ├── post
│       └── pre (optional)
│
└── data/
    ├── usr/
    ├── etc/
    ├── lib/
    └── ...
```

`.pkg/` はメタデータとスクリプトを保持します。

`data/` はインストール対象ファイルを保持します。

## `.pkg/info`

`.pkg/info` は TOML 形式です。

```toml
name = "firefox"
version = "139.0"
release = 1
arch = "x86_64"

description = "Web Browser"
license = "MPL-2.0"

package_size = 45678901
installed_size = 123456789

depends = [
    "glibc>=2.42",
    "gtk4>=4.18"
]

provides = [
    "browser"
]
```

## サイズ情報

`package_size` はパッケージファイルのサイズです。

主な用途:

```bash
cpsi info firefox
```

`installed_size` は展開後サイズです。

主な用途:

- インストール容量計算
- インストール前確認
