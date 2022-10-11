/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.16.5.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export type Uint128 = string;
export type Timestamp = Uint64;
export type Uint64 = string;
export interface ConfigResponse {
  admin: string;
  base_token_uri: string;
  factory: string;
  mint_price: Coin;
  num_tokens: number;
  per_address_limit: number;
  sg721_address: string;
  sg721_code_id: number;
  start_time: Timestamp;
  whitelist?: string | null;
  [k: string]: unknown;
}
export interface Coin {
  amount: Uint128;
  denom: string;
  [k: string]: unknown;
}
export type ExecuteMsg = {
  mint: {
    [k: string]: unknown;
  };
} | {
  set_whitelist: {
    whitelist: string;
    [k: string]: unknown;
  };
} | {
  purge: {
    [k: string]: unknown;
  };
} | {
  update_mint_price: {
    price: number;
    [k: string]: unknown;
  };
} | {
  update_start_time: Timestamp;
} | {
  update_trading_start_time: Timestamp | null;
} | {
  update_per_address_limit: {
    per_address_limit: number;
    [k: string]: unknown;
  };
} | {
  mint_to: {
    recipient: string;
    [k: string]: unknown;
  };
} | {
  mint_for: {
    recipient: string;
    token_id: number;
    [k: string]: unknown;
  };
} | {
  shuffle: {
    [k: string]: unknown;
  };
} | {
  burn_remaining: {
    [k: string]: unknown;
  };
};
export type Decimal = string;
export interface InstantiateMsg {
  create_msg: CreateMinterMsgForVendingMinterInitMsgExtension;
  params: MinterParamsForParamsExtension;
  [k: string]: unknown;
}
export interface CreateMinterMsgForVendingMinterInitMsgExtension {
  collection_params: CollectionParams;
  init_msg: VendingMinterInitMsgExtension;
  [k: string]: unknown;
}
export interface CollectionParams {
  code_id: number;
  info: CollectionInfoForRoyaltyInfoResponse;
  name: string;
  symbol: string;
  [k: string]: unknown;
}
export interface CollectionInfoForRoyaltyInfoResponse {
  creator: string;
  description: string;
  explicit_content: boolean;
  external_link?: string | null;
  image: string;
  royalty_info?: RoyaltyInfoResponse | null;
  trading_start_time?: Timestamp | null;
}
export interface RoyaltyInfoResponse {
  payment_address: string;
  share: Decimal;
}
export interface VendingMinterInitMsgExtension {
  base_token_uri: string;
  mint_price: Coin;
  num_tokens: number;
  payment_address?: string | null;
  per_address_limit: number;
  start_time: Timestamp;
  whitelist?: string | null;
  [k: string]: unknown;
}
export interface MinterParamsForParamsExtension {
  code_id: number;
  creation_fee: Coin;
  extension: ParamsExtension;
  max_trading_offset_secs: number;
  min_mint_price: Coin;
  mint_fee_bps: number;
  [k: string]: unknown;
}
export interface ParamsExtension {
  airdrop_mint_fee_bps: number;
  airdrop_mint_price: Coin;
  max_per_address_limit: number;
  max_token_limit: number;
  shuffle_fee: Coin;
  [k: string]: unknown;
}
export interface MintCountResponse {
  address: string;
  count: number;
  [k: string]: unknown;
}
export interface MintPriceResponse {
  airdrop_price: Coin;
  current_price: Coin;
  public_price: Coin;
  whitelist_price?: Coin | null;
  [k: string]: unknown;
}
export interface MintableNumTokensResponse {
  count: number;
  [k: string]: unknown;
}
export type Addr = string;
export interface MinterConfigForConfigExtension {
  collection_code_id: number;
  extension: ConfigExtension;
  factory: Addr;
  mint_price: Coin;
  [k: string]: unknown;
}
export interface ConfigExtension {
  admin: Addr;
  base_token_uri: string;
  num_tokens: number;
  payment_address?: Addr | null;
  per_address_limit: number;
  start_time: Timestamp;
  whitelist?: Addr | null;
  [k: string]: unknown;
}
export type QueryMsg = {
  config: {
    [k: string]: unknown;
  };
} | {
  mintable_num_tokens: {
    [k: string]: unknown;
  };
} | {
  start_time: {
    [k: string]: unknown;
  };
} | {
  mint_price: {
    [k: string]: unknown;
  };
} | {
  mint_count: {
    address: string;
    [k: string]: unknown;
  };
} | {
  status: {
    [k: string]: unknown;
  };
};
export interface StartTimeResponse {
  start_time: string;
  [k: string]: unknown;
}
export interface StatusResponse {
  status: Status;
  [k: string]: unknown;
}
export interface Status {
  is_blocked: boolean;
  is_explicit: boolean;
  is_verified: boolean;
  [k: string]: unknown;
}