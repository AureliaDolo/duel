## Installer Rust 

voir https://www.rust-lang.org/tools/install

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Rustup est le gestionnaire de toolchain (stable, beta, nightly, etc)

[Cargo](https://doc.rust-lang.org/cargo/) est le gestionnaire de paquet

`cargo build` pour compiler

`cargo run` pour lancer le programme

## Moteur utilisé
[Macroquad](https://github.com/not-fl3/macroquad)


Dépendances à installer

```
# ubuntu system dependencies
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```

## Anatomie d'un projet

### Cargo.toml
Contient les informations nécessaire pour générer le binaire, en particulier les dépendances mais aussi les auteurs et l'édition du langage.

Cargo télécharge et compile les dépendances sans soucis 🦀


### Cargo.lock
Auto généré à la compilaition, contient la liste de toutes les dépendances et leur dépendances etc avec un numéro de version et un hash.

### src/
Contient le code source

### src/main.rs
Point d'entrée du code. Pour l'instant contient tout le code !

## Idée générale
On a un squelette de projet, il reste plus qu'a ajouter quelques fonctionnalités pour ajouter du fun !

- ajouter une fin au niveau (et des niveaux suivants)
- ajouter des plateformes (qui bougent)
- agrandir le niveau et faire scroller la camera
- Ajuster les constantes qui gèrent la vitesse et le saut
- Ajouter un deuxième player
- Ajouter des ennemies
- Ajouter des armes
- Ajouter des teleporteurs
- Utiliser des sprites
- Modifier la gestion des sauts (pas de changement de direction une fois en l'air, rebond, double saut, ..)
