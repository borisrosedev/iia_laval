#bin bash
set -euo pipefail

create_boilerplate () {
    mkdir -p topology-"$1"/switches/core/interface
    mkdir -p topology-"$1"/switches/access/marketing/interfaces
    mkdir -p topology-"$1"/switches/acess/rh/interfaces
    mkdir -p topology-"$1"/switches/acess/direction/interfaces
    mkdir -p topology-"$1"/routers/routeur-1
    touch topology-"$1"/switches/core/interface/access.config
    touch topology-"$1"/switches/core/interface/core.config
    touch topology-"$1"/switches/vlans.config
    touch topology-"$1"/switches/access/marketing/interfaces/host.config
    touch topology-"$1"/switches/access/marketing/interfaces/core.config
    touch topology-"$1"/switches/access/rh/interfaces/core.config
    touch topology-"$1"/switches/access/rh/interfaces/host.config
    touch topology-"$1"/switches/access/direction/interfaces/core.config
    touch topology-"$1"/switches/access/direction/interfaces/host.config
    
}

create_boilerplate("$1");