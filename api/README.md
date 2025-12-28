# Task App API

A RESTful API for task management with user authentication using JWT tokens.

## Features

- ✅ User Registration
- ✅ User Login with JWT authentication
- ✅ Protected endpoints with middleware
- ✅ Password hashing with Argon2
- ✅ PostgreSQL database with sqlx
- ✅ Input validation

## Prerequisites

- Rust 1.70+
- PostgreSQL 13+
- Docker (for running PostgreSQL)

## Setup

### 1. Database Setup

Start the PostgreSQL database:

```bash
docker compose up -d
```

### 2. Environment Variables

Copy the example environment file:

```bash
cp .env.example .env
```

Update `.env` with your configuration:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/taskapp
JWT_SECRET=your-secret-key-here-min-32-chars
```

### 3. Run Migrations

Install sqlx-cli:

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

Create database and run migrations:

```bash
cd api/lib
sqlx database create
sqlx migrate run
```

### 4. Run the Server

```bash
cargo run
```

The server will start on `http://localhost:3000`

## API Endpoints

### Public Endpoints

#### Register a New User

```http
POST /api/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response** (201 Created):

```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "created_at": "2024-01-01T12:00:00Z"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Error Responses**:
- `400` - Invalid email or password too short
- `409` - Email already exists

#### Login

```http
POST /api/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123"
}
```

**Response** (200 OK):

```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "created_at": "2024-01-01T12:00:00Z"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Error Responses**:
- `400` - Invalid email format or password too short
- `401` - Invalid email or password

### Protected Endpoints

All protected endpoints require a JWT token in the Authorization header:

```http
Authorization: Bearer <token>
```

#### Get Current User

```http
GET /api/me
Authorization: Bearer <token>
```

**Response** (200 OK):

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "created_at": "2024-01-01T12:00:00Z"
}
```

**Error Responses**:
- `401` - Missing or invalid token

## Testing with cURL

### Register a user

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H 'Content-Type: application/json' \
  -d '{"email":"test@example.com","password":"password123"}'
```

### Login

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"test@example.com","password":"password123"}'
```

### Access protected endpoint

```bash
TOKEN="your-jwt-token-here"
curl http://localhost:3000/api/me \
  -H "Authorization: Bearer $TOKEN"
```

## Running Tests

Run all tests:

```bash
cargo test
```

Run tests sequentially (for environment-dependent tests):

```bash
cargo test -- --test-threads=1
```

## Project Structure

```
api/lib/
├── src/
│   ├── db/              # Database connection pool
│   ├── error.rs         # Error handling and responses
│   ├── handlers/        # HTTP request handlers
│   │   ├── auth.rs      # Authentication endpoints
│   │   └── user.rs      # User endpoints
│   ├── middleware/      # Custom middleware
│   │   └── auth.rs      # JWT authentication middleware
│   ├── models/          # Data models
│   │   └── user.rs      # User model and DTOs
│   ├── repositories/    # Database queries
│   │   └── user_repository.rs
│   ├── utils/           # Utility functions
│   │   ├── jwt.rs       # JWT token generation/verification
│   │   └── password.rs  # Password hashing/verification
│   └── main.rs          # Application entry point
├── migrations/          # Database migrations
│   ├── 20231228000001_create_users_table.sql
│   └── 20231228000001_create_users_table.down.sql
├── Cargo.toml
└── README.md
```

## Security Considerations

- Passwords are hashed using Argon2, a memory-hard password hashing algorithm
- JWT tokens expire after 24 hours
- All database queries use prepared statements to prevent SQL injection
- Input validation is performed on all user inputs
- HTTPS should be used in production environments

## Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://user:pass@localhost:5432/dbname` |
| `JWT_SECRET` | Secret key for JWT signing (min 32 chars) | `supersecretkey123...` |

## License

MIT
