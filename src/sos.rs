//! SOS — réglages gravés au build (voir CLAUDE.md à la racine).
//! Les deux constantes ci-dessous sont remplacées par le workflow avant compilation ;
//! laissées telles quelles (build local), rien n'est appliqué.

use hbb_common::{config::{self, Config}, log};

pub const PRESET_PASSWORD: &str = "__SOS_PRESET_PASSWORD__";
pub const INCOMING_ONLY: &str = "__SOS_INCOMING_ONLY__";

fn is_placeholder(s: &str) -> bool {
    s.starts_with("__SOS_")
}

/// Appelé au démarrage de chaque processus (interface, service, cm).
pub fn apply() {
    // Vérification par mot de passe permanent seul, non modifiable dans l'interface.
    config::OVERWRITE_SETTINGS
        .write()
        .unwrap()
        .insert("verification-method".to_owned(), "use-permanent-password".to_owned());

    // Pas de compte RustDesk, pas de carnet en ligne ni de groupe : tout est local
    // (serveur libre, sans API). Retire les boutons « Connexion » et l'onglet Compte.
    {
        let mut hard = config::HARD_SETTINGS.write().unwrap();
        hard.insert("disable-account".to_owned(), "Y".to_owned());
        hard.insert("disable-ab".to_owned(), "Y".to_owned());
    }
    {
        let mut local = config::OVERWRITE_LOCAL_SETTINGS.write().unwrap();
        local.insert("disable-group-panel".to_owned(), "Y".to_owned());
        local.insert("enable-check-update".to_owned(), "N".to_owned());
        local.insert("allow-auto-update".to_owned(), "N".to_owned());
    }
    {
        let mut builtin = config::BUILTIN_SETTINGS.write().unwrap();
        builtin.insert("hide-help-cards".to_owned(), "Y".to_owned());
        if INCOMING_ONLY == "Y" {
            // Les proches n'ont pas à toucher au serveur : il est gravé.
            builtin.insert("hide-server-settings".to_owned(), "Y".to_owned());
        }
    }

    // Client des proches : entrant seulement (pas de partie « contrôler »).
    if INCOMING_ONLY == "Y" {
        config::HARD_SETTINGS
            .write()
            .unwrap()
            .insert("conn-type".to_owned(), "incoming".to_owned());
    }

    if PRESET_PASSWORD.is_empty() || is_placeholder(PRESET_PASSWORD) {
        return;
    }

    if INCOMING_ONLY == "Y" {
        // Client des proches : mot de passe permanent préréglé, posé une seule fois
        // si aucun n'existe encore (modifiable ensuite dans Sécurité).
        let (stored, _) = Config::get_local_permanent_password_storage_and_salt();
        if stored.is_empty() {
            if !Config::set_permanent_password(PRESET_PASSWORD) {
                log::error!("SOS: impossible de poser le mot de passe permanent préréglé");
            }
        }
    } else {
        // Client opérateur : la phrase sert de mot de passe de connexion sortante par
        // défaut (option amont « default-connect-password »), essayée avant de demander.
        // Son propre mot de passe entrant reste à fixer par l'opérateur dans Sécurité.
        config::BUILTIN_SETTINGS
            .write()
            .unwrap()
            .insert("default-connect-password".to_owned(), PRESET_PASSWORD.to_owned());
    }
}
