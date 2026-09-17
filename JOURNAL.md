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
