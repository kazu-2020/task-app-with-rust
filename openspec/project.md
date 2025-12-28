# Project Context

## Purpose

Rust での Web アプケーション開発に慣れることが目的です。
このプロジェクトでは、タスク管理アプリを作成します

## Tech Stack

- Rust: v1.92.0
- API Framework: [axum](https://docs.rs/axum/latest/axum/index.html) v0.8.8
- Frontned Framework: [dioxus](https://github.com/dioxuslabs/dioxus) v0.7.1
- DB: PostgreSQL v18

## Project Conventions

### Code Style

[Describe your code style preferences, formatting rules, and naming conventions]

### Architecture Patterns

#### API アーキテクチャ

API部分では**レイヤードアーキテクチャ**を採用します。

各層の責務は以下の通りです：

- **プレゼンテーション層（Presentation Layer）**
  - HTTPリクエスト/レスポンスの処理
  - ルーティング定義
  - リクエストのバリデーション
  - axumのハンドラー関数

- **アプリケーション層（Application Layer）**
  - ユースケースの実装
  - トランザクション管理
  - 複数のドメインサービスの調整

- **ドメイン層（Domain Layer）**
  - **Entity**: ビジネス概念を表現するドメインオブジェクト
  - **Domain Service**: 複数のエンティティにまたがるドメインロジック
  - **Repository Interface**: データ永続化の抽象インターフェース

- **インフラストラクチャ層（Infrastructure Layer）**
  - Repository の実装
  - データベースアクセス
  - 外部APIとの連携

依存関係の方向：プレゼンテーション → アプリケーション → ドメイン ← インフラストラクチャ

#### ディレクトリ構造

```text
api/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs
    ├── presentation/          # プレゼンテーション層
    │   ├── mod.rs
    │   ├── routes.rs         # ルーティング定義
    │   └── handlers/         # HTTPハンドラー
    │       ├── mod.rs
    │       └── task_handler.rs
    ├── application/           # アプリケーション層
    │   ├── mod.rs
    │   └── usecases/         # ユースケース
    │       ├── mod.rs
    │       └── task_usecase.rs
    ├── domain/                # ドメイン層
    │   ├── mod.rs
    │   ├── entities/         # エンティティ
    │   │   ├── mod.rs
    │   │   └── task.rs
    │   ├── services/         # ドメインサービス
    │   │   ├── mod.rs
    │   │   └── task_service.rs
    │   └── repositories/     # リポジトリインターフェース
    │       ├── mod.rs
    │       └── task_repository.rs
    └── infrastructure/        # インフラストラクチャ層
        ├── mod.rs
        ├── database/         # DB接続設定
        │   ├── mod.rs
        │   └── connection.rs
        └── repositories/     # リポジトリ実装
            ├── mod.rs
            └── task_repository_impl.rs
```

### Testing Strategy

#### 基本方針

レイヤードアーキテクチャの各層で適切なテストを実施します。
Application層以降は **testcontainers を使った実DBテスト** を基本とし、DB制約やトランザクションの挙動も含めて検証します。

#### テストツール

- **cargo test**: Rust標準のテストフレームワーク
- **testcontainers-rs**: Docker コンテナを使った実DBテスト環境の構築
- **sqlx**: 型安全なクエリビルダー＆マイグレーション
- **mockall**: 必要に応じたモックオブジェクト生成

#### 各層のテスト戦略

##### Domain層

- **対象**: Entity, Domain Service, Repository trait（インターフェース）
- **方針**: 純粋なビジネスロジックのテスト。外部依存なし
- **配置**: 各モジュール内の `#[cfg(test)] mod tests`
- **例**:
  - Entityのバリデーションロジック
  - Domain Serviceの計算ロジック

##### Application層

- **対象**: UseCase
- **方針**: **testcontainers で実DBを使った統合テスト**
- **理由**:
  - DB制約（UNIQUE, FOREIGN KEY）の検証
  - トランザクションの動作確認
  - より現実に近いテスト環境
- **配置**: `tests/` ディレクトリ（統合テスト）
- **例**:
  - タスク作成時の重複チェック
  - ロールバック処理の検証
  - 複数テーブルにまたがる処理

##### Infrastructure層

- **対象**: Repository実装, DB接続, マイグレーション
- **方針**: testcontainersで実DBを使ったテスト
- **配置**: `tests/` ディレクトリ（統合テスト）
- **例**:
  - CRUDオペレーションの正確性
  - クエリのパフォーマンス
  - マイグレーションの整合性

##### Presentation層

- **対象**: HTTPハンドラー, ルーティング
- **方針**: axumのテストヘルパーを使ったHTTPリクエスト/レスポンステスト
- **配置**: 各モジュール内の `#[cfg(test)] mod tests` または `tests/` ディレクトリ
- **例**:
  - リクエストバリデーション
  - ステータスコードの検証
  - レスポンスボディの確認

### Git Workflow

#### ブランチ戦略: GitHub Flow

シンプルで効率的な **GitHub Flow** を採用します。

- **main ブランチ**
  - 常にデプロイ可能な状態を維持
  - 直接コミットは禁止
  - プルリクエスト経由でのみマージ

- **feature ブランチ**
  - mainブランチから作成
  - 機能開発、バグフィックス、ドキュメント更新など全ての変更に使用
  - 作業完了後はプルリクエストを作成

#### ブランチ命名規則

```
feature/<issue-number>-<short-description>
fix/<issue-number>-<short-description>
docs/<short-description>
```

**例**:
- `feature/123-add-task-creation`
- `fix/456-resolve-validation-error`
- `docs/update-architecture`

#### コミットメッセージ規約: Conventional Commits

構造化されたコミットメッセージで変更履歴を明確にします。

##### フォーマット

```
<type>(<scope>): <subject>

<body>

<footer>
```

##### Type（必須）

- **feat**: 新機能の追加
- **fix**: バグ修正
- **docs**: ドキュメントのみの変更
- **style**: コードの意味に影響しない変更（フォーマット、セミコロンなど）
- **refactor**: バグ修正や機能追加を伴わないコード変更
- **perf**: パフォーマンス改善
- **test**: テストの追加・修正
- **chore**: ビルドプロセスやツールの変更

##### Scope（任意）

変更の影響範囲を示す（例: api, domain, infrastructure）

##### 例

```
feat(api): タスク作成エンドポイントを追加

POST /tasks でタスクを作成できるようにした。
- リクエストバリデーションを実装
- データベースへの保存処理を追加

Closes #123
```

```
fix(domain): タスクのバリデーションロジックを修正

タイトルが空の場合にエラーを返すように修正
```

#### プルリクエストルール

1. **タイトル**: Conventional Commits形式で記載
2. **説明**: 変更内容、理由、影響範囲を明記
3. **レビュー**: セルフレビュー必須（個人開発の場合も変更を再確認）
4. **テスト**: 関連するテストが通ることを確認
5. **マージ**: Squash and Merge を推奨（コミット履歴をクリーンに保つ）

## Domain Context

[Add domain-specific knowledge that AI assistants need to understand]

## Important Constraints

[List any technical, business, or regulatory constraints]

## External Dependencies

[Document key external services, APIs, or systems]
