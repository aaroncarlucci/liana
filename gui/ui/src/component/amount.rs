pub use bitcoin::Amount;

use crate::{color, component::text::*, widget::*};

pub fn amount<'a, T: 'a>(a: &Amount) -> Row<'a, T> {
    amount_with_size(a, P1_SIZE)
}

pub fn amount_with_size<'a, T: 'a>(a: &Amount, size: u16) -> Row<'a, T> {
    let spacing = if size > P1_SIZE { 10 } else { 5 };

    let (btc, sats) = split_digits(&a.to_btc().to_string());
    let row = Row::new().spacing(spacing).push(
        Row::new()
            .push(text(btc).size(size).bold())
            .push(text(".").size(size))
            .push(text(sats).size(size)),
    );

    Row::with_children(vec![
        row.into(),
        text("BTC").size(size).style(color::GREY_3).into(),
    ])
    .spacing(spacing)
    .align_items(iced::Alignment::Center)
}

pub fn unconfirmed_amount_with_size<'a, T: 'a>(a: &Amount, size: u16) -> Row<'a, T> {
    let spacing = if size > P1_SIZE { 10 } else { 5 };

    let (btc, sats) = split_digits(&a.to_btc().to_string());
    let row = Row::new().spacing(spacing).push(
        Row::new()
            .push(text(btc).size(size))
            .push(text(".").size(size))
            .push(text(sats).size(size)),
    );

    Row::with_children(vec![
        row.into(),
        text("BTC").size(size).style(color::GREY_3).into(),
    ])
    .spacing(spacing)
    .align_items(iced::Alignment::Center)
}

fn split_digits(amount: &str) -> (String, String) {
    let (integer, fraction) = match amount.split_once('.') {
        Some((i, f)) => (i, f),
        None => (amount, "00000000"),
    };

    let mut integer = integer
        .chars()
        .collect::<Vec<_>>()
        .rchunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>();
    integer.reverse();
    let integer = integer.join(" ");

    let fraction = format!("{:0<8}", fraction);
    let mut fraction = fraction
        .chars()
        .collect::<Vec<_>>()
        .rchunks(3)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<_>>();
    fraction.reverse();
    let fraction = fraction.join(" ");

    (integer, fraction)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_split_digits() {
        assert_eq!(
            ("1 000".to_string(), "12 345 678".to_string()),
            split_digits("1000.12345678")
        );
        assert_eq!(
            ("1".to_string(), "12 345 678".to_string()),
            split_digits("1.12345678")
        );
        assert_eq!(
            ("0".to_string(), "12 345 678".to_string()),
            split_digits("0.12345678")
        );
        assert_eq!(
            ("0".to_string(), "00 000 001".to_string()),
            split_digits("0.00000001")
        );
        assert_eq!(
            ("1".to_string(), "00 000 000".to_string()),
            split_digits("1")
        );
        assert_eq!(
            ("5 100".to_string(), "00 000 000".to_string()),
            split_digits("5100.00000000")
        );
        assert_eq!(
            ("1 000".to_string(), "10 000 000".to_string()),
            split_digits("1000.1")
        );
        assert_eq!(
            ("1 000".to_string(), "10 100 000".to_string()),
            split_digits("1000.101")
        );
    }
}
