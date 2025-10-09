use sea_orm::prelude::Decimal;

/// Parses a given array of numbers into a bitboard representation
pub fn parse_vec_into_bitboard(vec: Vec<u8>) -> Decimal {
    let mut bitboard = 0;

    for num in vec {
        bitboard |= 1 << num;
    }

    bitboard.into()
}

/// Parses a given bitboard into a vector
pub fn parse_bitboard_into_vec(bitboard: Decimal) -> Result<Vec<u8>, String> {
    let mut vec = Vec::new();
    // ensuring purity
    let mut bitboard = bitboard.to_string().parse::<u128>().map_err(|e| e.to_string())?;

    while bitboard > 0 {
        let num = bitboard.trailing_zeros();
        vec.push(num as u8);
        bitboard &= !(1 << num);
    }

    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vec_into_bitboard() {
        // 0 0 0 1 1 1 0
        let result = parse_vec_into_bitboard(vec![1, 2, 3]);
        assert_eq!(result, Decimal::from(14));
    }

    #[test]
    fn test_parse_bitboard_into_vec() {
        let result = parse_bitboard_into_vec(Decimal::from(14));
        assert_eq!(result, Ok(vec![1, 2, 3]));
    }
}
