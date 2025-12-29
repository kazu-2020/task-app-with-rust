# Tasks: add-user-auth

## Overview

このドキュメントは、ユーザー登録とログイン機能の実装タスクを定義します。タスクは段階的に実装され、各ステップでテストと検証を行います。

## Completion Checklist

- [] 1. データベースマイグレーション: users テーブルの作成
- [] 2. プロジェクトに必要な依存関係を追加
- [] 3. 環境変数設定ファイルの作成
- [] 4. データベース接続とコネクションプールの設定
- [] 5. ユーザーモデルの実装
- [] 6. パスワードハッシュ化ユーティリティの実装
- [] 7. JWT ユーティリティの実装
- [] 8. ユーザーリポジトリの実装
- [] 9. ユーザー登録エンドポイントの実装
- [] 10. ユーザーログインエンドポイントの実装
- [] 11. JWT 認証ミドルウェアの実装
- [] 12. ルーターの設定
- [] 13. 保護されたエンドポイントの例の実装
- [] 14. エラーハンドリングの改善
- [] 15. 統合テストの作成
- [] 16. ドキュメントの作成

## Task List

### 1. データベースマイグレーション: users, active_users, user_emails, password_credentials テーブルの作成

**目的**: PostgreSQL にユーザー関連の4つのテーブルを作成する

**成果物**:
- マイグレーション SQL ファイル（up/down）
- **users テーブル**（id, name, created_at, updated_at）
- **active_users テーブル**（user_id [PK], activated_at）
- **user_emails テーブル**（id, user_id, email, is_primary, created_at, updated_at）
- **password_credentials テーブル**（id, user_id, password_hash, created_at, updated_at）

**テーブル設計の意図**:
- **users**: 全ユーザーのマスターデータ。コアな識別情報のみ（id, name）
- **active_users**: アクティブなユーザーを管理する状態テーブル
  - このテーブルにレコードが存在する = アクティブユーザー
  - 存在しない = まだアクティベートされていない、または退会済み
  - 将来的に deactivated_users, suspended_users などを追加可能（状態別テーブルパターン）
- **user_emails**: email を別テーブルで管理し、複数 email 対応を可能にする
  - is_primary: メインアドレスかどうか（通知送信先などに使用）
  - 1ユーザーが複数の email を持てる（メイン、リカバリー用、仕事用/個人用など）
- **password_credentials**: 認証情報を分離し、将来的に複数の認証方式（OAuth、WebAuthn など）をサポート可能にする

**制約**:
- **users テーブル**:
  - name: NOT NULL, CHECK (LENGTH(TRIM(name)) > 0) - 空文字列禁止
- **active_users テーブル**:
  - user_id: 外部キー制約（users.id への参照、ON DELETE CASCADE）
- **user_emails テーブル**:
  - email: NOT NULL, UNIQUE - 同じ email を複数ユーザーが使えない
  - user_id: 外部キー制約（users.id への参照、ON DELETE CASCADE）
  - 1ユーザーにつき is_primary=true の email は1つのみ（アプリケーションレベルまたは部分インデックスで制御）
- **password_credentials テーブル**:
  - password_hash: NOT NULL
  - user_id: 外部キー制約（users.id への参照、ON DELETE CASCADE）

**検証**:
- マイグレーション実行後、各テーブルの構造を確認（`\d users`, `\d active_users`, `\d user_emails`, `\d password_credentials`）
- 各外部キー制約と UNIQUE 制約が設定されていることを確認
- ON DELETE CASCADE が設定されていることを確認

**依存関係**: なし

**備考**: sqlx-cli を使用してマイグレーションファイルを作成

---

### 2. プロジェクトに必要な依存関係を追加

**目的**: Cargo.toml に認証に必要なクレートを追加する

**成果物**:
- 更新された api/Cargo.toml

**追加するクレート**:
- `sqlx` (features = ["runtime-tokio", "postgres", "uuid", "chrono"])
- `argon2` (パスワードハッシュ化)
- `jsonwebtoken` (default-features = false, features = ["aws_lc_rs"]) (JWT トークン生成・検証)
- `garde` (features = ["derive"]) (入力バリデーション)
- `dotenvy` (環境変数管理)
- `serde` (features = ["derive"])
- `serde_json`
- `chrono` (features = ["serde"])
- `uuid` (features = ["v4", "serde"])

**検証**:
- `cargo build` が成功する

**依存関係**: なし

---

### 3. 環境変数設定ファイルの作成

**目的**: .env ファイルを作成し、必要な環境変数を定義する

**成果物**:
- .env ファイル（DATABASE_URL, JWT_SECRET）
- .env.example ファイル（サンプル）

**検証**:
- .env ファイルが .gitignore に含まれていることを確認
- DATABASE_URL が PostgreSQL に接続できることを確認

**依存関係**: タスク 2（依存関係の追加）

---

### 4. データベース接続とコネクションプールの設定

**目的**: sqlx を使用してデータベース接続を確立する

**成果物**:
- データベース接続を管理する `db` モジュール
- コネクションプールの初期化コード
- main.rs での接続プール設定

