#!/bin/sh

# GET EXAMPLE 
curl -X GET http://localhost:3000

# POST EXAMPLE 
curl -H "Content-Type:application/json" \
    -d '{"email":"ethan@gmail.com","password":"1234"}' \
    -X http://localhost:3000/api/v1/users 

