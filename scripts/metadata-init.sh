FACTORY_CODE_ID=4628
MINTER_CODE_ID=4632
SG721_CODE_ID=4631

MSG=$(cat <<EOF
{
  "params": {
    "code_id": $MINTER_CODE_ID,
    "allowed_sg721_code_ids": [$SG721_CODE_ID],
    "frozen": false,
    "creation_fee": { "amount": "1000000000", "denom": "ustars" },
    "min_mint_price": { "amount": "50000000", "denom": "ustars" },
    "mint_fee_bps": 1000,
    "max_trading_offset_secs": 1209600,
    "extension": {
      "max_token_limit": 10000,
      "max_per_address_limit": 50,
      "airdrop_mint_price": { "amount": "0", "denom": "ustars" },
      "airdrop_mint_fee_bps": 0
    }
  }
}

EOF
)




starsd tx wasm instantiate $FACTORY_CODE_ID  "$MSG"  --label "on-chain metadata factory" --no-admin \
  --from mainnet-spot --gas-prices 0.025ustars --gas-adjustment 1.7 --gas auto \
  --chain-id elgafar-1 --node https://rpc.elgafar-1.stargaze-apis.com:443 \
  -b block -o json | jq .