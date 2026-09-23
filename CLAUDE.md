# SOS — client de prise en main à distance (fork RustDesk)

Projet personnel de JB. Fork de rustdesk/rustdesk (AGPL), branche `sos`.

## Ce que fait la branche `sos`
- Grave serveur, clé et nom au build : étape « SOS — graver … » dans
  `.github/workflows/flutter-build.yml` (jobs `build-for-windows-flutter` et
  `build-for-macOS`), par `sed` sur `libs/hbb_common/src/config.rs`
  (sous-module, donc pas modifiable par commit ici) :
  `APP_NAME` = SOS, `RENDEZVOUS_SERVERS` = sos.jbfelix.be, `RS_PUB_KEY` = clé du hbbs.
- macOS : `PRODUCT_NAME = SOS` (bundle SOS.app, exigé par le code Rust qui cherche
  `/Applications/{app_name}.app`), signature ad hoc du bundle avant le dmg.
- Icônes : `res/*`, `flutter/assets/icon.svg` (tracés, pas de police),
  `flutter/macos/Runner/AppIcon.icns`, `flutter/windows/runner/resources/app_icon.ico`.
- Windows : l'installeur renomme rustdesk.exe en SOS.exe à l'installation (code amont).

## Serveur
- hbbs/hbbr en Docker dans /opt/rustdesk sur le VPS Cloud « node » (83.228.217.121), commun famille + Actibel.
  Noms DNS : sos.jbfelix.be (gravé dans le build SOS), sos.actibel.be (futur build Actibel),
  remote.jbfelix.be (transitoire, clients < 1.5.0-2). Le VPS Lite 179.237.67.146 (`remote-jbf`) a servi
  du 17 au 18/9/2026 puis a été résilié (remboursement 30 jours).
- Ports 21115-21119/tcp + 21116/udp : à ouvrir dans ufw ET dans le pare-feu Infomaniak du VPS.

