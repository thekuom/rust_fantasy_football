# Rust Fantasy Football

## Overview
This is a sample project to explore building web APIs in Rust using a microservice
architecture.

## Services
API Gateway - The API Gateway is the publicly accessible API that provides an interface to
the rest of the microservices.

Players API - The Players API provides information about players and teams. This is a
read-heavy service. The idea is that the data in this service is fetched often and
does not change frequently. There is also not much data to store here so we can choose
a database instance that is faster but does not need to support having tons of data. We
may also decide to leverage a large amount of caching.

Auth Service - The Auth Service handles authentication. It will communicate with a user service
to log in a user and returns a JWT and refresh token to the API Gateway, which can forward the
tokens to the client.

## About Auth
The auth approach for this project is the following:
- The Auth Service exposes `POST /access_token` and `DELETE /refresh_token` routes to the API Gateway
- `POST /access_token` will return a refresh token and an access token (JWT) for the user to use to make API calls
- `DELETE /refresh_token` will revoke the refresh token and invalidate JWTs by that user. The API Gateway will enforce
  invalidating the JWTs by the following approach:
  1. User calls `DELETE /refresh_token` with JWT
  2. API Gateway reads when the expiration on the JWT and the issue date and calculates the difference.
  3. API Gateway will reject any JWT from that client that expires between now and now + [expiration time difference]
- The API Gateway will validate that the JWT is valid with a public key provided by the auth service
- The JWT has the following information as claims:
  - user ID
  - permissions
- Permissions and roles are stored in the Auth Service storage

## Tech Stack
### API Gateway
- [juniper][juniper] - GraphQL Implementation
- [actix-web][actix-web] - Web Framework

### Players API
- [actix-web][actix-web] - Web Framework
- [diesel][diesel] - ORM/Querybuilder
- [postgres][postgres] - RDMS

[juniper]: https://github.com/graphql-rust/juniper
[actix-web]: https://github.com/actix/actix-web
[diesel]: https://github.com/diesel-rs/diesel
[postgres]: https://www.postgresql.org/
