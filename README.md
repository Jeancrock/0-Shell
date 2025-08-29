# 0-Shell

0-Shell est un **shell minimaliste écrit en Rust**, offrant les commandes de base ainsi que la gestion des **jobs en arrière-plan**, un **prompt personnalisé**, et un **historique des commandes**.

---

## Fonctionnalités principales

* Prompt dynamique : `username@hostname:cwd$`
* Historique des commandes
* Couleurs ANSI pour différencier dossiers, fichiers et exécutables
* Commandes intégrées :

| Commande                         | Description                                          |
| -------------------------------- | ---------------------------------------------------- |
| `help`                           | Affiche l’aide et la liste des commandes disponibles |
| `exit`                           | Quitte le shell                                      |
| `echo [texte][[>/>>] [fichier]]` | Affiche du texte                                     |
| `cd [chemin]`                    | Change le répertoire courant                         |
| `pwd`                            | Affiche le répertoire courant                        |
| `ls [-a] [-F] [-l] [-r] [-R]`    | Liste fichiers et dossiers                           |
| `cat <fichier>`                  | Affiche le contenu d’un fichier                      |
| `cp <source> <destination>`      | Copie un fichier ou dossier                          |
| `mv <source> <destination>`      | Déplace ou renomme un fichier ou dossier             |
| `rm [-r] <fichier/dossier>`      | Supprime un fichier ou dossier                       |
| `mkdir [-p] <dossier>`           | Crée un nouveau dossier                              |
| `clear`                          | Efface l’écran                                       |
| `history`                        | Affiche l’historique des commandes                   |
---

## Installation & Lancement

`./install.sh`
`./0-Shell.sh`

## Prompt

Le prompt est dynamique et affiché sous la forme :

```
username@hostname:cwd$
```

* `username` : nom de l’utilisateur courant
* `hostname` : nom de la machine
* `cwd` : répertoire courant (le home est remplacé par `~`)
* Couleurs ANSI :

  * Bleu → dossiers
  * Vert → fichiers exécutables
  * Blanc → fichiers ordinaires

---

## Commandes détaillées

### Commandes de navigation et fichiers

* **`cd [dossier présent dans le répertoire courant]`** : change le répertoire courant
* **`cd ..`** remonte d’un dossier
* **`cd [chemin absolu]`**. Exemple : **`cd /home/student`**

* **`pwd`** : affiche le chemin du répertoire courant

* **`ls [-l] [-a] [-F]`** : liste fichiers et dossiers

  * `ls -a` : inclut les fichiers cachés
  * `ls -F` : ajoute un symbole pour le type (`/` pour dossiers, `*` pour exécutables)
  * `ls -l` : affichage détaillé (permissions, propriétaire, taille, date)
  * `ls -r` : inverse l'ordre d'affichage
  * `ls -R` : inclut le contenu des sous-dossiers de façon récursive
  * 

* **`cat <fichier>`** : affiche le contenu d’un fichier

* **`cp <source> <destination>`** : copie un fichier ou dossier

* **`mv <source> <destination>`** : déplace ou renomme un fichier ou dossier

* **`rm [-r] <fichier/dossier>`** : supprime un fichier ou dossier

  * `rm -r` pour suppression récursive de dossiers

* **`mkdir <dossier>`** : crée un nouveau dossier
  * `mkdir -p <dossier1> <dossier2>` : créé plusieurs dossiers

### Commandes utilitaires

* **`echo [texte]`** : affiche du texte
  * `echo [texte] > <chemin/fichier>` : ecrase le texte du fichier avec le texte de la commande
  * `echo [texte] >> <chemin/fichier>` : ajoute le texte de la commande dans le fichier

* **`clear`** : efface l’écran
* **`history`** : affiche l’historique des commandes valides
* **`help`** : affiche l’aide et la liste des commandes
* **`exit`** : quitte le shell
