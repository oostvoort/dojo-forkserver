#!/bin/bash
set -euo pipefail
pushd $(dirname "$0")/..

# make sure to include this
# this allows to have named variables for $WORLD_ADDRESS, $PRIVATE_KEY, $ACCOUNT_ADDRESS, $RPC_URL
# systems are also named variables (their name being the name of the system)
# -> e.g. $move_system points to its contract address
for arg in "$@"
do
    eval "${arg}"
done

# enable system -> component authorizations
COMPONENTS=("Position" "Moves" )

for component in ${COMPONENTS[@]}; do
    sozo auth writer $component $player_actions --world $WORLD_ADDRESS --private-key $PRIVATE_KEY --account-address $ACCOUNT_ADDRESS --rpc-url $RPC_URL
done

for component in ${COMPONENTS[@]}; do
    sozo auth writer $component $player_actions --world $WORLD_ADDRESS --private-key $PRIVATE_KEY --account-address $ACCOUNT_ADDRESS --rpc-url $RPC_URL
done

echo "Default authorizations have been successfully set."