## Build
- Actions → « Flutter Nightly Build » → Run workflow sur la branche `sos`.
- Noms (23/9) : fichiers `sos-client-<étiquette>-<arch>.dmg|exe` (release, proches) et
  `sos-operator-<étiquette>-<arch>.dmg|exe` (artefacts seulement) ; artefacts
  `sos-{client|operator}-macos-<arch>`, `sos-{client|operator}-windows-<arch>[-installeur]`
  (env `SOS_KIND`). L'updater (src/updater.rs, flutter_ffi download-file) cherche
  `sos::RELEASE_FILE_PREFIX` = sos-client. Transition : la release publie AUSSI l'ancien nom
  `sos-<étiquette>-<arch>` pour les clients ≤ 1.5.0-5 (Studio) — étapes « alias ancien nom »,
  à retirer après 1.5.0-6.
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
- Aucun secret gravé (décision du 22/9/2026) : le client public est distribuable librement.
  Modèle QuickSupport : verification-method = use-both-passwords ; à la première session le
  proche lit ID + code temporaire, l'opérateur pose ensuite un mot de passe permanent propre au
  poste (Sécurité, sur l'écran du proche) et le mémorise dans son client.
- Le déclenchement planifié (cron nocturne) a été retiré : builds manuels seulement.
- `prerelease` conditionné à `nightly` sur TOUTES les publications (les étapes amont republient
  sur le même tag) : sans ça `releases/latest` ne voit rien et la MAJ auto ne part jamais.

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
  `disable-group-panel` (local), `hide-powered-by-me` (builtin). JAMAIS `hide-help-cards` : il
  masque aussi les cartes de permissions macOS, et la carte « Installer le service » n'apparaît
  qu'après elles (constaté le 22/9 : Studio sans démon). Les favoris et sessions récentes
  restent (locaux). Vérification manuelle des mises à jour active (elle interroge le fork, voir plus bas).
- Proches : `hide-server-settings` en plus (serveur gravé, rien à saisir).

## Mises à jour automatiques (releases du fork)
- `sos::apply()` doit tourner dans CHAQUE processus : core_main (interface, --server, --service
  Windows) ET src/service.rs (démon macOS, entrée séparée) — oubli constaté le 22/9 : le démon
  disait « Auto update is disabled ». `check_update_as_root` ignore aussi le refus « custom client »
  quand BUILD est gravé.
- Le client (src/sos.rs `BUILD`) connaît l'étiquette de la release qui l'a produit. Le service
  (Windows : rendezvous_mediator → updater ; macOS : service root) interroge chaque jour
  `https://api.github.com/repos/jbfelix/rustdesk/releases/latest` (src/common.rs
  `sos_check_software_update`), compare les étiquettes (`get_version_number` : `1.5.0-3` > `1.5.0-2`
  > `1.5.0`), télécharge `sos-<étiquette>-<arch>.dmg|.exe` depuis la release et l'installe.
  Allowlist de téléchargement : rustdesk/rustdesk OU jbfelix/rustdesk (src/updater.rs).
- Build de release : « Run workflow » avec `version` = étiquette (ex. `1.5.0-1`), proches d'abord
  (publie dmg + exe dans la release, non pré-release), puis opérateur (artefacts seulement).
  `nightly` = build de test : BUILD reste un placeholder → pas de mise à jour automatique.
- Schéma d'étiquettes : `<version amont>-<n>` ; n augmente à chaque build SOS sur la même version
  amont, repart à 1 après un rebase sur une nouvelle version amont.
- Windows : l'auto-update lance l'installeur téléchargé ; exe non signé → à vérifier au premier
  passage (SmartScreen peut bloquer un lancement silencieux).
- Artefacts Windows : `sos-windows-<arch>-<rôle>-installeur` = exe auto-extractible à distribuer
  (le dossier `sos-windows-<arch>-<rôle>` est la version dépliée, pour débogage).

## Cibles construites
- Seuls macOS (arm64, x86_64) et Windows (x64, ARM64) sont construits ; les autres jobs de
  flutter-build.yml portent `if: false  # SOS : hors périmètre` (Linux, Android, iOS, web,
  Windows 32 bits sciter, publish_unsigned). Sans eux, la release ne reçoit que les assets SOS.
- Service macOS à la main si la carte ne s'affiche pas : `sudo bash installer-service-sos.sh`
  (reproduit install.scpt ; journal du démon : /var/log/sos_service.out|err).

## À propos / vérification manuelle (22/9, patch sos-apropos-maj)
- `main_get_version` renvoie l'étiquette gravée (1.5.0-3) quand elle existe ; sinon la version amont.
- FFI `sos_is_release()` : tout ce que l'amont cache aux « custom clients » (case « Vérifier au
  démarrage », case « Mise à jour automatique » macOS, carte « nouvelle version » de l'accueil,
  `checkUpdate()` au lancement) est réactivé quand le build est étiqueté.
- FFI `sos_check_update_now()` : bouton « Vérifier les mises à jour » dans À propos (interroge le
  fork tout de suite, sans passer par l'option de démarrage) ; puis « Mettre à jour » = handleUpdate.
- `download-file-<version>` (mise à jour depuis l'interface) nomme les assets `sos-…` et non `rustdesk-…`.
- Lien sos.jbfelix.be et mention AGPL retirés de À propos à la demande de JB (la mention reste
  dans le dépôt : README/LICENSE du fork).
- macOS, MAJ auto : le démon (src/platform/macos.rs) compare `CFBundleShortVersionString` du bundle
  téléchargé à l'étiquette attendue → l'étape « SOS — étiquette de release dans Info.plist » grave
  l'étiquette avant signature (constaté le 22/9 : « staged bundle version mismatch: expected 1.5.0-4,
  found 1.5.0 » ; la 1.5.0-4 n'est donc PAS installable automatiquement, la 1.5.0-5 sera la première).
- Opérateur : PAS de mise à jour automatique ni de bouton « Vérifier » (`sos::update_source()` = None
  hors client des proches) : la release ne contient que le client des proches, un opérateur qui s'y
  mettrait à jour deviendrait « entrant seul » (arrivé le 22/9 sur le MacBook). L'opérateur se met à
  jour à la main depuis l'artefact `sos-macos-aarch64-operateur` / `sos-windows-…-operateur-installeur`.
