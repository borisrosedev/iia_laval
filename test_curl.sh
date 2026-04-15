#!/bin/sh

# GET EXAMPLE 
# curl -X GET http://localhost:3001
# sleep 5
# POST EXAMPLE 
curl -H "Content-Type:application/json" \
    -d '{"email":"ethan@gmail.com","password":"1234"}' \
    -X POST http://localhost:3001/api/v1/users 