**検証**:
- サーバー起動時にデータベース接続が成功する
- コネクションプールが正常に動作する

**依存関係**: タスク 3（環境変数設定）

---

### 5. ユーザーモデルの実装

**目的**: User, ActiveUser, UserEmail, PasswordCredential 構造体と関連する型を定義する

**成果物**:
- `domain/entities/user.rs` モジュール
- **User 構造体**（id, name, created_at, updated_at）
- **ActiveUser 構造体**（user_id, activated_at）
- **UserEmail 構造体**（id, user_id, email, is_primary, created_at, updated_at）
- **PasswordCredential 構造体**（id, user_id, password_hash, created_at, updated_at）
- CreateUserDto 構造体（name, email, password）
- LoginDto 構造体（email, password）
- UserResponse 構造体（id, name, email, created_at）
  - email はプライマリアドレスを返す

**設計の意図**:
- **User**: ユーザーのコアな識別情報のみ（id, name）
- **ActiveUser**: アクティブ状態を表現。このレコードが存在する = アクティブユーザー
- **UserEmail**: email 管理を分離し、複数 email をサポート
- **PasswordCredential**: パスワード認証に特化した認証情報
- 各モデルは単一責任を持ち、将来的な拡張に強い設計

**検証**:
- 構造体が正しくシリアライズ/デシリアライズできる
- `cargo check` が成功する

**依存関係**: タスク 2（依存関係の追加）

---

### 6. パスワードハッシュ化ユーティリティの実装

**目的**: argon2 を使用したパスワードハッシュ化と検証機能を実装する

**成果物**:
- `infrastructure/password.rs` モジュール
- `hash_password(password: &str) -> Result<String>` 関数
- `verify_password(password: &str, hash: &str) -> Result<bool>` 関数

**検証**:
- パスワードが正しくハッシュ化される
- ハッシュ化されたパスワードを元に戻せない
- 正しいパスワードで検証が成功する
- 間違ったパスワードで検証が失敗する
- ユニットテストを作成して実行

**依存関係**: タスク 2（依存関係の追加）

---

### 7. JWT ユーティリティの実装

**目的**: JWT トークンの生成と検証機能を実装する

**成果物**:
- `infrastructure/jwt.rs` モジュール
- `generate_token(user_id: Uuid, email: &str) -> Result<String>` 関数
- `verify_token(token: &str) -> Result<Claims>` 関数
- Claims 構造体（sub, email, exp, iat）

**検証**:
- トークンが正しく生成される
- 生成されたトークンが検証できる
- 有効期限切れのトークンが拒否される
- 不正なトークンが拒否される
- ユニットテストを作成して実行

**依存関係**: タスク 3（環境変数設定）

---

### 8. ユーザーリポジトリの実装

**目的**: データベース操作を行うリポジトリ層を実装する

**成果物**:
- `domain/repositories/user_repository.rs` モジュール（trait 定義）
- `infrastructure/repositories/user_repository_impl.rs` モジュール（実装）
- `create_active_user_with_password(pool: &PgPool, name: &str, email: &str, password_hash: &str) -> Result<User>` 関数
  - トランザクション内で4テーブル（users, active_users, user_emails, password_credentials）にレコードを作成
- `find_active_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>>` 関数
  - active_users と JOIN してアクティブユーザーのみ返す
- `find_user_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>>` 関数
- `is_user_active(pool: &PgPool, user_id: Uuid) -> Result<bool>` 関数
  - active_users テーブルにレコードが存在するか確認
- `find_primary_email(pool: &PgPool, user_id: Uuid) -> Result<Option<UserEmail>>` 関数
- `find_password_credential_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Option<PasswordCredential>>` 関数
- `verify_active_user_password(pool: &PgPool, email: &str, password: &str) -> Result<Option<User>>` 関数
  - users, active_users, user_emails, password_credentials を JOIN してアクティブユーザーの認証

**実装の重要ポイント**:
- `create_active_user_with_password` のトランザクション処理：
  1. users にレコード挿入（id, name）
  2. active_users にレコード挿入（user_id, activated_at）
  3. user_emails にプライマリメール挿入（user_id, email, is_primary=true）
  4. password_credentials にパスワードハッシュ挿入（user_id, password_hash）
  5. いずれかが失敗したら全てロールバック
- ログイン処理では4テーブルを JOIN して1クエリで取得（アクティブ状態も同時に確認）
- アクティブでないユーザーはログインできない設計

**検証**:
- ユーザー、アクティブ状態、メールアドレス、パスワード認証情報が正しく作成される
- トランザクションが正しく動作する（どれか1つが失敗したら全てロールバック）
- アクティブなユーザーのみログインできる
- メールアドレスでアクティブユーザーを検索できる
- 統合テストを作成して実行

**依存関係**: タスク 4（データベース接続）、タスク 5（ユーザーモデル）

---

### 9. ユーザー登録エンドポイントの実装

**目的**: POST /api/auth/register エンドポイントを実装する

