## Final Goal

The final goal is to somehow enable evaluation of the hand in the following code:

```rust
/// - `key`: some unique value that represents 5-7 card combination
/// - `mask`: bit mask with 5-7 bits set to 1 (suits are in 16-bit groups)
pub struct Hand { key: u64, mask: u64 }

impl Hand {
    /// Returns hand strength in 16-bit integer.
    /// (this is a pseudo-code; won't compile because casts are omitted)
    pub fn evaluate(&self) -> u16 {
        // check whether the hand is flush or not
        let is_flush = self.key & FLUSH_MASK;

        if is_flush > 0 {
            // when flush, use 13-bit mask as a key:
            // the key is at most 0b1111111000000 = 8128
            let flush_key = (self.mask >> (4 * is_flush.leading_zeros())) as u16;

            // refer lookup table for flush
            LOOKUP_FLUSH[flush_key]
        } else {
            // get key value that represents the combination of ranks
            let rank_key = self.key as u32;

            // compute hash by a single displacement method
            let hash_key = mixed_key + OFFSETS[mixed_key >> OFFSET_SHIFT];

            // refer lookup table; the number of non-zero elements is 73775
            LOOKUP[hash_key]
        }
    }
}
```

## Hand Representation

First, consider how to associate `key` value with information about how many cards of each rank/suit are there.

We want to represent `key` value as a simple sum of the card values: for example, if we assign `Deuce = 1` and `Trey = 5` then a seven-card combination of four deuces and three treys has `key` value of `19 (= 4 * 1 + 3 * 5)`.

The most obvious representation is to assign ranks to the power of 5 and suits to the power of 8, and this scheme requires 31-bit space for ranks and 12-bit space for suits.

[OMPEval](https://github.com/zekyll/OMPEval) uses more sophisticated bases: [0x2000, 0x8001, 0x11000, 0x3a000, 0x91000, 0x176005, 0x366000, 0x41a013, 0x47802e, 0x479068, 0x48c0e4, 0x48f211, 0x494493]. This set requires only 25-bit space.

## Perfect Hashing for Non-flush

When the hand is not flush, the hand strength can be computed only by the sum of rank values. However, referring to a lookup table with a 24-bit index is inefficient. Actually, there are only 73,775 possible values for rank sums.

This is where the complete hash function comes in. The complete hash function is a hash function that is injective, so collisions do not occur by principle.

Here we use a simple hash function called the single displacement method. `01-offset_table.rs` attempts to generate an offset table used in the hash function in which the maximum hash key is minimized.

## Now, Refer to the Lookup Table!

`02-lookup_tables.rs` generates lookup tables both for flushes and non-flushes. The lookup table for flushes has 8,129 entries (= 16KB) and that for non-flushes has at least 73,775 entries (= 144KB).

Although there are 52 choose 5 (= 2,598,960) unique five-card poker hands, many of those have the same strength; actually, it is known that there are only 7,462 equivalence classes on five-card poker. Therefore, the return value fits in a 16-bit integer.
