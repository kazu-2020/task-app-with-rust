# Spec: user-registration

## Overview

ユーザー登録機能を提供し、新規ユーザーがメールアドレスとパスワードでアカウントを作成できるようにします。

## ADDED Requirements

### Requirement: ユーザー登録 API エンドポイント

API サーバーは POST /api/auth/register エンドポイントを提供しなければなりません (MUST)。

#### Scenario: 有効な入力での新規ユーザー登録

**GIVEN** データベースに登録されていない名前 "John Doe"、メールアドレス "user@example.com"、有効なパスワード "SecurePass123!"

**WHEN** POST /api/auth/register に以下の JSON ボディでリクエストを送信:
```json
{
  "name": "John Doe",
  "email": "user@example.com",
  "password": "SecurePass123!"
}
```

**THEN** ステータスコード 201 が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "user": {
    "id": "<UUID>",
    "name": "John Doe",
    "email": "user@example.com",
    "created_at": "<ISO8601 timestamp>"
  },
  "token": "<JWT token>"
}
```

**AND** トランザクション内で以下の4つのテーブルにデータが保存される:
- users テーブル (id, name)
- active_users テーブル (user_id, activated_at)
- user_emails テーブル (email, is_primary=true)
- password_credentials テーブル (password_hash)

**AND** パスワードが argon2 でハッシュ化されて保存される

#### Scenario: 既存のメールアドレスでの登録試行

**GIVEN** データベースに既に登録されているメールアドレス "existing@example.com"

**WHEN** POST /api/auth/register に以下の JSON ボディでリクエストを送信:
```json
{
  "name": "John Doe",
  "email": "existing@example.com",
  "password": "SecurePass123!"
}
```

**THEN** ステータスコード 409 (Conflict) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Email already registered"
}
```

#### Scenario: 無効なメールアドレス形式での登録試行

**GIVEN** 無効な形式のメールアドレス "invalid-email"

**WHEN** POST /api/auth/register に以下の JSON ボディでリクエストを送信:
```json
{
  "name": "John Doe",
  "email": "invalid-email",
  "password": "SecurePass123!"
}
```

**THEN** ステータスコード 400 (Bad Request) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Invalid email format"
}
```

#### Scenario: パスワードが短すぎる場合の登録試行

**GIVEN** 8文字未満のパスワード "short"

**WHEN** POST /api/auth/register に以下の JSON ボディでリクエストを送信:
```json
{
  "name": "John Doe",
  "email": "user@example.com",
  "password": "short"
}
```

**THEN** ステータスコード 400 (Bad Request) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Password must be at least 8 characters long"
}
```

### Requirement: データベーススキーマ

PostgreSQL データベースに以下の4つのテーブルが存在しなければなりません (MUST)。

#### Scenario: users テーブルの構造

**GIVEN** PostgreSQL データベースが利用可能

**THEN** 以下のカラムを持つ users テーブルが存在する:
- `id`: UUID 型、主キー、デフォルト値は自動生成
- `name`: VARCHAR(100) 型、NOT NULL、CHECK (LENGTH(TRIM(name)) > 0)
- `created_at`: TIMESTAMP WITH TIME ZONE 型、NOT NULL、デフォルト値は現在時刻
- `updated_at`: TIMESTAMP WITH TIME ZONE 型、NOT NULL、デフォルト値は現在時刻

#### Scenario: active_users テーブルの構造

**GIVEN** PostgreSQL データベースが利用可能

**THEN** 以下のカラムを持つ active_users テーブルが存在する:
- `user_id`: UUID 型、主キー、users.id への外部キー、ON DELETE CASCADE
- `activated_at`: TIMESTAMP WITH TIME ZONE 型、NOT NULL、デフォルト値は現在時刻

#### Scenario: user_emails テーブルの構造

**GIVEN** PostgreSQL データベースが利用可能

**THEN** 以下のカラムを持つ user_emails テーブルが存在する:
- `id`: UUID 型、主キー、デフォルト値は自動生成
- `user_id`: UUID 型、NOT NULL、users.id への外部キー、ON DELETE CASCADE
- `email`: VARCHAR(255) 型、NOT NULL、UNIQUE
- `is_primary`: BOOLEAN 型、NOT NULL、デフォルト値は false
- `created_at`: TIMESTAMP WITH TIME ZONE 型、NOT NULL、デフォルト値は現在時刻
- `updated_at`: TIMESTAMP WITH TIME ZONE 型、NOT NULL、デフォルト値は現在時刻

**AND** email カラムに UNIQUE インデックスが設定されている

#### Scenario: password_credentials テーブルの構造

**GIVEN** PostgreSQL データベースが利用可能

**THEN** 以下のカラムを持つ password_credentials テーブルが存在する:
- `id`: UUID 型、主キー、デフォルト値は自動生成
- `user_id`: UUID 型、NOT NULL、users.id への外部キー、ON DELETE CASCADE
- `password_hash`: TEXT 型、NOT NULL
- `created_at`: TIMESTAMP WITH TIME ZONE 型、NOT NULL、デフォルト値は現在時刻
- `updated_at`: TIMESTAMP WITH TIME ZONE 型、NOT NULL、デフォルト値は現在時刻

### Requirement: パスワードハッシュ化

パスワードは argon2 アルゴリズムでハッシュ化されなければなりません (MUST)。

#### Scenario: パスワードのハッシュ化

**GIVEN** 平文パスワード "SecurePass123!"

**WHEN** パスワードがハッシュ化される

**THEN** argon2id バリアントが使用される

**AND** ハッシュ化されたパスワードが元のパスワードと異なる

**AND** ハッシュ化されたパスワードから元のパスワードを復元できない

### Requirement: 入力バリデーション

登録リクエストの入力は適切にバリデーションされなければなりません (MUST)。

#### Scenario: 名前のバリデーション

**GIVEN** 登録リクエストの名前

**THEN** 名前が空でない必要がある

**AND** 名前がトリム後に1文字以上である必要がある

**AND** 名前が100文字以内である必要がある

#### Scenario: メールアドレスのバリデーション

**GIVEN** 登録リクエストのメールアドレス

**THEN** メールアドレスが RFC 5322 に準拠した形式である必要がある

**AND** メールアドレスが空でない必要がある

**AND** メールアドレスが255文字以内である必要がある

#### Scenario: パスワードのバリデーション

**GIVEN** 登録リクエストのパスワード

**THEN** パスワードが最低 8 文字以上である必要がある

**AND** パスワードが空でない必要がある
