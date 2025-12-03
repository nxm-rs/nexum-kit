use std::collections::HashMap;

pub fn translations() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Connect button
    map.insert("connect_wallet.label", "Conectar Billetera");
    map.insert("connect_wallet.connecting", "Conectando...");
    map.insert("connect_wallet.wrong_network", "Red incorrecta");

    // Connect modal
    map.insert("connect_modal.title", "Conectar una Billetera");
    map.insert("connect_modal.what_is_wallet", "¿Qué es una Billetera?");
    map.insert("connect_modal.get_wallet", "No tengo una billetera");
    map.insert("connect_modal.install_extension", "Instalar Extensión");
    map.insert("connect_modal.not_available", "No disponible");

    // Account modal
    map.insert("account_modal.title", "Cuenta");
    map.insert("account_modal.disconnect", "Desconectar");
    map.insert("account_modal.copy_address", "Copiar Dirección");
    map.insert("account_modal.copied", "¡Copiado!");
    map.insert("account_modal.view_explorer", "Ver en Explorador");

    // Chain modal
    map.insert("chain_modal.title", "Cambiar Redes");
    map.insert("chain_modal.wrong_network", "Red incorrecta");
    map.insert("chain_modal.switching", "Cambiando...");

    // Transaction status
    map.insert("transaction.pending", "Transacción pendiente");
    map.insert("transaction.confirmed", "Transacción confirmada");
    map.insert("transaction.failed", "Transacción fallida");
    map.insert("transaction.view", "Ver transacción");

    // Wallet names
    map.insert("wallet.metamask", "MetaMask");
    map.insert("wallet.walletconnect", "WalletConnect");
    map.insert("wallet.coinbase", "Coinbase Wallet");
    map.insert("wallet.rainbow", "Rainbow");
    map.insert("wallet.trust", "Trust Wallet");

    // Errors
    map.insert("error.connection_failed", "Conexión fallida");
    map.insert("error.user_rejected", "Usuario rechazó la solicitud");
    map.insert("error.not_installed", "Billetera no instalada");
    map.insert("error.unsupported_chain", "Cadena no soportada");

    // Common
    map.insert("common.cancel", "Cancelar");
    map.insert("common.confirm", "Confirmar");
    map.insert("common.close", "Cerrar");
    map.insert("common.back", "Atrás");
    map.insert("common.learn_more", "Aprende Más");

    map
}
