# 0-Shell

0-Shell est un **shell minimaliste écrit en Rust**, offrant les commandes de base (`ls`, `cd`, `pwd`, `echo`, `cat`, `cp`, `mv`, `rm`, `mkdir`, `clear`) avec un **prompt personnalisé** et un **historique des commandes**.

---

## Fonctionnalités

* Prompt dynamique : `username@hostname:cwd$`
* Historique des commandes
* Couleurs ANSI pour différencier dossiers, fichiers et exécutables
* Commandes intégrées :

| Commande                     | Description                                          |
| ---------------------------- | ---------------------------------------------------- |
| `help`                       | Affiche l’aide et la liste des commandes disponibles |
| `exit`                       | Quitte le shell                                      |
| `echo [texte]`               | Affiche du texte                                     |
| `cd [chemin]`                | Change le répertoire courant                         |
| `pwd`                        | Affiche le répertoire courant                        |
| `ls [-l] [-a] [-F] [chemin]` | Liste fichiers et dossiers                           |
| `cat <fichier>`              | Affiche le contenu d’un fichier                      |
| `cp <source> <cible>`        | Copie un fichier ou dossier                          |
| `mv <source> <cible>`        | Déplace ou renomme un fichier ou dossier             |
| `rm [-r] <chemin>`           | Supprime un fichier ou dossier                       |
| `mkdir <dossier>`            | Crée un nouveau dossier                              |
| `clear`                      | Efface l’écran                                       |
| `history`                    | Affiche l’historique des commandes                   |

---

## Installation

1. Installer Rust : [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Cloner le dépôt :

   ```bash
   git clone https://github.com/votre-utilisateur/0-shell.git
   cd 0-shell
   ```
3. Compiler le shell :

   ```bash
   cargo build --release
   ```
4. Lancer le shell :

   ```bash
   ./target/release/0-shell
   ```

---

## Usage et options des commandes

### `help`

Affiche la liste des commandes et leur description.

```bash
help
```

### `exit`

Quitte le shell.

```bash
exit
```

### `echo [texte]`

Affiche du texte sur la sortie standard.

```bash
echo "Hello World"
```

### `cd [chemin]`

Change le répertoire courant.

* `cd ..` : remonte d’un dossier

### `pwd`

Affiche le chemin du répertoire courant.

```bash
pwd
```

### `ls [-l] [-a] [-F]`

Liste les fichiers et dossiers.

* `-l` : affichage détaillé (permissions, propriétaire, taille, date)
* `-a` : affiche les fichiers cachés
* `-F` : ajoute un symbole pour le type de fichier (`/` pour dossiers, `*` pour exécutables)

```bash
ls -laF
```

* **Dossiers** : bleu
* **Fichiers exécutables** : vert
* **Fichiers ordinaires** : blanc

### `cat <fichier>`

Affiche le contenu d’un fichier texte.

```bash
cat fichier.txt
```

### `cp <fichier> <destination>`

Copie un fichier.

* Exemple :

```bash
cp fichier.txt ./destination
```


### `mv <source> <destination>`

Déplace un fichier ou dossier.

```bash
mv fichier.txt ./destination
mv dossier1 dossier2
```

### `rm [-r] <fichier/dossier>`

Supprime un fichier ou dossier.

* `-r` : suppression récursive pour dossiers

```bash
rm fichier.txt
rm -r dossier
```

### `mkdir <dossier>`

Crée un nouveau dossier.

```bash
mkdir mon_dossier
```

### `clear`

Efface l’écran.

```bash
clear
```

### `history`

Affiche l’historique des commandes valides entrées dans le shell depuis son lancement.

```bash
history
```

---