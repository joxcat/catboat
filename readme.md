# catboat
Juste un bot

---

Un bot simple (ou pas) pour Discord, qui sert à tout et en même temps pas grand chose.

---

## Prérequis
```
rustc: 1.46.0
cargo: 1.46.0

pkg-config: 0.29-6 (debian)
libssl-dev: 1.1.1d (debian)
```

## Pour le lancer

Créer le fichier `.env` dans votre dossier courant avec dedans :
```
DISCORD_TOKEN=<votre token de bot>
RUST_LOG=warn
```

*`RUST_LOG` peut être dans l'ordre d'importance : DEBUG, INFO, WARN, ERROR* 

### Méthode 1 - Version production
1. `cargo build --release`
2. `./target/release/catboat`

### Méthode 2 - En tache de fond
1. `./start.sh`

### Méthode 3 - Version de dev
1. `cargo run`