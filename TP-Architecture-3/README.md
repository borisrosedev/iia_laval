# TP Architecture 3 Tiers

Ce projet contient une application complète répondant aux exigences du TP:

- frontend React
- backend Express
- base PostgreSQL
- ORM Prisma
- CRUD complet sur `users`
- authentification JWT avec expiration
- mots de passe hachés avec `bcryptjs`
- rate limiting avec `express-rate-limit`
- redondance applicative via deux instances backend derrière Nginx

## Lancement

```bash
docker compose up --build
```

Applications:

- Frontend: `http://localhost:3000`
- API: `http://localhost:8080`
- PostgreSQL: `localhost:5432`

Compte initial:

- email: `admin@tp.local`
- mot de passe: `admin123`

## Endpoints principaux

- `POST /auth/register`
- `POST /auth/login`
- `GET /users`
- `GET /users/:id`
- `POST /users`
- `PUT /users/:id`
- `DELETE /users/:id`
