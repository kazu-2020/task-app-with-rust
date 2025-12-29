# Proposal: add-user-auth

## Summary

ユーザー登録とログイン機能を実装し、タスク管理アプリにユーザー認証機能を追加します。JWT (JSON Web Token) を使用したステートレスな認証方式を採用し、パスワードハッシュ化には argon2 を使用します。

## Motivation

現在のタスク管理アプリにはユーザー認証機能がないため、複数のユーザーが個別のタスクを管理することができません。ユーザー登録とログイン機能を実装することで、各ユーザーが自分のタスクを安全に管理できるようになります。

## Scope

この変更では、以下の機能を実装します:

1. **ユーザー登録機能**: 新規ユーザーがメールアドレスとパスワードでアカウントを作成
2. **ログイン機能**: 既存ユーザーがメールアドレスとパスワードでログイン
3. **JWT トークン発行**: ログイン成功時に JWT トークンを発行
4. **認証ミドルウェア**: 保護されたエンドポイントへのアクセス制御

## Out of Scope

以下は今回の変更には含まれません:

- メールアドレス検証機能
- パスワードリセット機能
- ログイン試行制限（レートリミット）
- OAuth/SNS ログイン
- リフレッシュトークン
- ユーザープロフィール管理

## Technical Approach

### Database Schema

PostgreSQL に以下の4つのテーブルを作成:

- **`users` テーブル**: ユーザーのコア情報（id, name, created_at, updated_at）
- **`active_users` テーブル**: アクティブユーザーの状態管理（user_id [PK], activated_at）
- **`user_emails` テーブル**: メールアドレス管理（id, user_id, email, is_primary, created_at, updated_at）
  - 複数メールアドレス対応（メイン、リカバリー用など）
  - UNIQUE 制約により同じメールを複数ユーザーで使用不可
- **`password_credentials` テーブル**: パスワード認証情報（id, user_id, password_hash, created_at, updated_at）
  - 認証情報を分離し、将来的に OAuth や WebAuthn などの追加を容易にする

**設計の意図**:
- ユーザーの識別情報、状態、連絡先、認証方式を分離することで、将来的な拡張性を確保
- アクティブユーザーのみログイン可能（退会済みユーザーは active_users にレコードなし）

### Authentication Flow

1. **登録フロー**:
   - ユーザーが名前、メールアドレス、パスワードを送信
   - パスワードを argon2 でハッシュ化
   - トランザクション内で4つのテーブルに保存:
     - users (id, name)
     - active_users (user_id, activated_at)
     - user_emails (email, is_primary=true)
     - password_credentials (password_hash)
   - JWT トークンを発行して返却

2. **ログインフロー**:
   - ユーザーがメールアドレスとパスワードを送信
   - users, active_users, user_emails, password_credentials を JOIN してユーザー情報を取得
   - アクティブユーザーのみログイン可能（active_users にレコードがない場合は拒否）
   - argon2 でパスワードを検証
   - JWT トークンを発行して返却

3. **認証フロー**:
   - クライアントが Authorization ヘッダーに JWT トークンを含めてリクエスト
   - ミドルウェアで JWT トークンを検証
   - 有効な場合、ユーザー情報をリクエストコンテキストに追加

### Technology Stack

- **パスワードハッシュ化**: `argon2` crate
- **JWT**: `jsonwebtoken` crate
- **バリデーション**: `garde` crate
- **データベース**: PostgreSQL with `sqlx` crate
- **環境変数管理**: `dotenvy` crate

## Dependencies

- 既存の axum API サーバー
- PostgreSQL データベース（compose.yml で既に設定済み）

## Risks and Mitigations

### リスク 1: パスワード平文保存のリスク
- **軽減策**: argon2 による強力なハッシュ化を実装

### リスク 2: JWT トークンの漏洩
- **軽減策**: HTTPS の使用推奨、短い有効期限の設定

### リスク 3: SQL インジェクション
- **軽減策**: sqlx のプリペアドステートメント使用

## Alternatives Considered

### 代替案 1: Session Cookie 認証
- **却下理由**: API サーバーをステートレスに保つため JWT を選択

### 代替案 2: bcrypt によるパスワードハッシュ化
- **却下理由**: argon2 がより新しく強力なアルゴリズムであるため

## Related Changes

なし（初回の認証機能実装）

## Spec Deltas

- `user-registration`: ユーザー登録機能の仕様
- `user-login`: ユーザーログイン機能の仕様
- `jwt-authentication`: JWT 認証ミドルウェアの仕様
