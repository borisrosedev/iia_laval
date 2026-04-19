#!/bin/bash

curl -v http://localhost:3000

curl -v -X POST -d '{"email": "demo@local.dev", "password": "secret123"}' -H "Content-Type: application/json" http://localhost:3000/api/v1/users
