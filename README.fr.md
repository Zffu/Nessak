<h1 style="text-align:center;">Nessak & structure Tundra</h1>
<h2 style="text-align:center;">Zffu - Août 2026</h2>

Cela introduit une nouvelle famille de fonctions de hachage _expérimentale_ appelée **Nessak**, ainsi qu'une structure de fonction de hachage _expérimentale_ appelée **Tundra**.

> [!NOTE]
> Plus d'information [ici](https://zffu.dev/works/nessak/Nessak-French.pdf)

> Cette famille de fonctions et cette structure n'ont pas fait l'objet d'une analyse cryptographique professionnelle, mais seulement de tests courants et d'attaques par force brute. Par conséquent, aucune véritable sécurité ne peut être revendiquée, ces éléments étant pour l'instant strictement expérimentaux.

**Définition du hachage**
Le hachage est une opération mathématique unidirectionnelle, rapide à calculer mais difficile à inverser. C'est pourquoi le stockage des [mots de passe](https://fr.wikipedia.org/wiki/Mot_de_passe) et les [signatures numériques](https://fr.wikipedia.org/wiki/Signature_num%C3%A9rique) tirent parti du hachage. Même une modification minime des données d'entrée produit un résultat (le « hash » ou empreinte) très différent. Cette technique est donc utile pour vérifier si deux copies de données ou de logiciels sont identiques. En général, l'opération s'effectue sur un bloc de données d'entrée ; le résultat obtenu est ensuite combiné (haché) avec le bloc suivant pour générer une nouvelle empreinte reflétant l'ensemble des données traitées jusqu'alors. Ce processus se répète jusqu'à ce que l'empreinte finale reflète la totalité des données, y compris le dernier bloc.

Une fonction de hachage est une fonction qui réalise cette opération de hachage.

## La famille Nessak

La famille de fonctions de hachage _Nessak_ est utilisée comme principal moyen de tester et d'améliorer la structure _Tundra_.

Voici les normes reconnues de la famille _Nessak_ :

- `nessak-k2048-2048`
- `nessak-k1024-1024`
- `nessak-k512-512`
- `nessak-k256-256`
- `nessak-k256-128`
- `nessak-k256-64`
- `nessak-k256-32`
- `nessak-k256-16`

Par ailleurs, voici les normes détaillées de la famille _Nessak_ :

- `nessak-k4096-2048`
- `nessak-k2048-1024`
- `nessak-k1024-512`
- `nessak-k512-256`
- `nessak-k512-128`
- `nessak-k512-64`
- `nessak-k512-32`
- `nessak-k512-16`

### Caractéristiques diverses

| **Nom standard**    | **Multiplicateur de longueur minimale de l'état interne** | **Taille du hash (en bits)** | **Taille de voie (en bits)** |
| ------------------- | --------------------------------------------------------- | ---------------------------- | ---------------------------- |
| `nessak-k2048-2048` | $1$                                                       | $2048$                       | $2048$                       |
| `nessak-k1024-1024` | $1$                                                       | $1024$                       | $1024$                       |
| `nessak-k512-512`   | $1$                                                       | $512$                        | $512$                        |
| `nessak-k256-256`   | $1$                                                       | $256$                        | $256$                        |
| `nessak-k256-128`   | $1$                                                       | $128$                        | $256$                        |
| `nessak-k256-64`    | $1$                                                       | $64$                         | $256$                        |
| `nessak-k256-32`    | $1$                                                       | $32$                         | $256$                        |
| `nessak-k256-16`    | $1$                                                       | $16$                         | $256$                        |
| `nessak-k4096-2048` | $2$                                                       | $2048$                       | $4096$                       |
| `nessak-k2048-1024` | $2$                                                       | $1024$                       | $2048$                       |
| `nessak-k1024-512`  | $2$                                                       | $512$                        | $1024$                       |
| `nessak-k512-256`   | $2$                                                       | $256$                        | $512$                        |
| `nessak-k512-128`   | $2$                                                       | $128$                        | $512$                        |
| `nessak-k512-64`    | $2$                                                       | $64$                         | $512$                        |
| `nessak-k512-32`    | $2$                                                       | $32$                         | $512$                        |
| `nessak-k512-16`    | $2$                                                       | $16$                         | $512$                        |

### Caractéristiques de compression

| **Nom standard**    | **Cycles de compression de descente** | **Cycles de compression** |
| ------------------- | ------------------------------------- | ------------------------- |
| `nessak-k2048-2048` | $8$                                   | $64$                      |
| `nessak-k1024-1024` | $8$                                   | $64$                      |
| `nessak-k512-512`   | $8$                                   | $64$                      |
| `nessak-k256-256`   | $8$                                   | $64$                      |
| `nessak-k256-128`   | $8$                                   | $64$                      |
| `nessak-k256-64`    | $8$                                   | $64$                      |
| `nessak-k256-32`    | $8$                                   | $64$                      |
| `nessak-k256-16`    | $8$                                   | $64$                      |
| `nessak-k4096-2048` | $16$                                  | $64$                      |
| `nessak-k2048-1024` | $16$                                  | $64$                      |
| `nessak-k1024-512`  | $16$                                  | $64$                      |
| `nessak-k512-256`   | $16$                                  | $64$                      |
| `nessak-k512-128`   | $16$                                  | $64$                      |
| `nessak-k512-64`    | $16$                                  | $64$                      |
| `nessak-k512-32`    | $16$                                  | $64$                      |
| `nessak-k512-16`    | $16$                                  | $64$                      |

### Caractéristiques de permutation

| **Nom standard**    | **Tours de permutation interne** | **Tours de permutation externe** |
| ------------------- | -------------------------------- | -------------------------------- |
| `nessak-k2048-2048` | $24$                             | $4$                              |
| `nessak-k1024-1024` | $24$                             | $4$                              |
| `nessak-k512-512`   | $24$                             | $4$                              |
| `nessak-k256-256`   | $24$                             | $4$                              |
| `nessak-k256-128`   | $24$                             | $4$                              |
| `nessak-k256-64`    | $24$                             | $4$                              |
| `nessak-k256-32`    | $24$                             | $4$                              |
| `nessak-k256-16`    | $24$                             | $4$                              |
| `nessak-k4096-2048` | $24$                             | $16$                             |
| `nessak-k2048-1024` | $24$                             | $16$                             |
| `nessak-k1024-512`  | $24$                             | $16$                             |
| `nessak-k512-256`   | $24$                             | $16$                             |
| `nessak-k512-128`   | $24$                             | $16$                             |
| `nessak-k512-64`    | $24$                             | $16$                             |
| `nessak-k512-32`    | $24$                             | $16$                             |
| `nessak-k512-16`    | $24$                             | $16$                             |

Par ailleurs, en raison de la nature de la structure **Tundra**, de nouvelles variantes aux paramètres modifiés (y compris la taille du hash) peuvent être facilement élaborées en modifiant simplement la configuration.

La famille **Nessak** hérite également de quelques avantages liés à cette structure, tels que :

- _Une immunité théorique aux attaques par extension de longueur_
- _La modularité_
- _La possibilité de personnaliser les paramètres sans perte de robustesse (en théorie)_
