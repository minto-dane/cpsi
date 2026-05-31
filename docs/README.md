# CPSI Documents

CPSI (Clos Package System Installer) は CPS Linux 向けに設計された軽量パッケージマネージャーです。

このディレクトリには CPSI 仕様 v0.1 Draft を、実装やレビューで参照しやすい単位に分割したドキュメントを置きます。

## 設計目標

- シンプルであること
- 高速であること
- 理解しやすいこと
- 保守しやすいこと
- パッケージが自己完結していること
- ビルドシステムとパッケージ管理を分離すること

## ドキュメント一覧

- [アーキテクチャ概要](architecture.md)
- [パッケージ形式](package-format.md)
- [バージョンとアーキテクチャ](versioning.md)
- [パッケージスクリプト](scripts.md)
- [ビルドシステム](build-system.md)
- [リポジトリ仕様](repository.md)
- [依存関係と Provides](dependencies.md)
- [インストール済みデータベース](installed-database.md)
- [署名システム](signatures.md)
- [トランザクション方針](transactions.md)
- [未確定仕様](open-issues.md)
- [設計思想](design-philosophy.md)

## 仕様ステータス

本仕様は v0.1 Draft です。現時点の設計案をまとめたものであり、将来的に変更される可能性があります。
