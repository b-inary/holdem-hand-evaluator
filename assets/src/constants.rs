/// number of ranks
pub const NUMBER_OF_RANKS: usize = 13;

/// number of ranks
pub const NUMBER_OF_CARDS: usize = 4 * NUMBER_OF_RANKS;

/// determines perfect hash function. adjust this parameter to modify the offset table
pub const OFFSET_SHIFT: usize = 10;

/// rank values that guarantee a unique sum for every rank combination of 7 cards.
/// computed by 'scripts/src/01-compute_bases.rs'
pub const BASES: [u32; NUMBER_OF_RANKS] = [
    0, 1, 5, 22, 98, 453, 2031, 8698, 22854, 83661, 262349, 636345, 1479181,
];

/// max rank key value (4 aces + 3 kings)
pub const MAX_KEY: u32 = 4 * BASES[NUMBER_OF_RANKS - 1] + 3 * BASES[NUMBER_OF_RANKS - 2];

/// number of bits of rank key
pub const KEY_BITS: usize = 32 - MAX_KEY.leading_zeros() as usize;

/// bit mask for rank key
pub const KEY_MASK: u32 = (1 << KEY_BITS) - 1;

/// suit value for club
pub const CLUB: u32 = 0;

/// suit value for diamond
pub const DIAMOND: u32 = 1;

/// suit value for heart
pub const HEART: u32 = 29;

/// suit value for spade
pub const SPADE: u32 = 37;

/// card IDs
pub const CARDS: [u32; NUMBER_OF_CARDS] = [
    /* 2c */ BASES[0] + (CLUB << KEY_BITS),
    /* 2d */ BASES[0] + (DIAMOND << KEY_BITS),
    /* 2h */ BASES[0] + (HEART << KEY_BITS),
    /* 2s */ BASES[0] + (SPADE << KEY_BITS),
    /* 3c */ BASES[1] + (CLUB << KEY_BITS),
    /* 3d */ BASES[1] + (DIAMOND << KEY_BITS),
    /* 3h */ BASES[1] + (HEART << KEY_BITS),
    /* 3s */ BASES[1] + (SPADE << KEY_BITS),
    /* 4c */ BASES[2] + (CLUB << KEY_BITS),
    /* 4d */ BASES[2] + (DIAMOND << KEY_BITS),
    /* 4h */ BASES[2] + (HEART << KEY_BITS),
    /* 4s */ BASES[2] + (SPADE << KEY_BITS),
    /* 5c */ BASES[3] + (CLUB << KEY_BITS),
    /* 5d */ BASES[3] + (DIAMOND << KEY_BITS),
    /* 5h */ BASES[3] + (HEART << KEY_BITS),
    /* 5s */ BASES[3] + (SPADE << KEY_BITS),
    /* 6c */ BASES[4] + (CLUB << KEY_BITS),
    /* 6d */ BASES[4] + (DIAMOND << KEY_BITS),
    /* 6h */ BASES[4] + (HEART << KEY_BITS),
    /* 6s */ BASES[4] + (SPADE << KEY_BITS),
    /* 7c */ BASES[5] + (CLUB << KEY_BITS),
    /* 7d */ BASES[5] + (DIAMOND << KEY_BITS),
    /* 7h */ BASES[5] + (HEART << KEY_BITS),
    /* 7s */ BASES[5] + (SPADE << KEY_BITS),
    /* 8c */ BASES[6] + (CLUB << KEY_BITS),
    /* 8d */ BASES[6] + (DIAMOND << KEY_BITS),
    /* 8h */ BASES[6] + (HEART << KEY_BITS),
    /* 8s */ BASES[6] + (SPADE << KEY_BITS),
    /* 9c */ BASES[7] + (CLUB << KEY_BITS),
    /* 9d */ BASES[7] + (DIAMOND << KEY_BITS),
    /* 9h */ BASES[7] + (HEART << KEY_BITS),
    /* 9s */ BASES[7] + (SPADE << KEY_BITS),
    /* Tc */ BASES[8] + (CLUB << KEY_BITS),
    /* Td */ BASES[8] + (DIAMOND << KEY_BITS),
    /* Th */ BASES[8] + (HEART << KEY_BITS),
    /* Ts */ BASES[8] + (SPADE << KEY_BITS),
    /* Jc */ BASES[9] + (CLUB << KEY_BITS),
    /* Jd */ BASES[9] + (DIAMOND << KEY_BITS),
    /* Jh */ BASES[9] + (HEART << KEY_BITS),
    /* Js */ BASES[9] + (SPADE << KEY_BITS),
    /* Qc */ BASES[10] + (CLUB << KEY_BITS),
    /* Qd */ BASES[10] + (DIAMOND << KEY_BITS),
    /* Qh */ BASES[10] + (HEART << KEY_BITS),
    /* Qs */ BASES[10] + (SPADE << KEY_BITS),
    /* Kc */ BASES[11] + (CLUB << KEY_BITS),
    /* Kd */ BASES[11] + (DIAMOND << KEY_BITS),
    /* Kh */ BASES[11] + (HEART << KEY_BITS),
    /* Ks */ BASES[11] + (SPADE << KEY_BITS),
    /* Ac */ BASES[12] + (CLUB << KEY_BITS),
    /* Ad */ BASES[12] + (DIAMOND << KEY_BITS),
    /* Ah */ BASES[12] + (HEART << KEY_BITS),
    /* As */ BASES[12] + (SPADE << KEY_BITS),
];

