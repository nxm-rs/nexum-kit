use std::collections::HashMap;

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Connect button
    map.insert("connect_wallet.label", "Connect Wallet");
    map.insert("connect_wallet.connecting", "Connecting...");
    map.insert("connect_wallet.wrong_network", "Wrong network");

    // Connect modal
    map.insert("connect_modal.title", "Connect a Wallet");
    map.insert("connect_modal.what_is_wallet", "What is a Wallet?");
    map.insert("connect_modal.get_wallet", "I don't have a wallet");
    map.insert("connect_modal.install_extension", "Install Extension");
    map.insert("connect_modal.not_available", "Not available");

    // Account modal
    map.insert("account_modal.title", "Account");
    map.insert("account_modal.disconnect", "Disconnect");
    map.insert("account_modal.copy_address", "Copy Address");
    map.insert("account_modal.copied", "Copied!");
    map.insert("account_modal.view_explorer", "View on Explorer");

    // Chain modal
    map.insert("chain_modal.title", "Switch Networks");
    map.insert("chain_modal.wrong_network", "Wrong network");
    map.insert("chain_modal.switching", "Switching...");

    // Transaction status
    map.insert("transaction.pending", "Transaction pending");
    map.insert("transaction.confirmed", "Transaction confirmed");
    map.insert("transaction.failed", "Transaction failed");
    map.insert("transaction.view", "View transaction");

    // Wallet names
    map.insert("wallet.metamask", "MetaMask");
    map.insert("wallet.walletconnect", "WalletConnect");
    map.insert("wallet.coinbase", "Coinbase Wallet");
    map.insert("wallet.nexum", "Nexum");
    map.insert("wallet.trust", "Trust Wallet");

    // Errors
    map.insert("error.connection_failed", "Connection failed");
    map.insert("error.user_rejected", "User rejected the request");
    map.insert("error.not_installed", "Wallet not installed");
    map.insert("error.unsupported_chain", "Unsupported chain");

    // Common
    map.insert("common.cancel", "Cancel");
    map.insert("common.confirm", "Confirm");
    map.insert("common.close", "Close");
    map.insert("common.back", "Back");
    map.insert("common.learn_more", "Learn More");

    map
}
