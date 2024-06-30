pub fn from_sudoku_exchange_bank_str(line: &str) -> Option<String> {
    // format is described in the grantm/sudoku-exchange-puzzle-bank README.md
    // it has 3 space-separated fields:
    // - SHA1 hash of the digit string
    // - digit string
    // - rating

    if let Some(s) = line.split(" ").skip(1).next() {
        Some(s.to_string())
    } else {
        None
    }
}