/// cards bit
pub const CARDS_BIT: [u64; NUMBER_OF_CARDS] = [
    /* 2c */ 0x0000000000000001,
    /* 2d */ 0x0000000000010000,
    /* 2h */ 0x0000000100000000,
    /* 2s */ 0x0001000000000000,
    /* 3c */ 0x0000000000000002,
    /* 3d */ 0x0000000000020000,
    /* 3h */ 0x0000000200000000,
    /* 3s */ 0x0002000000000000,
    /* 4c */ 0x0000000000000004,
    /* 4d */ 0x0000000000040000,
    /* 4h */ 0x0000000400000000,
    /* 4s */ 0x0004000000000000,
    /* 5c */ 0x0000000000000008,
    /* 5d */ 0x0000000000080000,
    /* 5h */ 0x0000000800000000,
    /* 5s */ 0x0008000000000000,
    /* 6c */ 0x0000000000000010,
    /* 6d */ 0x0000000000100000,
    /* 6h */ 0x0000001000000000,
    /* 6s */ 0x0010000000000000,
    /* 7c */ 0x0000000000000020,
    /* 7d */ 0x0000000000200000,
    /* 7h */ 0x0000002000000000,
    /* 7s */ 0x0020000000000000,
    /* 8c */ 0x0000000000000040,
    /* 8d */ 0x0000000000400000,
    /* 8h */ 0x0000004000000000,
    /* 8s */ 0x0040000000000000,
    /* 9c */ 0x0000000000000080,
    /* 9d */ 0x0000000000800000,
    /* 9h */ 0x0000008000000000,
    /* 9s */ 0x0080000000000000,
    /* Tc */ 0x0000000000000100,
    /* Td */ 0x0000000001000000,
    /* Th */ 0x0000010000000000,
    /* Ts */ 0x0100000000000000,
    /* Jc */ 0x0000000000000200,
    /* Jd */ 0x0000000002000000,
    /* Jh */ 0x0000020000000000,
    /* Js */ 0x0200000000000000,
    /* Qc */ 0x0000000000000400,
    /* Qd */ 0x0000000004000000,
    /* Qh */ 0x0000040000000000,
    /* Qs */ 0x0400000000000000,
    /* Kc */ 0x0000000000000800,
    /* Kd */ 0x0000000008000000,
    /* Kh */ 0x0000080000000000,
    /* Ks */ 0x0800000000000000,
    /* Ac */ 0x0000000000001000,
    /* Ad */ 0x0000000010000000,
    /* Ah */ 0x0000100000000000,
    /* As */ 0x1000000000000000,
];
