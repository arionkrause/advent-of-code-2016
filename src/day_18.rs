pub fn solve(input: &str) {
    println!("Day {}.", file!().chars().filter(|c| c.is_digit(10)).collect::<String>());
    println!("Part 1: {}.", part_1::solve(&input, 40));
    println!("Part 2: {}.", part_2::solve(&input, 400_000));
    println!();
}

fn get_amount_safe_tiles(input: &str, amount_rows: usize) -> usize {
    // Credits: https://github.com/birkenfeld/advent16/blob/master/src/bin/day18.rs
    // This algorithm is amazingly clever!
    // 1's are traps
    // 0's are safe tiles
    let mut amount_safe_tiles = 0;
    let mut row = 0u128;

    // For each character in the input, set the row's last bit to "1" or "0", then shift left.
    // When shifting left, a "0" (i.e. a safe tile) is automatically inserted as the last bit, and it's only changed to "1" if the next character is "^".
    // A trailing "0" is automatically inserted as padding (it is used to check for traps in the following "for" loop).
    // Example: input = "..^^.".
    // Character #1 = "."; row after shifting = "(...)000000" (since it was already all "0", no changes are effectively made).
    // Character #2 = "."; row after shifting = "(...)000000" (since it was already all "0", no changes are effectively made).
    // Character #3 = "^"; row after shifting = "(...)000010" (a "1" is inserted as the last bit, then all bits are shifted left).
    // Character #4 = "^"; row after shifting = "(...)000110" (a "1" is inserted as the last bit, then all bits are shifted left).
    // Character #5 = "."; row after shifting = "(...)001100" (a "0" is inserted as the last bit, then all bits are shifted left).
    // "amount_tiles" is effectively just the amount of characters processed by the "map" iterator (i.e. 5 characters).
    let amount_tiles = input.chars().map(|character| row = (if character == '^' { row | 1 } else { row }) << 1).count();

    // Set the mask to match the amount of tiles.
    // Example: amount_tiles = 5.
    // (...)000001  "1": a simple "1" in decimal or "(...)000001" in binary.
    // (...)100000  "1 << amount_tiles": a "1" is inserted and shifted left 5 times. This results in "32" in decimal or "100000" in binary.
    // (...)011111  "(1 << amount_tiles) - 1": "32 - 1" is "31" in decimal or "011111" in binary, effectively inverting all bits up to the "1".
    // (...)111110  "((1 << amount_tiles) - 1) << 1": shifting left to pad last bit (this last bit is used to check for traps in the following "for" loop).
    let mask = ((1 << amount_tiles) - 1) << 1;

    for _ in 0..amount_rows {
        amount_safe_tiles += amount_tiles - row.count_ones() as usize;

        // For the new row, only the tiles "left" and "right" matter, because if they're equal, the tile is safe.
        // Example (dots are actually zeroes):
        // .10111.  Row representing "^.^^^"
        // 10111..  Row shifted left
        // ..10111  Row shifted right
        // 1001011  "Row shifted left" XORed "Row shifted right"
        // .11111.  Mask
        // .00101.  With mask applied. Results "..^.^"
        row = ((row << 1) ^ (row >> 1)) & mask;
    }

    amount_safe_tiles
}

mod part_1 {
    use crate::day_18::get_amount_safe_tiles;

    pub fn solve(input: &str, amount_rows: usize) -> usize {
        get_amount_safe_tiles(&input, amount_rows)
    }

    #[cfg(test)]
    #[test]
    fn test_1() {
        assert_eq!(solve("..^^.", 3), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve(".^^.^.^^^^", 10), 38);
    }
}

mod part_2 {
    use crate::day_18::get_amount_safe_tiles;

    pub fn solve(input: &str, amount_rows: usize) -> usize {
        get_amount_safe_tiles(&input, amount_rows)
    }
}
