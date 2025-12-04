use alloy::primitives::Address;

/// Format an Ethereum address for display
///
/// Truncates the address to show first 6 and last 4 characters (including 0x prefix).
/// Example: 0x1234567890abcdef... -> 0x1234...cdef
///
/// This matches the original RainbowKit's address formatting behavior.
pub fn format_address(address: &Address) -> String {
    let addr_str = format!("{:?}", address);

    if addr_str.len() <= 10 {
        return addr_str;
    }

    format!("{}...{}", &addr_str[0..6], &addr_str[addr_str.len()-4..])
}

/// Format a balance for display
///
/// Converts a raw balance (in wei) to a decimal representation with the given decimals.
/// Shows up to 4 decimal places.
///
/// # Arguments
///
/// * `balance` - The balance in the smallest unit (e.g., wei for ETH)
/// * `decimals` - Number of decimal places for the token (18 for ETH, 6 for USDC, etc.)
///
/// # Example
///
/// ```rust
/// use leptos_rainbowkit::utils::format_balance;
///
/// // 1.5 ETH (1500000000000000000 wei)
/// let formatted = format_balance(1_500_000_000_000_000_000, 18);
/// assert_eq!(formatted, "1.5000");
/// ```
pub fn format_balance(balance: u128, decimals: u8) -> String {
    if balance == 0 {
        return "0.0000".to_string();
    }

    let divisor = 10u128.pow(decimals as u32);
    let whole = balance / divisor;
    let fractional = balance % divisor;

    // Calculate fractional part with 4 decimal places
    let fractional_display = fractional / (divisor / 10_000);

    format!("{}.{:04}", whole, fractional_display)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_address() {
        let address: Address = "0x1234567890abcdef1234567890abcdef12345678".parse().unwrap();
        let formatted = format_address(&address);
        assert_eq!(formatted, "0x1234...5678");
    }

    #[test]
    fn test_format_balance() {
        // 1.5 ETH
        assert_eq!(format_balance(1_500_000_000_000_000_000, 18), "1.5000");

        // 0.1234 ETH
        assert_eq!(format_balance(123_400_000_000_000_000, 18), "0.1234");

        // 1000 USDC (6 decimals)
        assert_eq!(format_balance(1_000_000_000, 6), "1000.0000");

        // Zero balance
        assert_eq!(format_balance(0, 18), "0.0000");
    }
}
