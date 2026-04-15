#!/bin/bash

curl -v http://localhost:3000

curl -v -X POST -d '{"email": "a@a.aa", "password": "aaaaaa"}' -H "Content-Type: application/json"  http://localhost:3000/api/v1/users