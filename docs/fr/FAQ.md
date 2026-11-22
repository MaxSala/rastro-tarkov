<p align="center">
  <a href="https://maxsala.github.io/rastro-tarkov/fr/docs/faq/">Site web</a> · <a href="../../README.fr.md">Retour au README</a> · <a href="FEATURES.md">Fonctionnalités</a> · <a href="GUIDE.md">Guide</a> · <b>Questions et sécurité</b>
</p>

# Questions et sécurité

## Est-ce sûr ?

Rastro est un petit programme portable. Voici ce que cela signifie et comment le vérifier.

- **Sans installation ni droits d'administrateur.** C'est un seul `.exe`. Il n'installe ni service ni pilote, ne démarre pas avec Windows et ne touche pas aux fichiers du jeu. Ses réglages et les cartes téléchargées sont dans un seul dossier (voir [Où sont stockées vos données](GUIDE.md#où-sont-stockées-vos-données)) : supprimez ce dossier et le `.exe`, et il a complètement disparu.
- **Open source.** Tout le code est dans ce dépôt. N'importe qui peut le lire.
- **Compilé en public.** Chaque version est compilée par les serveurs de GitHub directement depuis le code source, pas sur le PC de quelqu'un. Vous pouvez voir chaque compilation dans l'onglet [Actions](https://github.com/MaxSala/rastro-tarkov/actions), et la page de chaque version indique le SHA-256 du fichier pour vérifier que vous avez exactement celui-là :

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **N'envoie rien.** Pas de télémétrie, pas de publicité, pas de compte. Il ne se connecte à Internet que quand vous le lui demandez, et la liste complète des connexions est dans [Confidentialité](#confidentialité-et-réseau).

**« Windows a protégé votre ordinateur ».** La première fois que vous ouvrez Rastro, Windows SmartScreen affichera très probablement une fenêtre bleue indiquant qu'il a bloqué une application non reconnue. Cela arrive avec tout nouveau programme qui n'est pas signé avec un certificat payant, pas parce que le fichier pose problème. Cliquez sur **Informations complémentaires**, puis sur **Exécuter quand même**. Windows retient votre choix. Certains antivirus se méfient aussi des nouveaux programmes non signés. Si le vôtre signale Rastro, vous pouvez le restaurer et ajouter une exception, envoyer le fichier sur [VirusTotal](https://www.virustotal.com/) pour un second avis et le signaler comme faux positif à l'éditeur de votre antivirus.

> [!IMPORTANT]
> Rastro est un projet de fans non officiel, sans lien avec Battlestate Games ni tarkov.dev, et non approuvé par eux. Il est conçu pour rester en dehors du jeu : il lit seulement des fichiers sur le disque et n'interagit jamais avec le processus du jeu. Malgré tout, utiliser des outils tiers avec Escape from Tarkov relève de votre responsabilité au regard des conditions d'utilisation de Battlestate Games.

## Questions

**Est-ce que je risque un ban ?**
Rastro ne lit jamais la mémoire du jeu, ne touche jamais au processus du jeu et n'envoie jamais d'entrées au jeu. Il lit les noms des captures et les journaux une fois que le jeu les a écrits, les mêmes fichiers que vous pouvez ouvrir vous-même dans l'Explorateur. Nous ne pouvons cependant pas parler au nom de Battlestate Games : voir la note dans [Est-ce sûr ?](#est-ce-sûr-)

**Ça marche en PvE ?**
Oui. La détection des raids a été testée avec des journaux PvE, et Rastro télécharge les prix et données PvE quand il détecte le PvE.

**Faut-il Internet ?**
Seulement pour télécharger les cartes et les prix, chercher les mises à jour (si vous l'autorisez) et jouer en squad. Tout le reste fonctionne hors ligne.

**Pourquoi des captures d'écran ?**
C'est le seul endroit où le jeu écrit votre position exacte sur le disque. Quelques secondes de décalage, en échange de ne jamais toucher au jeu.

**Linux ou macOS ?**
Rastro se compile et fonctionne sous Linux ; macOS n'est pas testé. Tarkov lui-même ne tourne que sous Windows, donc le téléchargement est réservé à Windows. Voir [Compiler depuis les sources](GUIDE.md#compiler-depuis-les-sources).

## Confidentialité et réseau

Rastro n'a ni compte, ni télémétrie, ni publicité. L'interface ne parle qu'à l'application elle-même sur `127.0.0.1`, protégée par une clé aléatoire qui change à chaque lancement. Tout ce qu'il lit sur votre disque reste sur votre PC. Le reste n'arrive que si vous le demandez :

| Quand | Se connecte à |
|---|---|
| Toujours | Rien en dehors de votre PC |
| Vous téléchargez des cartes depuis tarkov.dev | `raw.githubusercontent.com`, `json.tarkov.dev` et, en secours, `api.tarkov.dev` |
| Vous téléchargez ou actualisez les prix | `json.tarkov.dev`, plus `assets.tarkov.dev` pour les icônes des objets |
| Vous téléchargez ou actualisez les quêtes | `json.tarkov.dev` et, en secours, `api.tarkov.dev`, plus `assets.tarkov.dev` pour les images |
| Vous importez un pack de cartes depuis une URL | Uniquement l'adresse saisie (HTTPS seulement, jamais d'adresse interne) |
| Vous hébergez ou rejoignez un groupe | Directement les PC de vos coéquipiers |
| Une fois par lancement, si vous avez accepté la recherche de mises à jour (ou avec *Vérifier maintenant*) | `api.github.com`, pour lire le numéro de la dernière version. Rien vous concernant n'est envoyé. |

Chaque serveur de téléchargement figure sur une liste fixe d'autorisations, vérifiée aussi lors des redirections, et tout ce qui est téléchargé est validé avant usage. Le modèle de menace complet est dans [docs/SECURITY.md](../SECURITY.md) (en anglais).

## Limites connues

- Votre position n'est mise à jour que lorsque vous faites une capture. C'est voulu : c'est le seul endroit où le jeu écrit votre position.
- Labs n'a pas de carte SVG sur tarkov.dev. Rastro le reconnaît, mais ne peut l'afficher que si vous importez votre propre pack de cartes.
- La détection des raids a été vérifiée avec des journaux PvE. Le PvP devrait se comporter pareil, mais ce n'est pas encore confirmé avec de vrais journaux.
- Les traductions espagnole et anglaise sont relues. Les 14 autres n'ont pas encore été vérifiées par des natifs. Les corrections sont les bienvenues, ici ou sur [Discord](https://discord.gg/H4pRxGBGDb).
- Après un changement de langue ou de mode de jeu (PvP/PvE), il faut retélécharger les cartes et les prix pour voir les nouveaux noms.
- Certains détails techniques des messages d'erreur et du journal restent en anglais quelle que soit la langue choisie.

## Non commercial

Rastro est un projet amateur fait pour la communauté. Il est gratuit, il n'existe pas de version payante, il n'affiche aucune publicité et ne collecte aucune donnée. Cela restera ainsi, par choix et parce que les cartes de tarkov.dev sur lesquelles il s'appuie sont sous licence non commerciale.
