## Final Goal

The final goal is to somehow prepare to be able to evaluate the hand in the following code:

```rust
/// - `key`: some unique value that represents 7-card combination
/// - `mask`: bit mask with exactly 7 bits set to 1 (suits are in 16-bit groups)
pub struct Hand { key: u32, mask: u64 }

impl Hand {
    /// Returns hand strength in 16-bit integer.
    /// (this is a pseudo-code; won't compile because casts are omitted)
    pub fn evaluate(&self) -> u16 {
        // extract suit information
        let suit_key = self.key >> RANK_KEY_BITS;

        // check whether the hand is flush or not
        let is_flush = FLUSH_TABLE[suit_key];

        if is_flush >= 0 {
            // when flush, use 13-bit mask as a key:
            // the key is at most 0b1111111000000 = 8128
            let flush_key = (self.mask >> (16 * is_flush)) & ((1 << NUMBER_OF_RANKS) - 1);

            // refer lookup table for flush
            LOOKUP_FLUSH[flush_key]
        } else {
            // mix bits by multiplying some odd number
            let mixed_key = (self.key * MIX_MULTIPLIER) & RANK_KEY_MASK;

            // compute hash by a single displacement method
            let hash_key = mixed_key + OFFSETS[mixed_key >> OFFSET_SHIFT];

            // refer lookup table; the number of non-zero elements is 49205
            LOOKUP[hash_key]
        }
    }
}
```

## Hand Representation

First, consider how to associate `key` value with information about how many cards of each rank/suit are there.

We want to represent `key` value as a simple sum of the card values: for example, if we assign `Deuce = 1` and `Trey = 5` then seven card combination of four deuces and three treys has `key` value of `19 (= 4 * 1 + 3 * 5)`.

The most obvious representation is to assign ranks to the power of 5 and suits to the power of 8, and this scheme requires 31-bit space for ranks and 12-bit space for suits.

`01-rank_bases.rs` finds more efficient bases for ranks by the greedy method. It gives us bases of [0, 1, 5, 22, 98, 453, 2031, 8698, 22854, 83661, 262349, 636345, 1479181] and this set requires only 23-bit space. There might be a more efficient set of bases, but here we will use this.

The optimal bases for suits is [0, 1, 29, 37]. It requires a 9-bit space, and thus we can store the information just in 32-bit space.

## Flush Checking

Once we obtain the sum of suit values, it is easy to check whether the hand is flush or not. When the hand is flush, there are no possibilities that the hand is also four-of-a-kind or full house; so we can completely divide the process into flush and non-flush.

`02-flush_table.rs` precomputes a lookup table for judging flush.

## Perfect Hashing for Non-flush

When the hand is not flush, the hand strength can be computed only by the sum of rank values. However, making a lookup table with a 23-bit index is too expensive. Actually, there are only 49205 possible values for rank sums.

This is where the complete hash function comes in. The complete hash function is a hash function that is injective, so collisions do not occur by principle.

Here we use a simple hash function called the single displacement method. `03-offset_table.rs` attempts to generate an offset table used in the hash function in which the maximum hash key is minimized.

To achieve a better compression ratio, we also *mix the bits* before applying the hash function by multiplying an odd number. A decent multiplier is also given by `03-offset_table.rs`.

## Now, Refer the Lookup Table!

`04-lookup_tables.rs` computes lookup tables both for flushes and non-flushes. The lookup table for flushes has 8,129 entries (= 16kB) and that for non-flushes has 49,205 entries (= 96kB).

Although there are 52 choose 5 (= 2,598,960) unique five-card poker hands, many of those have the same strength; actually, it is known that there are only 7,462 *equivalence classes* on five-card poker. Therefore, the return value fits in a 16-bit integer.
