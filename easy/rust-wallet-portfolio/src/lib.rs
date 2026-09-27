use std::collections::HashMap;

// Complete the two functions below using the rules in README.txt.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenEntry {
    pub symbol: String,
    pub amount: u64,
}

pub fn build_portfolio(entries: &[TokenEntry]) -> HashMap<String, u64> {
    let _ = entries;
    todo!("build the portfolio")
}

pub fn balance_of(portfolio: &HashMap<String, u64>, symbol: &str) -> u64 {
    let _ = (portfolio, symbol);
    todo!("look up the balance")
}
