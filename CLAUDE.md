# SOS — client de prise en main à distance (fork RustDesk)

Projet personnel de JB. Fork de rustdesk/rustdesk (AGPL), branche `sos`.

## Ce que fait la branche `sos`
- Grave serveur, clé et nom au build : étape « SOS — graver … » dans
  `.github/workflows/flutter-build.yml` (jobs `build-for-windows-flutter` et
  `build-for-macOS`), par `sed` sur `libs/hbb_common/src/config.rs`
  (sous-module, donc pas modifiable par commit ici) :
  `APP_NAME` = SOS, `RENDEZVOUS_SERVERS` = remote.jbfelix.be, `RS_PUB_KEY` = clé du hbbs.
- macOS : `PRODUCT_NAME = SOS` (bundle SOS.app, exigé par le code Rust qui cherche
  `/Applications/{app_name}.app`), signature ad hoc du bundle avant le dmg.
- Icônes : `res/*`, `flutter/assets/icon.svg` (tracés, pas de police),
  `flutter/macos/Runner/AppIcon.icns`, `flutter/windows/runner/resources/app_icon.ico`.
- Windows : l'installeur renomme rustdesk.exe en SOS.exe à l'installation (code amont).

## Serveur
- VPS Lite Infomaniak 179.237.67.146 (alias ssh `remote-jbf`), hbbs/hbbr Docker dans /opt/rustdesk.
- Ports 21115-21119/tcp + 21116/udp : à ouvrir dans ufw ET dans le pare-feu Infomaniak du VPS.

## Build
- Actions → « Flutter Nightly Build » → Run workflow sur la branche `sos`.
- Artefacts : `sos-macos-aarch64` (dmg), `sos-windows-x86_64` (dossier exe).
- Workflow permissions du dépôt en « Read and write » (job generate-sbom).

## Pièges rencontrés
- La recette « secrets RENDEZVOUS_SERVER / RS_PUB_KEY » de la doc RustDesk ne marche
  plus en 1.5.0 : plus lu par le workflow ni par le code. D'où le sed.
- Artefact macOS amont : plante au lancement (Team ID du framework Flutter ≠ binaire),
  réglé par la signature ad hoc.
- Clé serveur ≠ clé SSH : 44 caractères finissant par `=`.

## Réglages gravés supplémentaires (src/sos.rs, appelé au démarrage dans core_main)
- Vérification par mot de passe permanent seul (OVERWRITE_SETTINGS verification-method).
- Entrée de workflow `sos-incoming-only` (défaut : vrai) → client des proches sans partie
  « contrôler » (HARD_SETTINGS conn-type=incoming). Décocher pour construire le client
  opérateur de JB. Artefacts suffixés `-proches` / `-operateur`.
- Secret GitHub `SOS_PRESET_PASSWORD` injecté dans src/sos.rs au build (jamais dans le dépôt) :
  client proches → mot de passe permanent entrant, posé une seule fois si aucun n'existe
  (modifiable ensuite dans Sécurité) ; client opérateur → mot de passe de connexion sortante
  par défaut (option amont default-connect-password), essayé avant de demander ; l'entrant de
  l'opérateur reste à fixer par lui dans Sécurité. Sans secret : rien.
- Le déclenchement planifié (cron nocturne) a été retiré : builds manuels seulement.

## Signature Apple (Developer ID + notarisation)
Quatre secrets GitHub, tous produits par JB sur son Mac (jamais dans le dépôt ni le chat) :
- `MACOS_P12_BASE64` : certificat « Developer ID Application » exporté en .p12, encodé base64.
- `MACOS_P12_PASSWORD` : mot de passe de ce .p12.
- `MACOS_CODESIGN_IDENTITY` : l'identité ENTRE GUILLEMETS, ex. `"Developer ID Application: Nom (TEAMID)"`
  (le workflow l'insère telle quelle dans le shell).
- `APPLE_ID`, `APPLE_TEAM_ID` (2B7S7GBCSU), `APPLE_APP_PASSWORD` : les mêmes identifiants que le
  profil notarytool « rops » de StockAAV/desktop (recette commune à toutes les apps macOS de JB).
Dès que `MACOS_P12_BASE64` existe, le job macOS signe (hardened runtime, entitlements
Release.entitlements), notarise et agrafe le dmg ; l'artefact `sos-macos-<arch>-<rôle>` contient
`sos-<version>-<arch>-signe.dmg`. Sans ces secrets : signature ad hoc + « Ouvrir quand même ».

## Interface épurée (src/sos.rs)
- Sans serveur API, rien à « connecter » : `disable-account`, `disable-ab` (HARD_SETTINGS),
  `disable-group-panel` (local), `hide-help-cards` (builtin). Les favoris et sessions récentes
  restent (locaux). Vérification de mise à jour désactivée (elle pointerait vers rustdesk.com).
- Proches : `hide-server-settings` en plus (serveur gravé, rien à saisir).