**成果物**:
- `presentation/handlers/auth.rs` モジュール
- `register` ハンドラー関数
- リクエスト/レスポンスの型定義
- 入力バリデーション

**検証**:
- 有効な入力でユーザー登録が成功する
- 既存のメールアドレスで 409 エラーが返る
- 無効なメールアドレスで 400 エラーが返る
- パスワードが短すぎる場合に 400 エラーが返る
- 統合テストを作成して実行

**依存関係**: タスク 6（パスワードハッシュ化）、タスク 7（JWT）、タスク 8（リポジトリ）

---

### 10. ユーザーログインエンドポイントの実装

**目的**: POST /api/auth/login エンドポイントを実装する

**成果物**:
- `login` ハンドラー関数（auth.rs に追加）
- リクエスト/レスポンスの型定義

**検証**:
- 正しい認証情報でログインが成功する
- 間違ったパスワードで 401 エラーが返る
- 存在しないユーザーで 401 エラーが返る
- エラーメッセージが一貫している
- 統合テストを作成して実行

**依存関係**: タスク 6（パスワードハッシュ化）、タスク 7（JWT）、タスク 8（リポジトリ）

---

### 11. JWT 認証ミドルウェアの実装

**目的**: JWT トークンを検証する認証ミドルウェアを実装する

**成果物**:
- `presentation/middleware/auth.rs` モジュール
- `require_auth` ミドルウェア関数
- ユーザーコンテキストの型定義

**検証**:
- 有効なトークンで認証が成功する
- トークンなしで 401 エラーが返る
- 無効なトークンで 401 エラーが返る
- 有効期限切れのトークンで 401 エラーが返る
- ユーザー情報がリクエストコンテキストに追加される
- ユニットテストを作成して実行

**依存関係**: タスク 7（JWT）

---

### 12. ルーターの設定

**目的**: 認証エンドポイントをルーターに登録する

**成果物**:
- 更新された main.rs
- /api/auth/register ルート
- /api/auth/login ルート

**検証**:
- curl または Postman で各エンドポイントにアクセスできる
- サーバーが正常に起動する

**依存関係**: タスク 9（登録エンドポイント）、タスク 10（ログインエンドポイント）

---

### 13. 保護されたエンドポイントの例の実装

**目的**: 認証ミドルウェアを使用した保護されたエンドポイントの例を実装する

**成果物**:
- GET /api/me エンドポイント（現在のユーザー情報を返す）

**検証**:
- 有効なトークンでユーザー情報が取得できる
- トークンなしで 401 エラーが返る
- 統合テストを作成して実行

**依存関係**: タスク 11（認証ミドルウェア）、タスク 12（ルーター設定）

---

### 14. エラーハンドリングの改善

**目的**: 統一されたエラーレスポンス形式を実装する

**成果物**:
- `error.rs` モジュール
- AppError 型
- エラーレスポンスの標準化

**検証**:
- すべてのエラーが一貫した形式で返される
- 適切なステータスコードが設定される

**依存関係**: タスク 9-13（各エンドポイント実装）

**並列化可能**: このタスクは他のタスクと並行して実装可能

---

### 15. 統合テストの作成

**目的**: E2E テストを作成し、全体的な動作を確認する

**成果物**:
- tests/auth_test.rs
- 登録からログイン、保護されたエンドポイントへのアクセスまでの一連のフロー

**検証**:
- `cargo test` が成功する
- すべてのシナリオがカバーされている

**依存関係**: タスク 13（保護されたエンドポイント）

---

### 16. ドキュメントの作成

**目的**: API ドキュメントと使用方法を記述する

**成果物**:
- api/README.md の更新
- API エンドポイントの説明
- リクエスト/レスポンスの例
- 環境変数の説明

**検証**:
- ドキュメントを参照して API を使用できる

**依存関係**: タスク 15（統合テスト）

**並列化可能**: このタスクは実装完了後に並行して作成可能

## Task Dependencies Graph

```
1 (DB Migration) → 4 (DB Connection) → 8 (Repository) → 9 (Register)
                                                       → 10 (Login)
2 (Dependencies) → 3 (Env Vars) → 4 (DB Connection)
                → 5 (User Model) → 8 (Repository)
                → 6 (Password) → 9 (Register)
                               → 10 (Login)
                → 7 (JWT) → 9 (Register)
                          → 10 (Login)
                          → 11 (Auth Middleware)

9 (Register) → 12 (Router)
10 (Login) → 12 (Router)

11 (Auth Middleware) → 13 (Protected Endpoint)
12 (Router) → 13 (Protected Endpoint)

13 (Protected Endpoint) → 15 (Integration Tests) → 16 (Docs)

14 (Error Handling) - 並列実装可能
```

## Parallelizable Tasks

以下のタスクは並行して実装可能です:

- タスク 5 (User Model), 6 (Password Utils), 7 (JWT Utils) は独立して実装可能
- タスク 14 (Error Handling) は他のタスクと並行して実装可能
- タスク 16 (Docs) は実装完了後に並行して作成可能
