# インストール済みデータベース

CPSI はインストール済み状態を Apache Parquet 形式で保存します。

## 形式

```text
Apache Parquet
```

## 保存場所候補

```text
/var/lib/cpsi/
```

## `packages.parquet`

保持内容:

```text
name
version
release
arch
install_time
```

## `files.parquet`

保持内容:

```text
package
path
```

例:

```text
firefox -> /usr/bin/firefox
firefox -> /usr/share/applications/firefox.desktop
```

用途:

- パッケージ削除
- ファイル衝突検出
- 所有者検索
