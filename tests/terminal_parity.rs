//! 0.13.0 contract parity: the five new operations (query wire format +
//! response bodies) and the new fields on existing responses. Bodies are
//! shaped after the OpenAPI response schemas.

use robinhood_chain::types::*;
use serde_json::json;

#[test]
fn deployer_activity_query_and_body() {
    let q = serde_json::to_value(DeployerActivityParams {
        limit: Some(50),
        types: Some("launch,dev_sell".into()),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(q, json!({"limit": 50, "types": "launch,dev_sell"}));

    let r: DeployerActivityResponse = serde_json::from_value(json!({
        "chain": "robinhood", "address": "0xd",
        "events": [
            {"id": "l:0xt", "type": "launch", "at": "2026-10-15T00:00:00Z", "block_number": null,
             "token_address": "0xt", "token_symbol": "T", "attribution": "creator",
             "token_name": "Tok", "launchpad": "noxa"},
            {"id": "t:9", "type": "dev_sell", "at": "2026-10-15T01:00:00Z", "block_number": 7,
             "token_address": "0xt", "token_symbol": "T", "attribution": "pool_creator",
             "event_id": "dx1:0xh:0xt:dev_sell:0xd", "tx_hash": "0xh", "log_index": 3, "dex": "uniswap-v2",
             "eth_amount": 0.5, "token_amount_raw": "123456789012345678901234567890", "price_usd": null}
        ],
        "count": 2, "summary": {"launches": 1, "dev_buys": 0, "dev_sells": 1},
        "next_cursor": null, "has_more": false,
        "window": {"since": "2026-09-15T00:00:00Z", "max_days": 90},
        "coverage": {"tokens_attributed": 1, "tokens_truncated": false, "dev_trades_complete_from": null,
            "archive_required_before": null, "trades_scanned": 10, "scan_truncated": false,
            "transfers_included": false, "linked_wallets_included": false},
        "note": "n"
    }))
    .unwrap();
    assert_eq!(r.events[0].event_type, "launch");
    assert_eq!(
        r.events[1].event_id.as_deref(),
        Some("dx1:0xh:0xt:dev_sell:0xd")
    );
    assert_eq!(
        r.events[1].token_amount_raw.as_deref(),
        Some("123456789012345678901234567890")
    );
    let cov = r.coverage.unwrap();
    assert_eq!(cov.transfers_included, Some(false));
    assert!(cov.dev_trades_complete_from.is_none());
}

#[test]
fn scouts_leaderboard_query_and_body() {
    let q = serde_json::to_value(ScoutLeaderboardParams {
        scout_tier: Some(ScoutTier::A),
        sort: Some(ScoutSort::NFirstTouches30d),
        min_first_touches: Some(40),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        q,
        json!({"scout_tier": "A", "sort": "n_first_touches_30d", "min_first_touches": 40})
    );
    assert_eq!(
        serde_json::to_value(ScoutSort::ScoutScore).unwrap(),
        json!("scout_score")
    );
    assert_eq!(
        serde_json::to_value(ScoutSort::Swarm3plusPct).unwrap(),
        json!("swarm_3plus_pct")
    );

    let r: ScoutLeaderboardResponse = serde_json::from_value(json!({
        "chain": "robinhood", "as_of": "2026-10-15T00:00:00Z",
        "scouts": [{"kol_id": "11111111-1111-1111-1111-111111111111", "name": "k", "twitter_url": null,
            "avatar_url": null, "evm_addresses": ["0xa", "0xb"], "as_of": "2026-10-15T00:00:00Z",
            "first_touches_30d": 31, "avg_followers_4h": 1.5, "swarm_3plus_pct": 41.9,
            "swarm_5plus_pct": null, "scout_tier": "S"}],
        "count": 1, "min_first_touches": 30,
        "method": {"tier_min_first_touches": 30, "note": "n"}
    }))
    .unwrap();
    assert_eq!(r.scouts[0].evm_addresses.len(), 2);
    assert_eq!(r.scouts[0].scout_tier.as_deref(), Some("S"));
    assert!(r.scouts[0].swarm_5plus_pct.is_none());
}

#[test]
fn bundle_share_query_and_body() {
    let q = serde_json::to_value(BundleShareParams {
        window: Some(BundleWindow::H6),
        min_wallets: Some(4),
    })
    .unwrap();
    assert_eq!(q, json!({"window": "6h", "min_wallets": 4}));

    let r: BundleShareResponse = serde_json::from_value(json!({
        "chain": "robinhood", "token_address": "0xt", "window": "24h", "min_wallets": 3,
        "bundled_pct_of_supply_now": null, "scope": "launch_and_clusters", "clusters_status": "ok",
        "wallets": {"total": 9, "launch_cohort_wallets": 4, "cluster_wallets": 6, "overlap": 1,
            "clusters_counted": 2, "verified": 8, "holding": 5, "max": 500},
        "held_now": {"source": "rhc_node_balanceOf_latest", "checked_at": "2026-10-15T00:00:00Z",
            "wallets": 9, "verified_wallets": 8, "holding_wallets": 5, "complete": false,
            "incomplete_reason": "call_failed", "tokens_held_raw": null, "tokens_held": null,
            "held_pct_of_supply": null, "supply_basis": null},
        "as_of": "2026-10-15T00:00:00Z", "note": "n"
    }))
    .unwrap();
    // Null is "not every wallet was read", never 0.
    assert!(r.bundled_pct_of_supply_now.is_none());
    let held = r.held_now.unwrap();
    assert_eq!(held.complete, Some(false));
    assert!(held.tokens_held_raw.is_none());
    assert_eq!(r.wallets.unwrap().overlap, Some(1));
}

#[test]
fn bundle_clusters_query_and_body() {
    let q = serde_json::to_value(BundleClustersParams {
        window: Some(BundleWindow::H1),
        limit: Some(10),
        include_balances: Some(true),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        q,
        json!({"window": "1h", "limit": 10, "include_balances": true})
    );

    let r: BundleClustersResponse = serde_json::from_value(json!({
        "chain": "robinhood", "token_address": "0xt", "window": "24h", "min_wallets": 3,
        "launch_at": "2026-10-15T00:00:00Z",
        "summary": {"cluster_count": 1, "launch_clusters": 0, "later_clusters": 1,
            "latest_cluster_at": "2026-10-15T02:00:00Z", "largest_cluster_wallets": 3,
            "baseline": {"active_units": 4}},
        "clusters": [{"block_number": 88000000, "same_tx_groups": 1, "eth_volume": 1.2,
            "token_volume_raw": "1000000000000000000000000", "at": "2026-10-15T02:00:00Z",
            "minutes_after_launch": 120.0, "phase": "later", "wallet_count": 3, "new_wallets": 3,
            "wallets": ["0x1", "0x2", "0x3"],
            "held_now": {"complete": true, "tokens_held_raw": "5", "tokens_held": "0.000000000000000005",
                "held_pct_of_supply": 0.01, "supply_basis": "total_supply_raw"},
            "exit": {"cluster_id": 7, "followed_until": "2026-10-22T02:00:00Z", "wallets_tracked": 3,
                "wallets_sold": 1, "wallets_exited": 1, "bought_tokens_raw": null, "sold_tokens_raw": "9",
                "sold_pct": null, "realized_eth": 0.1, "last_sell_at": null}}],
        "held_now_clusters": 1, "exit_tracking": "ok", "exit_note": "n", "note": "n"
    }))
    .unwrap();
    let c = &r.clusters[0];
    assert_eq!(
        c.token_volume_raw.as_deref(),
        Some("1000000000000000000000000")
    );
    assert_eq!(
        c.held_now.as_ref().unwrap().tokens_held_raw.as_deref(),
        Some("5")
    );
    assert_eq!(
        c.exit.as_ref().unwrap().sold_tokens_raw.as_deref(),
        Some("9")
    );
    assert_eq!(r.held_now_clusters, Some(1));
}

#[test]
fn token_kol_trades_query_and_body() {
    let q = serde_json::to_value(TokenKolTradesParams {
        action: Some(TradeAction::Sell),
        include_balances: Some(true),
        cursor: Some("c1".into()),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        q,
        json!({"action": "sell", "cursor": "c1", "include_balances": true})
    );

    let r: TokenKolTradesResponse = serde_json::from_value(json!({
        "chain": "robinhood", "token_address": "0xt",
        "trades": [{"id": "1", "kol_name": "k", "kol_twitter": null, "evm_address": "0xk",
            "action": "buy", "eth_amount": 0.2, "token_amount": 1000.0, "price_usd_at_trade": null,
            "market_cap_usd_at_trade": 50000.0, "dex": "uniswap-v4", "tx_hash": "0xh",
            "log_index": null, "block_number": 1, "traded_at": "2026-10-15T00:00:00Z"}],
        "count": 1, "kols_on_page": 1, "next_cursor": null, "has_more": false, "note": "n",
        "balances": {"source": "rhc_node_balanceOf_latest", "checked_at": "2026-10-15T00:00:00Z",
            "by_wallet": {
                "0xk": {"status": "verified", "balance_raw": "1000000000000000000000", "decimals": 18,
                    "balance": "1000", "holding": true, "reason": null},
                "0xj": {"status": "unverified", "balance_raw": null, "decimals": null,
                    "balance": null, "holding": null, "reason": "call_failed"}},
            "summary": {"wallets": 2, "verified": 1, "holding": 1, "zero": 0, "unverified": 1}}
    }))
    .unwrap();
    assert_eq!(r.trades[0].action, TradeAction::Buy);
    let b = r.balances.unwrap();
    assert_eq!(
        b.by_wallet["0xk"].balance_raw.as_deref(),
        Some("1000000000000000000000")
    );
    // An unverified read is never a zero.
    assert!(b.by_wallet["0xj"].balance_raw.is_none());
    assert_eq!(b.by_wallet["0xj"].reason.as_deref(), Some("call_failed"));
}

#[test]
fn kol_winrate_leaderboard_query_and_body() {
    let q = serde_json::to_value(KolWinrateLeaderboardParams {
        sort: KolWinrateSort::Winrate30d,
        min_closed: Some(5),
        ..Default::default()
    })
    .unwrap();
    assert_eq!(q, json!({"sort": "winrate_30d", "min_closed": 5}));
    assert_eq!(
        serde_json::to_value(KolWinrateLeaderboardParams::default()).unwrap(),
        json!({"sort": "winrate_7d"})
    );

    let r: KolWinrateLeaderboardResponse = serde_json::from_value(json!({
        "chain": "robinhood", "sort": "winrate_7d", "min_closed": 3,
        "leaderboard": [{"evm_address": "0xk", "kol_name": null, "kol_twitter": null,
            "status": "available", "winrate_7d": 66.7, "winrate_30d": null,
            "closed_positions_7d": 3, "closed_positions_30d": 9, "pnl_7d_eth": 0.4, "pnl_30d_eth": -0.1,
            "median_hold_minutes_30d": 42.0, "auto_strategy_tag": "day_trader",
            "data_fingerprint": "fp1", "computed_at": "2026-10-15T00:00:00Z"}],
        "count": 1,
        "scores_meta": {"source": "rhc_kol_scores", "fresh_within_minutes": 120, "definition": "d"}
    }))
    .unwrap();
    let e = &r.leaderboard[0];
    assert_eq!(e.closed_positions_7d, Some(3));
    assert_eq!(e.pnl_30d_eth, Some(-0.1));
    assert_eq!(e.auto_strategy_tag.as_deref(), Some("day_trader"));
    assert_eq!(e.data_fingerprint.as_deref(), Some("fp1"));
    assert_eq!(r.scores_meta.unwrap().fresh_within_minutes, Some(120));
}

#[test]
fn opt_in_query_flags_serialize() {
    assert_eq!(
        serde_json::to_value(BundleParams {
            include_balances: Some(true)
        })
        .unwrap(),
        json!({"include_balances": true})
    );
    assert_eq!(
        serde_json::to_value(BuyerQualityParams::default()).unwrap(),
        json!({})
    );
    assert_eq!(
        serde_json::to_value(EarlyBuyersParams {
            include_scout_scores: Some(true),
            verify_current_balances: Some(true),
            ..Default::default()
        })
        .unwrap(),
        json!({"include_scout_scores": true, "verify_current_balances": true})
    );
    assert_eq!(
        serde_json::to_value(HoldersParams {
            include_wallet_flags: Some(true),
            ..Default::default()
        })
        .unwrap(),
        json!({"include_wallet_flags": true})
    );
    assert_eq!(
        serde_json::to_value(DeployerTokensParams {
            include_launches_24h: Some(true),
            ..Default::default()
        })
        .unwrap(),
        json!({"include_launches_24h": true})
    );
}

#[test]
fn existing_responses_carry_new_fields_and_still_parse_without_them() {
    // Bundle: held_now + balance_now.
    let b: RhcBundleResponse = serde_json::from_value(json!({
        "chain": "robinhood", "token_address": "0xt",
        "bundle": {"wallet_count": 3, "bundle_kind": "same_block", "held_ratio": 0.5,
            "held_pct_of_supply": 0.02, "fully_exited": false, "buy_volume": 10.0, "tokens_held": 5.0,
            "holdings_basis": "swap_only"},
        "held_now": {"source": "rhc_node_balanceOf_latest", "complete": true, "tokens_held_raw": "42"},
        "wallets": [{"rank": 1, "wallet": "0x1", "held_ratio": null, "has_sold": false,
            "balance_now": {"status": "verified", "balance_raw": "42", "decimals": 18,
                "balance": "0.000000000000000042", "holding": true, "reason": null}}]
    }))
    .unwrap();
    assert_eq!(b.held_now.unwrap().tokens_held_raw.as_deref(), Some("42"));
    assert_eq!(
        b.wallets[0].balance_now.as_ref().unwrap().status,
        "verified"
    );
    assert_eq!(b.bundle.holdings_basis.as_deref(), Some("swap_only"));

    // Buyer quality: dump_cluster_held_now.
    let bq: BuyerQualityResponse = serde_json::from_value(json!({
        "chain": "robinhood", "token_address": "0xt",
        "quality": {"score": 50, "confidence": "low", "signal": "neutral",
            "breakdown": {"early_buyers_analyzed": 0, "alpha_wallet_count": 0, "kol_count": 0,
                "bundle_buyer_count": 0, "dump_cluster_count": 1, "recycled_early_buyer_count": 0,
                "bot_dominated": false}},
        "dump_cluster_held_now": {"cohort_size": 20, "members": 1,
            "held_now": {"complete": true}, "wallets": [{"wallet": "0x1", "balance_now": null}], "note": "n"}
    }))
    .unwrap();
    let d = bq.dump_cluster_held_now.unwrap();
    assert_eq!(d.members, Some(1));
    assert!(d.wallets[0].balance_now.is_none());

    // Early buyers: holding_reason, scout_score, recon, verification.
    let eb: EarlyBuyersResponse = serde_json::from_value(json!({
        "chain": "robinhood", "token_address": "0xt", "count": 1,
        "early_buyers": [{"rank": 1, "wallet": "0x1", "position": "open",
            "holding_status": "verified_nonzero", "holding_reason": "pinned_chain_nonzero_balance",
            "is_kol": true,
            "scout_score": {"as_of": "2026-10-01T00:00:00Z", "first_touches_30d": 33,
                "avg_followers_4h": 2.0, "swarm_3plus_pct": 30.0, "swarm_5plus_pct": 10.0, "scout_tier": "A"}}],
        "holder_recon_block": 88000000, "holder_recon_checked_at": "2026-10-15T00:00:00Z",
        "onchain_balance_verification": {"source": "rhc_node_pinned_balanceOf", "block_number": 88000001,
            "block_hash": "0xb", "checked_at": "2026-10-15T00:00:00Z", "verified_wallets": 1, "complete": true},
        "scout_scores_status": "ok", "scout_scores_note": "n"
    }))
    .unwrap();
    assert_eq!(
        eb.early_buyers[0].holding_reason.as_deref(),
        Some("pinned_chain_nonzero_balance")
    );
    assert_eq!(
        eb.early_buyers[0]
            .scout_score
            .as_ref()
            .unwrap()
            .scout_tier
            .as_deref(),
        Some("A")
    );
    assert_eq!(eb.holder_recon_block, Some(88000000));
    assert_eq!(
        eb.onchain_balance_verification.unwrap().complete,
        Some(true)
    );
    assert_eq!(eb.scout_scores_status.as_deref(), Some("ok"));

    // Holders: per-row shares + wallet flags.
    let h: HoldersResponse = serde_json::from_value(json!({
        "chain": "robinhood", "token_address": "0xt", "verified": true,
        "share_basis": {"share": "non-burned supply"},
        "holders": [{"holder": "0x1", "balance": "10", "share": 0.1, "share_of_supply": 0.09,
            "share_of_circulating": null, "is_pool": true,
            "wallet_flags": {"is_sniper": null, "is_bundler": null, "is_kol": false,
                "is_alpha_tracked": false, "likely_bot": null, "is_dumper": false,
                "dump_cluster_as_of": null, "label_coverage": {"is_kol": {"evaluated": true}}}}],
        "count": 1, "has_more": false,
        "wallet_flags_meta": {"status": "partial", "rule_version": "v1", "note": "n"}
    }))
    .unwrap();
    assert_eq!(h.holders[0].share_of_supply, Some(0.09));
    assert!(h.holders[0].share_of_circulating.is_none());
    assert!(h.holders[0]
        .wallet_flags
        .as_ref()
        .unwrap()
        .is_sniper
        .is_none());
    assert_eq!(
        h.wallet_flags_meta.unwrap().status.as_deref(),
        Some("partial")
    );
    assert!(h.share_basis.is_some());

    // Top traders: fifo.
    let tt: TopTradersResponse = serde_json::from_value(json!({
        "chain": "robinhood", "token_address": "0xt", "count": 1, "limit": 50, "offset": 0,
        "has_more": false, "cost_method": "fifo", "fifo_status": "ok", "fifo_note": "n",
        "traders": [{"trader_eoa": "0x1",
            "fifo": {"realized_pnl_eth": 0.3, "roi": null, "cost_basis_matched_eth": 0.0,
                "bought_eth": 1.0, "sold_eth": 1.3, "unmatched_sell_tokens": 5.0,
                "cost_basis_complete": false, "fifo_open_tokens": 0.0, "open_cost_basis_eth": 0.0,
                "sized_trades": 2, "unpriced_trades": 0, "unsized_trades": 0}}]
    }))
    .unwrap();
    assert_eq!(tt.fifo_status.as_deref(), Some("ok"));
    assert_eq!(tt.cost_method.as_deref(), Some("fifo"));
    assert_eq!(
        tt.traders[0].fifo.as_ref().unwrap().cost_basis_complete,
        Some(false)
    );

    // Deployer tokens: launches_24h (count None unless observed).
    let dt: DeployerTokensResponse = serde_json::from_value(json!({
        "chain": "robinhood", "is_deployer": true, "address": "0xd", "tokens": [],
        "total": 0, "limit": 50, "offset": 0, "has_more": false, "meta": null,
        "launches_24h": {"window_start": "a", "window_end": "b", "count": null,
            "status": "source_unavailable", "completeness": "indexed_observations_only"}
    }))
    .unwrap();
    let l = dt.launches_24h.unwrap();
    assert!(l.count.is_none());
    assert_eq!(l.status.as_deref(), Some("source_unavailable"));

    // Token detail + batch: liquidity_source / liquidity_status.
    let td: TokenDetailResponse = serde_json::from_value(json!({
        "chain": "robinhood", "token_address": "0xt",
        "liquidity_source": "bonding_curve_reserve", "liquidity_status": "unavailable"
    }))
    .unwrap();
    assert_eq!(td.liquidity_status.as_deref(), Some("unavailable"));
    let tb: TokenBatchResponse = serde_json::from_value(json!({
        "chain": "robinhood", "requested": 1, "found": 1,
        "tokens": [{"address": "0xt", "found": true, "liquidity_source": "pool_balances",
            "liquidity_status": "verified_zero", "liquidity_usd": 0.0}]
    }))
    .unwrap();
    assert_eq!(
        tb.tokens[0].liquidity_source.as_deref(),
        Some("pool_balances")
    );
    assert_eq!(
        tb.tokens[0].liquidity_status.as_deref(),
        Some("verified_zero")
    );
}
