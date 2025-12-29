# Spec: user-login

## Overview

ユーザーログイン機能を提供し、既存ユーザーがメールアドレスとパスワードで認証できるようにします。

## ADDED Requirements

### Requirement: ユーザーログイン API エンドポイント

API サーバーは POST /api/auth/login エンドポイントを提供しなければなりません (MUST)。

#### Scenario: 有効な認証情報でのログイン

**GIVEN** データベースに登録されているユーザー（メール: "user@example.com"、パスワード: "SecurePass123!"）

**WHEN** POST /api/auth/login に以下の JSON ボディでリクエストを送信:
```json
{
  "email": "user@example.com",
  "password": "SecurePass123!"
}
```

**THEN** ステータスコード 200 が返される

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

**AND** JWT トークンが有効で検証可能である

**AND** users, active_users, user_emails, password_credentials テーブルを JOIN してユーザー情報が取得されている

#### Scenario: 間違ったパスワードでのログイン試行

**GIVEN** データベースに登録されているユーザー（メール: "user@example.com"）

**AND** 登録時と異なるパスワード "WrongPassword!"

**WHEN** POST /api/auth/login に以下の JSON ボディでリクエストを送信:
```json
{
  "email": "user@example.com",
  "password": "WrongPassword!"
}
```

**THEN** ステータスコード 401 (Unauthorized) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Invalid email or password"
}
```

#### Scenario: 存在しないメールアドレスでのログイン試行

**GIVEN** データベースに登録されていないメールアドレス "nonexistent@example.com"

**WHEN** POST /api/auth/login に以下の JSON ボディでリクエストを送信:
```json
{
  "email": "nonexistent@example.com",
  "password": "SomePassword123!"
}
```

**THEN** ステータスコード 401 (Unauthorized) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Invalid email or password"
}
```

**AND** エラーメッセージはユーザーが存在するかどうかを明示しない（セキュリティ対策）

#### Scenario: 非アクティブユーザーのログイン試行

**GIVEN** データベースに登録されているが active_users テーブルにレコードがないユーザー（メール: "deactivated@example.com"）

**WHEN** POST /api/auth/login に以下の JSON ボディでリクエストを送信:
```json
{
  "email": "deactivated@example.com",
  "password": "SecurePass123!"
}
```

**THEN** ステータスコード 401 (Unauthorized) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Invalid email or password"
}
```

**AND** 非アクティブユーザーはログインできない

#### Scenario: 無効な入力形式でのログイン試行

**GIVEN** 空のメールアドレスまたはパスワード

**WHEN** POST /api/auth/login に以下の JSON ボディでリクエストを送信:
```json
{
  "email": "",
  "password": "SomePassword123!"
}
```

**THEN** ステータスコード 400 (Bad Request) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Email and password are required"
}
```

### Requirement: パスワード検証

ログイン時にパスワードを安全に検証しなければなりません (MUST)。

#### Scenario: argon2 によるパスワード検証

**GIVEN** データベースに保存されている argon2 ハッシュ化されたパスワード

**AND** ログインリクエストの平文パスワード

**WHEN** パスワードを検証する

**THEN** argon2 の verify 関数を使用してハッシュと平文パスワードを比較する

**AND** タイミング攻撃を防ぐために一定時間で処理を完了する

### Requirement: JWT トークン発行

ログイン成功時に JWT トークンを発行しなければなりません (MUST)。

#### Scenario: JWT トークンの生成

**GIVEN** 認証に成功したユーザー（ID: "123e4567-e89b-12d3-a456-426614174000"）

**WHEN** JWT トークンを生成する

**THEN** トークンのペイロードに以下の情報が含まれる:
- `sub`: ユーザー ID (UUID)
- `email`: ユーザーのメールアドレス
- `exp`: トークンの有効期限（発行から 24 時間後）
- `iat`: トークンの発行時刻

**AND** トークンが HS256 アルゴリズムで署名される

**AND** 署名に使用される秘密鍵が環境変数から読み込まれる

#### Scenario: JWT トークンの有効期限

**GIVEN** 新しく発行された JWT トークン

**THEN** トークンの有効期限が発行時刻から 24 時間後である

**AND** 有効期限が過ぎたトークンは無効として扱われる

### Requirement: セキュリティ対策

ログイン機能はセキュリティベストプラクティスに従わなければなりません (MUST)。

#### Scenario: エラーメッセージの一貫性

**GIVEN** 存在しないメールアドレスまたは間違ったパスワードでのログイン試行

**THEN** どちらの場合も同じエラーメッセージ "Invalid email or password" が返される

**AND** エラーメッセージからユーザーの存在を推測できない
