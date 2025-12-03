use std::collections::HashMap;

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Connect button
    map.insert("connect_wallet.label", "Connecter le Portefeuille");
    map.insert("connect_wallet.connecting", "Connexion...");
    map.insert("connect_wallet.wrong_network", "Mauvais réseau");

    // Connect modal
    map.insert("connect_modal.title", "Connecter un Portefeuille");
    map.insert("connect_modal.what_is_wallet", "Qu'est-ce qu'un Portefeuille ?");
    map.insert("connect_modal.get_wallet", "Je n'ai pas de portefeuille");
    map.insert("connect_modal.install_extension", "Installer l'Extension");
    map.insert("connect_modal.not_available", "Non disponible");

    // Account modal
    map.insert("account_modal.title", "Compte");
    map.insert("account_modal.disconnect", "Déconnecter");
    map.insert("account_modal.copy_address", "Copier l'Adresse");
    map.insert("account_modal.copied", "Copié !");
    map.insert("account_modal.view_explorer", "Voir sur l'Explorateur");

    // Chain modal
    map.insert("chain_modal.title", "Changer de Réseaux");
    map.insert("chain_modal.wrong_network", "Mauvais réseau");
    map.insert("chain_modal.switching", "Changement...");

    // Transaction status
    map.insert("transaction.pending", "Transaction en attente");
    map.insert("transaction.confirmed", "Transaction confirmée");
    map.insert("transaction.failed", "Transaction échouée");
    map.insert("transaction.view", "Voir la transaction");

    // Wallet names
    map.insert("wallet.metamask", "MetaMask");
    map.insert("wallet.walletconnect", "WalletConnect");
    map.insert("wallet.coinbase", "Coinbase Wallet");
    map.insert("wallet.rainbow", "Rainbow");
    map.insert("wallet.trust", "Trust Wallet");

    // Errors
    map.insert("error.connection_failed", "Échec de la connexion");
    map.insert("error.user_rejected", "L'utilisateur a rejeté la demande");
    map.insert("error.not_installed", "Portefeuille non installé");
    map.insert("error.unsupported_chain", "Chaîne non prise en charge");

    // Common
    map.insert("common.cancel", "Annuler");
    map.insert("common.confirm", "Confirmer");
    map.insert("common.close", "Fermer");
    map.insert("common.back", "Retour");
    map.insert("common.learn_more", "En savoir plus");

    map
}
