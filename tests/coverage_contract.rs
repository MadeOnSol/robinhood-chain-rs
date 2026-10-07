use robinhood_chain::types::*;
use serde_json::json;

#[test]
fn holders_preserve_partial_history_and_older_responses() {
    let mut row = json!({"chain":"rhc", "token_address":"0x1", "verified":true,
        "holders":[], "count":0, "has_more":false,
        "history_coverage":{"state":"partial_history", "history":"tracked_only", "reason":"reentry_gap",
        "fold_from_block":100, "compared_at_block":200, "verified_at_block":null}});
    let holders: HoldersResponse = serde_json::from_value(row.clone()).unwrap();
    let coverage = holders.history_coverage.unwrap();
    assert_eq!(coverage.state, "partial_history"); assert_eq!(coverage.verified_at_block, None);
    row.as_object_mut().unwrap().remove("history_coverage");
    assert!(serde_json::from_value::<HoldersResponse>(row).unwrap().history_coverage.is_none());
}

#[test]
fn trade_pair_attribution_and_wallet_visibility_are_not_discarded() {
    let trade: RhcTrade = serde_json::from_value(json!({"block_number":1, "block_time":"t",
        "tx_hash":"0xS", "log_index":0, "dex":"uniswap", "pool":"0xP",
        "pair_status":"unsupported_pair", "token_address":null, "action":null})).unwrap();
    assert_eq!(trade.pair_status.as_deref(), Some("unsupported_pair")); assert!(trade.token_address.is_none());
    let funding: WalletFundingResponse = serde_json::from_value(json!({"chain":"rhc", "chain_id":"eip155:4663",
        "native_asset":"ETH", "address":"0xW", "status":"partial_coverage", "summary":"n", "shared_funders":[],
        "pagination":{"limit":20,"offset":0,"total":0,"has_more":false}, "coverage":{}, "disclaimer":"n",
        "wallet_coverage":{"state":"internal_transfers_not_visible", "currently_tracked":true,
        "ever_tracked":true,"address_kind":"contract", "limitations":["internal_eth_transfers_not_visible"]}})).unwrap();
    assert_eq!(funding.wallet_coverage.unwrap().address_kind.as_deref(), Some("contract"));
}
