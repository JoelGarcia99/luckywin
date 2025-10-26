use std::fmt::Display;

pub enum LotteryStatus {
    Paid,
    Unpaid,
}

impl Display for LotteryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LotteryStatus::Paid => write!(f, "paid"),
            LotteryStatus::Unpaid => write!(f, "unpaid"),
        }
    }
}
