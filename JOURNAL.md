# Journal

## 2026-09-17
- Fait : fork jbfelix/rustdesk ; run #3 amont (préconfiguré par secrets) vert mais client
  standard — les secrets ne sont plus lus en 1.5.0.
- Décidé : gravure par sed sur hbb_common au build ; nom SOS ; icône piste B (SOS + morse) ;
  signature ad hoc dans le workflow ; pas de RustDesk Pro.
- Écarté : hébergement mutualisé/Node Infomaniak (ports bruts impossibles) ; fork séparé de
  hbb_common (le sed suffit).
- Décidé aussi : mot de passe préréglé par secret GitHub, mode permanent forcé, client des proches
  en entrant seulement ; liste blanche IP écartée par défaut (IP publique non garantie stable).
- Reste ouvert : premier build de la branche sos ; réduire la matrice aux cibles utiles
  (macOS arm64, Windows x64) ; intégrer service + mot de passe permanent au déploiement.
- Soir : client proches validé sur le Mac Studio (icône, nom, préréglage, connexion OK). Correctifs :
  import log via hbb_common ; glob du renommage dmg ; préréglage entrant réservé aux proches,
  l'opérateur reçoit la phrase en mot de passe sortant par défaut. Champs Réseau vides = normal
  (surcharge « serveur personnalisé », les valeurs gravées s'appliquent).

## 2026-09-18
- Run #7 opérateur vert (macOS) ; jobs Linux/Android/Win32 rouges, hors périmètre (matrice à réduire).
- Décidé : signature Developer ID + notarisation (JB a l'Apple Developer Program) ; branche signée du
  workflow adaptée à SOS.app, artefact signé publié. Secrets à créer par JB (voir CLAUDE.md).
- Interface épurée : compte, carnet en ligne, groupe et cartes d'aide retirés ; mises à jour
  amont désactivées ; réglages serveur masqués chez les proches.
- Mises à jour automatiques depuis les releases du fork (étiquettes `1.5.0-n`), entrée `version`
  du workflow ; installeur Windows auto-extractible publié. Reste : premier test réel d'une montée
  de version (1.5.0-1 → 1.5.0-2) sur le Studio et sur un Windows.
- Décidé : un seul serveur hbbs pour famille et Actibel, sur le VPS Cloud node ; noms sos.jbfelix.be /
  sos.actibel.be ; le VPS Lite sera résilié (garantie 30 jours). Build 1.5.0-2 = premier avec sos.jbfelix.be.

## 2026-09-22
- Release 1.5.0-1 proches en ligne (signée, notarisée) mais pré-version + assets amont : MAJ auto
  inopérante. Corrigé : prerelease conditionné partout. Cron master en échec chaque nuit → sos → master.
- Décidé : plus de mot de passe préréglé (le client proches sera en téléchargement public sur
  sos.jbfelix.be / sos.actibel.be, un secret gravé y serait lisible). Modèle QuickSupport, mot de
  passe permanent par poste posé par l'opérateur à la première session. Secret SOS_PRESET_PASSWORD
  à supprimer du dépôt GitHub.
- Opérateur 1.5.0-1 (run #16) lancé pour le MacBook. Reste : page de téléchargement nginx sur node.
- 1.5.0-2 proches publiée (latest OK). Bug : hide-help-cards cachait les cartes de permissions macOS →
  démon jamais installé sur le Studio (script installer-service-sos.sh en contournement). Corrigé :
  hide-help-cards retiré, hide-powered-by-me à la place, vérification manuelle des MAJ réactivée,
  matrice réduite à Mac + Windows.
- Démon macOS : sos::apply() n'y tournait pas (service.rs a son propre main) → allow-auto-update non posé,
  + refus « custom client » dans check_update_as_root. Corrigé ; 1.5.0-3 est donc la première version
  dont le démon Mac se met à jour seul — le Studio en 1.5.0-1/-2 doit être monté à la main une dernière fois.

## 2026-09-22 (suite) — 1.5.0-3, À propos
- Fait : release 1.5.0-3 (proches) publiée avec le correctif démon ; opérateur 1.5.0-3 lancé (run #20).
  Studio réinstallé en 1.5.0-3 à la main.
- Constaté : À propos affichait 1.5.0 (version amont) et aucune option de mise à jour : l'amont
  masque tout pour un custom client, et l'ID/clé gravés font de SOS un custom client.
- Fait : patch sos-apropos-maj (voir CLAUDE.md) → étiquette affichée, bouton « Vérifier les mises
  à jour », options réactivées, noms d'assets sos-… dans la MAJ manuelle, lien et mention retirés.
- Reste ouvert : preuve de la MAJ auto Mac (1.5.0-4 après ce patch), Windows non testé,
  sos.actibel.be, page de téléchargement.
- Constaté (MAJ auto Studio 1.5.0-3 → 1.5.0-4) : détection, téléchargement et extraction OK, puis refus
  « staged bundle version mismatch » : le bundle porte la version amont 1.5.0. Correctif workflow :
  CFBundleShortVersionString = étiquette. Nécessite 1.5.0-5 ; 1.5.0-4 restera à installer à la main.
