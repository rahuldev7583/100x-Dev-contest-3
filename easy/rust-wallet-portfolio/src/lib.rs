use std::collections::HashMap;

// Complete the two functions below using the rules in README.txt.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenEntry {
    pub symbol: String,
    pub amount: u64,
}

pub fn build_portfolio(entries: &[TokenEntry]) -> HashMap<String, u64> {
    let _ = entries;
    let mut port: HashMap<String, u64> = HashMap::new();

    let itr = entries.iter();

    for i in itr {
        //port.insert(i.symbol.clone(), i.amount);
        let ast = port.get(&i.symbol);

        match ast {
            None => port.insert(i.symbol.clone(), i.amount),
            Some(a) => port.insert(i.symbol.clone(), a + i.amount),
        };
    }

    println!("port: {:?}", port);
    return port;
}

pub fn balance_of(portfolio: &HashMap<String, u64>, symbol: &str) -> u64 {
    let _ = (portfolio, symbol);
    let ast = portfolio.get(symbol);

    match ast {
        None => 0,
        Some(a) => a.to_owned(),
    }
}
