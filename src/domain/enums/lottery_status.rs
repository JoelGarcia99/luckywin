pub enum LotteryStatus {
    Paid,
    Unpaid,
}

impl ToString for LotteryStatus {
    fn to_string(&self) -> String {
        match self {
            LotteryStatus::Paid => "pagado".to_string(),
            LotteryStatus::Unpaid => "pendiente".to_string(),
        }
    }
}
