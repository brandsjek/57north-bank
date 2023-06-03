pub type MarketItems = std::collections::HashMap<crate::barcode::Barcode, crate::products::Product>;

//#[derive(Debug, Serialize, Deserialize, Clone)]

pub fn read_market() -> Result<MarketItems, String> {
    let mut marketitems = std::collections::HashMap::new();

    let market_raw = match std::fs::read("./data/market") {
        Ok(p) => p,
        Err(e) => return Err(format!("cannot open market file {}", e)),
    };
    let market_str = match String::from_utf8(market_raw) {
        Ok(p) => p,
        Err(e) => return Err(format!("cannot parse market file {}", e)),
    };
    let market_lines = market_str
        .split("\n")
        .filter(|l| !l.trim().is_empty() && l.chars().nth(0).unwrap() != '#')
        .collect::<Vec<_>>();

    for line in market_lines {
        let mut left = line.clone();

        let mut take_part = || match left.split_once(" ") {
            Some(v) => {
                left = v.1;
                Ok(v.0.trim())
            },
            None => return Err(format!("invalid line {}", line))
        };

        
        let barcode = take_part()?;
        let ben_price = take_part()?;
        let space_profit = take_part()?;
        let beneficiary = take_part()?;
        let descriptor = left;

        let barcode = match crate::barcode::Barcode::try_parse(barcode) {
            Some(d) => d,
            None => return Err(format!("invalid barcode {}", barcode))
        };

        let ben_price = match u32::from_str_radix(ben_price, 10) {
            Ok(p) => p,
            Err(e) => return Err(format!("invalid price {}", e))
        };

        let space_profit = match u32::from_str_radix(space_profit, 10) {
            Ok(p) => p,
            Err(e) => return Err(format!("invalid price {}", e))
        };

        marketitems.insert(barcode.clone(), crate::products::Product {
            name: descriptor.to_string(),
            ben_price,
            space_profit,
            beneficiary: beneficiary.to_string(),
            barcode,
        });
    }

    Ok(marketitems)
}