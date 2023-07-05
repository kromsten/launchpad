# starsd config node $NODE
# starsd config chain-id $CHAIN_ID
# starsd config output json

KEY=$(starsd keys show $ADMIN | jq -r .name)
FACTORY_CODE_ID=2605
MINTER_CODE_ID=2606

MSG=$(cat <<EOF
{
  "params": {
    "code_id": $MINTER_CODE_ID,
    "allowed_sg721_code_ids": [2604],
    "frozen": false,
    "creation_fee": {"amount": "1000000000", "denom": "ustars"},
    "min_mint_price": {"amount": "0", "denom": "ustars"},
    "mint_fee_bps": 1000,
    "max_trading_offset_secs": 604800,
    "extension": {
        "max_token_limit": 10000,
        "max_per_address_limit": 50,
        "airdrop_mint_price": { "denom": "ustars", "amount": "0" },
        "airdrop_mint_fee_bps": 10000,
        "shuffle_fee": { "amount": "500000000", "denom": "ustars" }
    }
  }
}
EOF
)
echo $MSG


starsd tx wasm instantiate $FACTORY_CODE_ID "$MSG" --label "Factory" \
  --admin $ADMIN --gas-prices 0.025ustars --gas 500000 --gas-adjustment 1.9 \
  --from $KEY -y -b block -o json | jq .
