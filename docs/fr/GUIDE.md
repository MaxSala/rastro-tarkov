<p align="center">
  <a href="../../README.fr.md">Retour au README</a> · <a href="FEATURES.md">Fonctionnalités</a> · <b>Guide</b> · <a href="FAQ.md">Questions et sécurité</a>
</p>

# Guide

## Installation

1. **[Téléchargez `Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**. Ce lien donne toujours la dernière version.
2. Placez-le où vous voulez : sur le bureau, dans un dossier de Documents, n'importe où. Il n'a besoin d'aucun autre fichier à côté.
3. Double-cliquez dessus. Si Windows affiche un avertissement bleu, voir [Est-ce sûr ?](FAQ.md#est-ce-sûr-)

Nécessite Windows 10 ou 11, 64 bits.

### Premier lancement

Un court guide s'ouvre la première fois :

1. **Dossier des captures.** Rastro trouve tout seul `Documents\Escape from Tarkov\Screenshots`, y compris quand Windows a déplacé Documents dans OneDrive. S'il n'y arrive pas, collez le chemin et il vous dira combien de captures avec coordonnées il voit. Vous pouvez aussi activer ici la détection des raids.
2. **Cartes.** Téléchargez les cartes de la communauté depuis tarkov.dev (recommandé), importez un pack de cartes ou essayez la carte de démonstration.
3. C'est prêt. Faites une capture en jeu et votre flèche apparaît sur la carte.

Pour utiliser le Marché, le tableau des Munitions et les noms des clés, ouvrez une fois l'onglet Marché et cliquez sur *Télécharger les prix*.

### Mise à jour

Si vous avez autorisé la recherche de mises à jour, un bouton apparaît dans la barre du haut quand une nouvelle version sort et vous mène à sa page de téléchargement. Sinon, téléchargez le nouveau `.exe` avec le [lien ci-dessus](#installation), fermez Rastro et utilisez le nouveau fichier à la place de l'ancien. Vos réglages, points, cartes et prix sont conservés [à part](#où-sont-stockées-vos-données), rien n'est perdu.

Après une mise à jour, Rastro vous montre ce qui a changé. L'historique complet se trouve dans *Réglages → À propos → Nouveautés* et dans [CHANGELOG.md](../../CHANGELOG.md). Si une mise à jour apporte de nouvelles données de carte, utilisez *Mettre à jour depuis tarkov.dev* dans le panneau Cartes.

### Raccourcis

| Action | Comment |
|---|---|
| Mettre à jour votre position | Faire une capture en jeu |
| Changer de section (Carte, Marché, Munitions, Quêtes, Planque) | `Ctrl+1`, `Ctrl+2`, `Ctrl+3`, `Ctrl+4`, `Ctrl+5` |
| Palette de commandes | `Ctrl+K` |
| Panneaux extractions, points, groupe, cartes et quêtes | `1`, `2`, `3`, `4`, `5` |
| Nouveau raid (effacer trace et marques) | `N` |
| Voir toute la carte / suivre votre position | `F` / `G` |
| Zoom | `+` / `-` ou la molette |
| Ajouter un point, guider vers un endroit | Clic droit sur la carte |
| Pinguer votre squad | `Maj` + clic |
| Chercher dans le Marché | `/` |
| Réglages | `,` |

### Où sont stockées vos données

Les réglages, vos points, votre progression de quêtes, les cartes téléchargées et les caches des prix et des quêtes sont rangés dans un dossier, séparé du `.exe` :

| Système | Dossier |
|---|---|
| Windows | `%APPDATA%\Rastro\data\` |
| Linux | `~/.local/share/rastro/` |
| macOS | `~/Library/Application Support/Rastro/` |

*Réglages → Journal* propose un bouton pour l'ouvrir.

### Si quelque chose ne marche pas

- **Il s'ouvre dans le navigateur au lieu de sa propre fenêtre.** Rastro utilise Microsoft WebView2 pour afficher sa fenêtre. Il est fourni avec Windows 10 et 11, mais s'il a été supprimé, Rastro bascule sur votre navigateur. Installer le [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) ramène la fenêtre.
- **Une page d'erreur indique que le port est déjà utilisé.** Une autre copie de Rastro tourne sûrement encore. Fermez-la depuis la barre des tâches ou le Gestionnaire des tâches (cherchez `rastro.exe`) et relancez.
- **Ma position ne bouge jamais.** Vérifiez que le dossier des captures dans *Réglages → Général* est bien celui où le jeu enregistre, et que vous appuyez sur la touche de capture du jeu.
- **Mes coéquipiers n'arrivent pas à rejoindre mon salon.** Ils doivent pouvoir joindre votre PC. Utilisez l'adresse de votre VPN mesh (Tailscale, ZeroTier) en ouvrant le salon, ou ouvrez le port du groupe sur votre box.
- **Toujours bloqué ?** Demandez sur [Discord](https://discord.gg/H4pRxGBGDb). *Réglages → Journal* a un bouton *Copier*, et coller ce journal aide beaucoup.

## Comment ça marche

Rastro lit deux types de fichiers qu'Escape from Tarkov enregistre déjà sur votre PC. Il n'a besoin de rien d'autre.

| Ce qu'il lit | Où | Ce que Rastro en tire |
|---|---|---|
| **Les noms de vos captures d'écran** | `Documents\Escape from Tarkov\Screenshots` | Votre position et la direction de votre regard. Tarkov écrit vos coordonnées dans le nom de chaque capture qu'il enregistre. Rastro lit le nom, jamais l'image, et ne déplace, ne modifie ni ne supprime jamais vos captures. |
| **Les journaux du jeu** (facultatif, désactivé tant que vous ne l'activez pas) | `Battlestate Games\Escape from Tarkov\Logs` | Quelle carte se charge, quand un raid commence et si vous jouez en PvP ou en PvE. Rastro s'en sert pour passer tout seul à la bonne carte, démarrer une nouvelle trace à chaque raid et afficher le temps de raid restant. Seuls le nom de la carte et le mode de jeu sont conservés. Il lit aussi les messages des marchands, pour cocher les quêtes que vous acceptez, rendez ou ratez. |

En pratique : vous appuyez sur la touche de capture en jeu, et une seconde plus tard votre flèche bouge sur la carte.

Ce que Rastro **ne fait pas** : il ne lit pas la mémoire du jeu, n'injecte rien, ne s'accroche pas au processus du jeu, ne capture pas le trafic réseau, ne dessine pas par-dessus le jeu et n'appuie sur aucune touche à votre place. C'est un programme séparé qui regarde seulement des fichiers sur le disque.

## Compiler depuis les sources

Ce n'est utile que pour modifier le code ou compiler le `.exe` vous-même. Il vous faut [Rust](https://rustup.rs/) 1.85 ou plus récent et [Node.js](https://nodejs.org/) LTS (sous Linux, aussi `libwebkit2gtk-4.1-dev` et `libgtk-3-dev`) :

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # build.bat sous Windows
```

Le résultat est un seul exécutable dans `target/release/`. L'environnement de développement, l'organisation du projet, les règles de code et l'ajout d'une traduction sont décrits dans **[CONTRIBUTING.md](../../CONTRIBUTING.md)** (en anglais).
