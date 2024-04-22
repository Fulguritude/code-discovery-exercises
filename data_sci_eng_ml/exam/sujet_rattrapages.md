# Big Data & Analytics Advanced

# Examen de rattrapage (durée <1h)

**Vous devez rendre votre copie avant la fin de l'heure, sans quoi vous aurez zéro.**

**Vous avez accès à toutes les ressources que vous souhaitez, notamment le cours, le lexique, et ChatGPT. Par contre, l'examen a été spécifiquement créé pour que ChatGPT ait du mal, et que le suivre bêtement vous donne une note proche de zéro. Il peut très bien être utile pour que vous évitiez le zéro par contre, mais ça vous demandera d'avoir quand même revu le cours, et d'utiliser votre tête.**



## Exercice I) Questions de réflexion (10 points):

Notez que le nombre de points est une indication de la quantité d'information attendue. Chaque information attendue correspond à +0.25 point. Une information inutile à la question (vraiment hors-sujet, comme environ un quart du blabla habituel de ChatGPT) sera comptée comme -0.25 point (mais ne s'appliquera qu'à la question elle-même, pas de manière globale).

### Questions

1) Comment expliqueriez-vous, en détail, la notion de concurrence en informatique (1,5 points) ? Quel est son usage particulier pour le calcul distribué (0.5 points) ? (2 points)

2) Expliquez ce qu'est une base de données dite "SQL", son importance, son fonctionnement, les considérations importantes à avoir en l'utilisant, notamment de manière distribuée, et ses limites (3,5 points); nommez et décrivez brièvement 4 types de bases de données dite "NoSQL" fréquentes (0,25 points chacune) (4,5 points)

3) Décrivez les rôles respectifs, composants principaux et le fonctionnement de Apache Hadoop (1 point), Spark (1,5 point), et Kafka (1 point) (3.5 points)



## Exercice II) QCM (20 questions, 0,5 points chacune, soit 10 points au total; malus *global* en cas de mauvaise réponse)

### Barême

Notez le barême suivant. Il est fait pour vous encourager à ne sélectionner que ce de quoi vous êtes sûrs, et ne jamais viser au hasard. Si votre score est négatif à cette section, cela impactera la précédente.

  - question à une seule bonne réponse:
    - réponse juste: +0,5 point
    - réponse vide: +0 point
    - réponse fausse: -1 point

  - question à réponses multiples:
    - réponse exacte (toute les réponses valides sont sélectionnées): +0,5 point
    - réponse juste, mais avec une réponse valide manquante: +0,25 point
    - réponse juste ou vide (c'est-à-dire avec plus d'une réponse valide manquante): +0 point
    - réponse avec une info fausse sélectionnée: -1 point

### Questions

1) La force d'un réseau peer-to-peer (pair-à-pair) est sa capacité à:
  x distribuer les données
  x distribuer les calculs
  x distribuer les transferts
  x aider à centraliser des calculs complexes dans un réseau
  x aider à décentraliser des calculs complexes dans un réseau

2) La différence entre TCP et UDP c'est:
  - l'envoyeur d'un message TCP attend une réponse du receveur, mais pas pour un envoyeur UDP
  - l'envoyeur d'un message UDP attend une réponse du receveur, mais pas pour un envoyeur TCP
  - les deux attendent une réponse du receveur, mais la forme et l'envoi cette réponse diffèrent
  - autre chose

3) Lors d'une "ingestion" de données, on va chercher:
  x de la donnée brute avec des capteurs physiques
  x de la donnée numérisée à l'aide de formulaire
  x de la donnée numérisée mais dans des machines difficile d'accès
  x de la donnée déjà présente dans un système informatique

4) Un cache mémoire est un mécanisme:
  x permettant de gagner en taille mémoire, contre le fait de perdre en vitesse de récupération
  x permettant de gagner en vitesse de récupération mémoire, contre le fait de perdre en taille
  x applicable dans un contexte local
  x applicable dans un contexte distribué

5) La raison pour laquelle on peut faire des représentations graphiques partielles de tables relationnelles comme des nuages de points dans une représentation similaire à celle des espaces vectoriels est:
  - parce que la donnée dans un tableau est forcément celle d'un espace vectoriel
  - les axes de l'espace représentent les lignes du tableau
  - parce que l'une comme l'autre sont fondées sur le concept de produits cartésiens d'ensembles
  - en réalité, on ne peut pas toujours représenter la donnée d'un tableau dans un espace vectoriel

6) Un graphe est dit "orienté" si:
  - les liens y sont unidirectionnels
  - les liens vont toujours dans les deux sens
  - les barres peuvent être positives ou négatives
  - on associe une valeur aux noeuds et aux liens
  - on associe une valeur aux barres

