# URL Shortener

This is a simple URL shortener that I am building to learn Rust and Svelte. Also some ops and testing things. So, no url analytics, custom short codes, expiration or authentication.

It has only two api's:
POST: /shorten -> shortens a given url
GET: /{short_code} -> redirects the short code to original url

Here's the tech stack I am using:

## Backend:

- Rust with Axum
- SQLx with postgres
- Serde for serialization / deserialization
- Valkey for cache
- base62 encoding
- Rate limiting in rust
- Short code collision handling

## Frontend:

- Typescript with SvelteKit and Svelte 5 as both frontend and a backend for frontend
- Shadcn-svelte for components
- zod / superforms / formsnap for forms
- Vite for build tool

## Testing:

- Unit test with vitest
- Component test with storybook
- Playwright for end-to-end test

## Ops:

- Podman for containerization
- nginx for reverse proxy
- Deploy in aws
- SSL / TLS using Let’s Encrypt
- Rate limiting in nginx

## Miscellaneous

- Logs
- Tracing
- Metrics
- Healthcheck
