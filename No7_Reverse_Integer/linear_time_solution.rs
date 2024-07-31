impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let divisor = 10;
        let sign = match x >= 0 { true => 1, false => -1};
        let mut x_pos = x.abs();
        // if x == i32::MIN, rust abs() method will return i32::MIN (since the result of abs would be greater than i32::MAX)
        // This results in one edge case with immediate return of 0
        if x_pos == std::i32::MIN { return 0;};
        // preallocate max size of vector to ensure no dynamic allocation during runtime
        let mut digits_v = Vec::<i32>::with_capacity(10);
        while x_pos != 0 {
            let digit : i32 = x_pos % divisor;
            digits_v.push(digit);
            x_pos = x_pos.div_euclid(10);
        }

        // Calculate reverse number
        let mut x_rev = 0;
        let mut factor = 1;
        //In case of an overflow, the resulting sum would NOT always be smaller, than before.
        // Since we are calculating in base 10, the range of values added is in [9*factor, 0]. The maximum factor reachable is
        // determined by the number of digits of 2^31 in base 10, therefore the maximum factor is 1_000_000_000. 
        // Example: Suppose our x_rev value is 0, and we have a only digits 0 until we reach the maximum factor.
        // To force an overflow, that flows exactly back to 0, we need to add ofl_to_zero = 2*i32::MAX + i32::MAX -1. To not force another overflow, the
        // number must not be bigger than ofl_to_zero + i32::MAX, so in base 10 the resulting range is:
        // [6442450941, 8589934588]. So a digit with value 7 or 8 would cause an overflow, that will not result in a smaller value than before.
        // So a straight check, if the value got smaller after adding a positive number is not enough.
        // On the other hand, the only time when an overflow can occur, is when adding a digit with the maximum factor.
        // Proof: Maximum number has 10 digits, maximum value with 9 digits is 999_999_999 < i32::MAX (case for i32::MIN analog), so overflow occurs with digits >2 with max factor in two cases.
        // 1. factor >2 -> always overflow
        // 2. factor ==2 -> overflows if previous value is bigger than i32::MAX - 2_000_000_000 for negative sign, or bigger than i32::MAX - 1_999_999_999 for positive sign.


        for (ind, d) in digits_v.iter().rev().enumerate() {
        // Do special check for edge case
        if ind == 9 {
            match d {
                &x if x > 2 => return 0,
                &x if x == 2 => { 
                    match x_rev { 
                        y if (x_rev > std::i32::MAX - 2000000000 , sign < 0) == (true, true) => return 0,
                        y if (x_rev > std::i32::MAX - 1999999999, sign > 0) == (true, true) => return 0,
                        _ => (),

                }},
                _ => (),
            }
        }
            x_rev = x_rev + d * factor;
            factor = factor * 10;


    
        }
        match sign * x_rev {
            x if {x > std::i32::MAX} => 0,
            x if {x < std::i32::MIN} => 0,
            x => x,
        }
    }
}