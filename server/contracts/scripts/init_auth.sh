#!/bin/bash
set -euo pipefail
pushd $(dirname "$0")/..

export WORLD_ADDRESS=$1;
export PRIVATE_KEY=$2;
export ACCOUNT_ADDRESS=$3;

# enable system -> component authorizations
COMPONENTS=("Position" "Moves" )

for component in ${COMPONENTS[@]}; do
    sozo auth writer $component spawn --world $WORLD_ADDRESS --private-key $PRIVATE_KEY --account-address $ACCOUNT_ADDRESS
done

for component in ${COMPONENTS[@]}; do
    sozo auth writer $component move --world $WORLD_ADDRESS --private-key $PRIVATE_KEY --account-address $ACCOUNT_ADDRESS
done

echo "Default authorizations have been successfully set."