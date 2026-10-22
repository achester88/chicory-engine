use crate::chicory::bitboard::Bitboard;

pub fn bit_scan(num: Bitboard) -> usize {
    if !num.is_empty() {
        return num.val.trailing_zeros() as usize;
    }

    0
}

pub fn bit_scan_neg(num: Bitboard) -> usize {
    (num.val.leading_zeros() ^ 63) as usize
}
