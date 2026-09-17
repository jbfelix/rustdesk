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
