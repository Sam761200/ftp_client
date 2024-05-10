# Mon Client FTP

Ce projet est un client FTP simple écrit en Rust. Il permet de se connecter à un serveur FTP et d'afficher un message de connexion réussie. Ce projet utilise la bibliothèque rust-ftp pour gérer les connexions FTP de manière asynchrone avec tokio.

## Prérequis

Assurez-vous d'avoir installé Rust et Cargo sur votre machine. Vous pouvez les installer via rustup.

## Installation

Pour commencer, clonez ce dépôt sur votre machine locale :

```bash
git clone https://github.com/Sam761200/ftp_client.git
cd mon_client_ftp
```

Ensuite, construisez le projet avec Cargo :

```bash
cargo build
```

## Configuration

Avant de lancer le client, assurez-vous de connaître les informations de connexion au serveur FTP que vous allez utiliser, notamment l'adresse du serveur, le port, le nom d'utilisateur et le mot de passe.

## Utilisation

Pour exécuter le client FTP, utilisez la commande suivante dans le terminal :

```bash
cargo run
```

Cela établira une connexion avec le serveur FTP spécifié dans le code et affichera un message en cas de connexion réussie. Vous devez modifier le fichier src/main.rs pour configurer les paramètres de connexion au serveur FTP que vous souhaitez utiliser.

## Dépendances

Ce projet dépend des crates suivantes, qui sont ajoutées dans le fichier Cargo.toml :

ftp pour la gestion des connexions FTP.
tokio pour la programmation asynchrone.
Pour installer les dépendances, Cargo s'en chargera automatiquement lors de la construction ou de l'exécution du projet.

## Licence

Ce projet est distribué sous la licence MIT. Voir le fichier LICENSE pour plus de détails.
