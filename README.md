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