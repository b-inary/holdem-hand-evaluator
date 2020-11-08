/// number of ranks
pub const NUMBER_OF_RANKS: usize = 13;

/// number of ranks
pub const NUMBER_OF_CARDS: usize = 4 * NUMBER_OF_RANKS;

/// determines perfect hash function. adjust this parameter to modify the offset table
pub const OFFSET_SHIFT: usize = 10;

/// rank values that guarantee a unique sum for every rank combination of 7 cards.
/// computed by 'scripts/src/01-compute_bases.rs'
pub const BASES: [u64; NUMBER_OF_RANKS] = [
    0, 1, 5, 22, 98, 453, 2031, 8698, 22854, 83661, 262349, 636345, 1479181,
];

/// max rank key value (4 aces + 3 kings)
pub const MAX_KEY: u64 = 4 * BASES[NUMBER_OF_RANKS - 1] + 3 * BASES[NUMBER_OF_RANKS - 2];

/// number of bits of rank key
pub const KEY_BITS: usize = 64 - MAX_KEY.leading_zeros() as usize;

/// bit mask for rank key
pub const KEY_MASK: u64 = (1 << KEY_BITS) - 1;

/// suit value for club
pub const CLUB: u64 = 0;

/// suit value for diamond
pub const DIAMOND: u64 = 1;

/// suit value for heart
pub const HEART: u64 = 29;

/// suit value for spade
pub const SPADE: u64 = 37;

/// (card ID, bit mask) of cards
pub const CARDS: [(u64, u64); NUMBER_OF_CARDS] = [
    /* 2c */ (BASES[0] + (CLUB << KEY_BITS), 0x1),
    /* 2d */ (BASES[0] + (DIAMOND << KEY_BITS), 0x10000),
    /* 2h */ (BASES[0] + (HEART << KEY_BITS), 0x100000000),
    /* 2s */ (BASES[0] + (SPADE << KEY_BITS), 0x1000000000000),
    /* 3c */ (BASES[1] + (CLUB << KEY_BITS), 0x2),
    /* 3d */ (BASES[1] + (DIAMOND << KEY_BITS), 0x20000),
    /* 3h */ (BASES[1] + (HEART << KEY_BITS), 0x200000000),
    /* 3s */ (BASES[1] + (SPADE << KEY_BITS), 0x2000000000000),
    /* 4c */ (BASES[2] + (CLUB << KEY_BITS), 0x4),
    /* 4d */ (BASES[2] + (DIAMOND << KEY_BITS), 0x40000),
    /* 4h */ (BASES[2] + (HEART << KEY_BITS), 0x400000000),
    /* 4s */ (BASES[2] + (SPADE << KEY_BITS), 0x4000000000000),
    /* 5c */ (BASES[3] + (CLUB << KEY_BITS), 0x8),
    /* 5d */ (BASES[3] + (DIAMOND << KEY_BITS), 0x80000),
    /* 5h */ (BASES[3] + (HEART << KEY_BITS), 0x800000000),
    /* 5s */ (BASES[3] + (SPADE << KEY_BITS), 0x8000000000000),
    /* 6c */ (BASES[4] + (CLUB << KEY_BITS), 0x10),
    /* 6d */ (BASES[4] + (DIAMOND << KEY_BITS), 0x100000),
    /* 6h */ (BASES[4] + (HEART << KEY_BITS), 0x1000000000),
    /* 6s */ (BASES[4] + (SPADE << KEY_BITS), 0x10000000000000),
    /* 7c */ (BASES[5] + (CLUB << KEY_BITS), 0x20),
    /* 7d */ (BASES[5] + (DIAMOND << KEY_BITS), 0x200000),
    /* 7h */ (BASES[5] + (HEART << KEY_BITS), 0x2000000000),
    /* 7s */ (BASES[5] + (SPADE << KEY_BITS), 0x20000000000000),
    /* 8c */ (BASES[6] + (CLUB << KEY_BITS), 0x40),
    /* 8d */ (BASES[6] + (DIAMOND << KEY_BITS), 0x400000),
    /* 8h */ (BASES[6] + (HEART << KEY_BITS), 0x4000000000),
    /* 8s */ (BASES[6] + (SPADE << KEY_BITS), 0x40000000000000),
    /* 9c */ (BASES[7] + (CLUB << KEY_BITS), 0x80),
    /* 9d */ (BASES[7] + (DIAMOND << KEY_BITS), 0x800000),
    /* 9h */ (BASES[7] + (HEART << KEY_BITS), 0x8000000000),
    /* 9s */ (BASES[7] + (SPADE << KEY_BITS), 0x80000000000000),
    /* Tc */ (BASES[8] + (CLUB << KEY_BITS), 0x100),
    /* Td */ (BASES[8] + (DIAMOND << KEY_BITS), 0x1000000),
    /* Th */ (BASES[8] + (HEART << KEY_BITS), 0x10000000000),
    /* Ts */ (BASES[8] + (SPADE << KEY_BITS), 0x100000000000000),
    /* Jc */ (BASES[9] + (CLUB << KEY_BITS), 0x200),
    /* Jd */ (BASES[9] + (DIAMOND << KEY_BITS), 0x2000000),
    /* Jh */ (BASES[9] + (HEART << KEY_BITS), 0x20000000000),
    /* Js */ (BASES[9] + (SPADE << KEY_BITS), 0x200000000000000),
    /* Qc */ (BASES[10] + (CLUB << KEY_BITS), 0x400),
    /* Qd */ (BASES[10] + (DIAMOND << KEY_BITS), 0x4000000),
    /* Qh */ (BASES[10] + (HEART << KEY_BITS), 0x40000000000),
    /* Qs */ (BASES[10] + (SPADE << KEY_BITS), 0x400000000000000),
    /* Kc */ (BASES[11] + (CLUB << KEY_BITS), 0x800),
    /* Kd */ (BASES[11] + (DIAMOND << KEY_BITS), 0x8000000),
    /* Kh */ (BASES[11] + (HEART << KEY_BITS), 0x80000000000),
    /* Ks */ (BASES[11] + (SPADE << KEY_BITS), 0x800000000000000),
    /* Ac */ (BASES[12] + (CLUB << KEY_BITS), 0x1000),
    /* Ad */ (BASES[12] + (DIAMOND << KEY_BITS), 0x10000000),
    /* Ah */ (BASES[12] + (HEART << KEY_BITS), 0x100000000000),
    /* As */ (BASES[12] + (SPADE << KEY_BITS), 0x1000000000000000),
];
