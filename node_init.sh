#!/bin/bash

create_node_boilerplate() {
    cd node_api
    #initialise un package json
    npm init -y
    npm install -D cors express dotenv
    touch server.js
    npm pkg set scripts.dev="node --watch server.js"
    touch .env

}

create_node_boilerplate