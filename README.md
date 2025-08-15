# URL Shortener Service

A full-stack URL shortener service built with:
- **Backend**: Rust + Axum framework
- **Frontend**: SvelteKit (Svelte)
- **Database**: PostgreSQL

## Project Structure

```
.
├── backend/               # Rust backend code
│   ├── src/               # Source code
│   │   ├── config/        # Configuration management
│   │   ├── database/      # Database connection handling
│   │   ├── error.rs       # Error handling
│   │   ├── models/        # Data models
│   │   ├── routes/        # API routes
│   │   ├── services/      # Business logic
│   │   └── main.rs        # Application entry point
│   ├── Cargo.toml         # Rust dependencies
│   └── Dockerfile         # Backend container configuration
├── frontend/              # SvelteKit frontend
│   ├── src/               # Source code
│   │   └── routes/        # Frontend routes
│   ├── package.json       # Node.js dependencies
│   └── Dockerfile         # Frontend container configuration
├── docker-compose.yml     # Docker Compose configuration
└── README.md              # Project documentation
```

## Features

- Create shortened URLs
- Redirect from short URLs to original URLs

## Running with Docker Compose

1. Clone the repository
2. Run the following command:

   ```bash
   docker-compose up -d
   ```

This will start:
- PostgreSQL database on port 5432
- Backend API on port 3000
- Frontend on port 8080

## Development Setup

### Backend (Rust)

1. Install Rust: https://www.rust-lang.org/tools/install
2. Navigate to the backend directory:
   ```bash
   cd backend
   ```
3. Create a `.env` file with the following variables:
   ```
   DATABASE_URL=postgres://postgres:postgres@localhost/urlshortener
   SERVER_ADDRESS=127.0.0.1:3000
   BASE_URL=http://localhost:3000
   ```
4. Run the development server:
   ```bash
   cargo run
   ```

### Frontend (SvelteKit)

1. Install Node.js: https://nodejs.org/
2. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```
3. Install dependencies:
   ```bash
   npm install
   ```
4. Run the development server:
   ```bash
   npm run dev
   ```

## API Endpoints

- `POST /api/shorten` - Create a shortened URL
- `GET /api/urls` - List all URLs
- `GET /:id` - Redirect to the original URL

## License

MIT