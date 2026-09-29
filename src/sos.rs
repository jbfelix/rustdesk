//! SOS — réglages gravés au build (voir notes de projet à la racine).
//! Les constantes ci-dessous sont remplacées par le workflow avant compilation ;
//! laissées telles quelles (build local), rien n'est appliqué.
//! Aucun secret n'est gravé : le client public est distribuable librement.

use hbb_common::config;

pub const INCOMING_ONLY: &str = "__SOS_INCOMING_ONLY__";
/// Édition gravée : proches | operateur | actibel (serveur sos.actibel.be, déployée par MSI/Intune).
pub const EDITION: &str = "__SOS_EDITION__";
/// Étiquette de la release GitHub qui a produit ce binaire (ex. 1.5.0-3) ; placeholder = build de test.
pub const BUILD: &str = "__SOS_BUILD__";
/// Dépôt dont les releases servent de source de mise à jour.
pub const UPDATE_REPO: &str = "jbfelix/rustdesk";
/// Préfixe des assets de release selon l'édition (workflow SOS_KIND) :
/// client → sos-client-<étiquette>-<arch>.<dmg|exe>, opérateur → sos-<étiquette>-<arch>.<dmg|exe>.
pub fn release_file_prefix() -> &'static str {
    if INCOMING_ONLY == "Y" { "sos-client" } else { "sos" }
}

/// Étiquette de build si elle a été gravée (sinon None : pas de mise à jour automatique).
pub fn build() -> Option<&'static str> {
    if BUILD.is_empty() || is_placeholder(BUILD) { None } else { Some(BUILD) }
}

/// Source de mise à jour utilisable ? Client et opérateur (depuis 1.5.0-6 la release publie les
/// deux, chacun sous son préfixe — avant, un opérateur mis à jour devenait « entrant seul »,
/// constaté le 22/9 sur le MacBook). Jamais l'édition Actibel : Intune est maître.
pub fn update_source() -> Option<&'static str> {
    if !is_actibel() { build() } else { None }
}

/// Édition Actibel : postes gérés par Intune, montées de version par MSI, jamais par l'updater.
pub fn is_actibel() -> bool {
    EDITION == "actibel"
}

fn is_placeholder(s: &str) -> bool {
    s.starts_with("__SOS_")
}

/// Appelé au démarrage de chaque processus (interface, service, cm).
pub fn apply() {
    // Modèle QuickSupport : code temporaire affiché sous l'ID pour la première session,
    // mot de passe permanent propre au poste posé ensuite par l'opérateur (Sécurité).
    config::OVERWRITE_SETTINGS
        .write()
        .unwrap()
        .insert("verification-method".to_owned(), "use-both-passwords".to_owned());

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
        local.insert("enable-check-update".to_owned(), "Y".to_owned());
    }
    // Mise à jour automatique (service Windows, service macOS) depuis les releases du fork,
    // seulement pour un binaire issu d'une release étiquetée.
    config::OVERWRITE_SETTINGS.write().unwrap().insert(
        "allow-auto-update".to_owned(),
        if update_source().is_some() { "Y" } else { "N" }.to_owned(),
    );
    {
        let mut builtin = config::BUILTIN_SETTINGS.write().unwrap();
        // « Utilise la technologie RustDesk » sur l'accueil : la mention légale est dans À propos.
        builtin.insert("hide-powered-by-me".to_owned(), "Y".to_owned());
        // hide-help-cards NE DOIT PAS être posé : il masque aussi les cartes de permissions
        // macOS, et la carte « Installer le service » n'apparaît qu'après elles (bug du 22/9).
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
}
