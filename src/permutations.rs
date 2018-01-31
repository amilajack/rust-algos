// You have a given amount of money. Determine all the possible ways of
// representing that money given that you can use $1, $5, %10, and $20
// dollar bills
pub fn coins_problem(amount: i32) -> i32 {
    if amount <= 0 {
        0
    } else  {
        coins_problem(amount - 1) +
        coins_problem(amount - 5) +
        coins_problem(amount - 10) +
        coins_problem(amount - 20)
    }
}
