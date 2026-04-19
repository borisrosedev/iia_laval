#!/bin/bash

create_node_api_boilerplate() {
    cd SOcket

    npm init -y
    npm install -D cors express dotenv
    touch server.js
    npm pkg set scripts.dev="node --watch server.js"
}

create_node_api_boilerplate