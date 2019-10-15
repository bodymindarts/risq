use super::currency::Currency;

#[derive(Clone, PartialEq)]
pub struct MonetaryAmount {
    base_amount: u64,
    currency: &'static Currency,
}

impl MonetaryAmount {
    pub fn new(base_amount: u64, currency: &'static Currency) -> Self {
        Self {
            base_amount,
            currency,
        }
    }

    pub fn format(&self) -> String {
        let mut ret = String::new();
        let mut rest_amount = self.base_amount;

        while ret.len() < self.currency.precision as usize {
            ret.push(char_of_last_digit(rest_amount));
            rest_amount = rest_amount / 10;
        }
        ret.push('.');

        while rest_amount > 0 {
            ret.push(char_of_last_digit(rest_amount));
            rest_amount = rest_amount / 10;
        }
        if ret.len() == self.currency.precision as usize + 1 {
            ret.push('0');
        }
        ret.chars().rev().collect()
    }
}

fn char_of_last_digit(n: u64) -> char {
    match n % 10 {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => unimplemented!(),
    }
}
