#[derive(Debug, Hash, PartialEq, Eq, Clone, Serialize)]
/// This object represents a portion of the price for goods or services.
pub struct LabeledPrice {
    /// Portion label
    pub label: String,
    /// Price of the product in the smallest units of the
    /// [currency](https://core.telegram.org/bots/payments#supported-currencies)
    /// (integer, not float/double). For example, for a price of US$ 1.45 pass
    /// amount = 145. See the exp parameter in [`currencies.json`](https://core.telegram.org/bots/payments/currencies.json),
    /// it shows the number of digits past the decimal point for each currency
    /// (2 for the majority of currencies).
    pub amount: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let labeled_price = LabeledPrice {
            label: "Label".to_string(),
            amount: 60
        };
        let expected = r#"{"label":"Label","amount":60}"#;
        let actual = serde_json::to_string(&labeled_price).unwrap();
        assert_eq!(actual, expected);
    }
}
