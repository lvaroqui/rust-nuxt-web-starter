#!/bin/bash

# run command with exec to pass control
echo "Running CMD: $@"
exec /opt/scripts/wait-for-it.sh db:5432 --timeout=0 -- "$@"
