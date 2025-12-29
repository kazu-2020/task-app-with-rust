# Spec: jwt-authentication

## Overview

JWT トークンを使用した認証ミドルウェアを提供し、保護されたエンドポイントへのアクセスを制御します。

## ADDED Requirements

### Requirement: 認証ミドルウェア

API サーバーは JWT トークンを検証する認証ミドルウェアを提供しなければなりません (MUST)。

#### Scenario: 有効な JWT トークンでの認証

**GIVEN** 有効な JWT トークンを含む Authorization ヘッダー "Bearer <valid_token>"

**WHEN** 保護されたエンドポイントにリクエストを送信する

**THEN** トークンが検証される

**AND** リクエストが正常に処理される

**AND** ユーザー情報がリクエストコンテキストに追加される

#### Scenario: JWT トークンなしでの保護されたエンドポイントへのアクセス

**GIVEN** Authorization ヘッダーが含まれないリクエスト

**WHEN** 保護されたエンドポイントにリクエストを送信する

**THEN** ステータスコード 401 (Unauthorized) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Authentication required"
}
```

#### Scenario: 無効な JWT トークンでの認証試行

**GIVEN** 無効な JWT トークンを含む Authorization ヘッダー "Bearer invalid_token"

**WHEN** 保護されたエンドポイントにリクエストを送信する

**THEN** ステータスコード 401 (Unauthorized) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Invalid or expired token"
}
```

#### Scenario: 有効期限切れの JWT トークンでの認証試行

**GIVEN** 有効期限が切れた JWT トークンを含む Authorization ヘッダー

**WHEN** 保護されたエンドポイントにリクエストを送信する

**THEN** ステータスコード 401 (Unauthorized) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Invalid or expired token"
}
```

#### Scenario: 不正な形式の Authorization ヘッダー

**GIVEN** "Bearer" プレフィックスのない Authorization ヘッダー "invalid_format"

**WHEN** 保護されたエンドポイントにリクエストを送信する

**THEN** ステータスコード 401 (Unauthorized) が返される

**AND** レスポンスボディに以下の形式の JSON が含まれる:
```json
{
  "error": "Invalid authorization header format"
}
```

### Requirement: JWT トークン検証

JWT トークンは厳密に検証されなければなりません (MUST)。

#### Scenario: トークン署名の検証

**GIVEN** JWT トークン

**WHEN** トークンを検証する

**THEN** トークンの署名が秘密鍵を使用して検証される

**AND** 署名が一致しない場合、トークンが拒否される

#### Scenario: トークンペイロードの検証

**GIVEN** JWT トークン

**WHEN** トークンを検証する

**THEN** トークンのペイロードに `sub` (ユーザー ID) が含まれる

**AND** トークンのペイロードに `exp` (有効期限) が含まれる

**AND** 有効期限が現在時刻より後である

#### Scenario: トークンアルゴリズムの検証

**GIVEN** JWT トークン

**WHEN** トークンを検証する

**THEN** トークンが HS256 アルゴリズムで署名されていることを確認する

**AND** 他のアルゴリズム（特に "none"）を許可しない

### Requirement: ユーザーコンテキスト

認証されたユーザー情報をリクエストコンテキストに追加しなければなりません (MUST)。

#### Scenario: ユーザー情報の抽出

**GIVEN** 検証済みの JWT トークン

**WHEN** トークンからユーザー情報を抽出する

**THEN** ユーザー ID がリクエストコンテキストに追加される

**AND** メールアドレスがリクエストコンテキストに追加される

**AND** 後続のハンドラーでユーザー情報にアクセスできる

### Requirement: 環境変数

JWT の秘密鍵は環境変数から読み込まれなければなりません (MUST)。

#### Scenario: JWT 秘密鍵の読み込み

**GIVEN** 環境変数 `JWT_SECRET`

**THEN** サーバー起動時に `JWT_SECRET` が読み込まれる

**AND** `JWT_SECRET` が設定されていない場合、サーバーの起動が失敗する

**AND** エラーメッセージ "JWT_SECRET environment variable is required" が表示される

#### Scenario: JWT 秘密鍵の長さ

**GIVEN** 環境変数 `JWT_SECRET`

**THEN** `JWT_SECRET` が最低 32 文字以上である

**AND** 32 文字未満の場合、サーバーの起動が失敗する