7) Le théorème CAP exprime:
  - une façon mathématique de faire une table de données relationnelle résistante au changement
  - un tradeoff entre disponibilité et cohérence des machines et leur contenu dans le cas de partition d'un réseau
  - une technique de gestion de la concurrence dans un programme distribué via l'atomisation des transactions
  - autre chose

8) Une série temporelle représente une donnée de type:
  - int vers int
  - int vers float
  - int vers tout type de données
  - float vers int
  - float vers float
  - float vers tout type de données

9) Un type en informatique c'est:
  x un espace mathématique utilisé pour définir la nature d'un objet dans un programme 
  x un encodage spécifique de la donnée en question
  x une contrainte sur ce que peut être une variable
  x quelque chose que tout objet dans un langage possède
  x quelque chose qui n'est pas transformable comme le serait une variable par une fonction

10) Un index dans une base de données est représenté sous forme de:
  - arbre binaire quelconque simple
  - hashmap simple
  - BTree simple
  - arbre binaire quelconque avec liste doublement chaînée
  - hashmap avec liste doublement chaînée
  - BTree avec liste doublement chaînée

11) La notation "grand O" en algorithmique représente:
  - le nombre moyen d'opérations nécessaires, pour toute quantité d'input
  - le nombre moyen d'opérations nécessaires, mais seulement quand la quantité d'inputs à traiter est grand
  - le nombre total d'opérations nécessaires, pour toute quantité d'input
  - le nombre total d'opérations nécessaires, mais seulement quand la quantité d'inputs à traiter est grand
  - l'ordre de grandeur d'opérations nécessaires, pour toute quantité d'input
  - l'ordre de grandeur d'opérations nécessaires, mais seulement quand la quantité d'inputs à traiter est grand

12) Docker est une technologie permettant de:
  x faire tourner la couche applicative d'un OS virtuel dans un autre OS
  x faire tourner un OS virtuel dans un autre OS
  x déployer des machines virtuelles massivement dans un cloud
  x organiser des machines virtuelles en clusters
  x centraliser la gestion de diverses machines automatiquement

13) Dans le contexte de la cryptographie asymétrique:
  - on encrypte un message avec une clef privé que l'on envoie, accompagné d'une clef publique, pour que le receveur puisse lire le message
  - deux individus partagent la même clef publique et la même clef privée afin de s'envoyer des messages que seul eux deux peuvent lire
  - on envoie une clef publique avec laquelle le receveur de cette clef encrypte son message, qu'il nous envoie encrypté, et que l'on décrypte avec notre clef privée
  - autre chose

14) Laquelle des suites de commandes suivantes correspond à l'ordre d'exécution normal d'une requête de type lecture SQL ?
  - SELECT, FROM, JOIN, ON, WHERE, GROUPBY, HAVING, ORDERBY, LIMIT
  - SELECT, FROM, GROUPBY, HAVING, ORDERBY, LIMIT, JOIN, ON, WHERE
  - SELECT, FROM, JOIN, ON, HAVING, ORDERBY, WHERE, GROUPBY, LIMIT
  - SELECT, FROM, LIMIT, JOIN, ON, WHERE, GROUPBY, ORDERBY, HAVING
  - SELECT, FROM, GROUPBY, ORDERBY, JOIN, ON, WHERE, HAVING, LIMIT
  - SELECT, FROM, GROUPBY, JOIN, ON, HAVING, ORDERBY, LIMIT, WHERE
  - autre chose

15) Quand vaut-il mieux créer un index sur un attribut en SQL ?
  x quand le contenu de la colonne change peu souvent
  x quand le contenu de la colonne change souvent
  x quand la colonne est fréquemment utilisée pour sélectionner les rangées
  x quand on a une requête exacte que l'on fait tout le temps

16) Un tenseur c'est:
  x une série d'arrays imbriqués, contenant des éléments du même type
  x une structure dans un GPU
  x un type de vecteur particulier, si ses éléments sont des float
  x une structure à dimension dynamique

17) Laquelle des technologies suivantes permet de faire des caches ?
  - Redis
  - Hadoop
  - Spark
  - Hive
  - Kafka

18) Un disque dur énorme, plein de vidéos, c'est de la donnée:
  - non-structurée
  - semi-structurée
  - structurée
  - ça dépend

19) Le propre des vecteurs dans un K-espace vectoriel c'est:
  x d'être des listes finies de nombres
  x de pouvoir être additionné
  x de pouvoir être dilatés (scaled)
  x de pouvoir être multipliés
  x d'avoir un angle entre eux
  x de ne pas avoir un angle entre eux

20) Sélectionnez les techniques de monitoring de système informatique
  x logging
  x write-ahead logging
  x tracing
  x tracking
  x backtracing
  x backtracking
  x stack tracing
  x resource tracking
  x caching
