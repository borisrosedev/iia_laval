#!/bin/bash 

set -euo pipefail 

create_boilerplate () {
    mkdir -p topology-"$1"/switches/core/interfaces
    mkdir -p topology-"$1"/switches/access/marketing/interfaces
    mkdir -p topology-"$1"/switches/access/rh/interfaces 
    mkdir -p topology-"$1"/switches/access/direction/interfaces
    mkdir -p topology-"$1"/routers/router-1
    touch topology-"$1"/switches/vlans.config
    touch topology-"$1"/switches/core/interfaces/access.config
    touch topology-"$1"/switches/core/interfaces/router.config
    touch topology-"$1"/switches/access/marketing/interfaces/hosts.config
    touch topology-"$1"/switches/access/marketing/interfaces/core.config
    touch topology-"$1"/switches/access/rh/interfaces/hosts.config
    touch topology-"$1"/switches/access/rh/interfaces/core.config
    touch topology-"$1"/switches/access/direction/interfaces/hosts.config
    touch topology-"$1"/switches/access/direction/interfaces/core.config
}

create_boilerplate "$1"