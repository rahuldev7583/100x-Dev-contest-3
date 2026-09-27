use rust_wallet_portfolio::{balance_of, build_portfolio, TokenEntry};

fn entry(symbol: &str, amount: u64) -> TokenEntry {
    TokenEntry {
        symbol: symbol.into(),
        amount,
    }
}

#[test]
fn combines_repeated_symbols_and_keeps_other_symbols() {
    let portfolio = build_portfolio(&[entry("SOL", 2), entry("ETH", 1), entry("SOL", 3)]);
    assert_eq!(portfolio.len(), 2);
    assert_eq!(portfolio.get("SOL"), Some(&5));
    assert_eq!(portfolio.get("ETH"), Some(&1));
}

#[test]
fn empty_portfolio_and_missing_lookup() {
    let portfolio = build_portfolio(&[]);
    assert!(portfolio.is_empty());
    assert_eq!(balance_of(&portfolio, "BTC"), 0);
}
