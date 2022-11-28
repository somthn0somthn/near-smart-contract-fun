#!/bin/bash
START=0
END=$3
set -e
for (( c=$START; c<=$END; c++ ))
do
  near call $1 nft_mint '{"token_id": "'$c'", "receiver_id": "'$2'"}' --accountId $1 --deposit 0.01
  echo "minted nft"
done
