#!/bin/bash

set -euo pipefail

create_boilerplate () {
    mkdir topology-"$1"/switches
    mkdir topology-"$1"/routers
}