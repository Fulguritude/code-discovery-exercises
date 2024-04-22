
## Exercice I) Questions de réflexion (10 points):

### Correction

1) Comment expliqueriez-vous, en détail, la notion de concurrence en informatique (1,5 points) ? Quel est son usage particulier pour le calcul distribué (0.5 points) ? (2 points)

- la concurrence revient à avoir plusieurs tâches qui sont exécutés simultanément et s'oppose à la notion de séquentialité (0.25 point)
- la concurrence sert à deux choses: améliorer la performance (0.25 point)
- la concurrence comprend 2 notions sous-jacentes fondamentales et distinctes: asynchronie et multithreading (0.25 point)
- l'asynchronie correspond à avoir un processus en train d'exécuter plusieurs tâches (un cuisiner en train de préparer la sauce, la viande, et les légumes dans 3 pots, à tour de rôle, en même temps) (0.25 point)
- le multithreading correspond à avoir plusieurs processus en train d'exécuter une ou plusieurs tâches en même temps (comme engager plusieurs cuisiniers) (0.25 point)
- la concurrence mène à 3 types de bug fondamentaux: race conditions et deadlocks, qui se règlent en général à coup de sémaphore / lock / mutex. (0.25 point)

- dans un système distribué, la concurrence est inévitable, car plusieurs ordis en réseau impliquent forcément plusieurs processus concurrents (0.25 point)
- dans un système distribué, on va en général souhaiter faire des opérations parallèles (c'est-à-dire le même calcul mais sur des données volumineuses d'un même type). (0.25 point)

Si on parle de big data, analyse en temps réel, fault tolerance, load balancing; tout sujet "big data" qui n'est pas directement la concurrence, et celle qui a lieu pour le calcul distribué, on perd des points.



2) Expliquez ce qu'est une base de données dite "SQL", son importance, son fonctionnement, les considérations importantes à avoir en l'utilisant, notamment de manière distribuée, et ses limites (3,5 points). Nommez et décrivez brièvement 4 types de bases de données dite "NoSQL" fréquentes (0.25 points chacune) (4,5 points)

(Total général 1.5 point)
- Une base de données SQL, aussi dite "relationnelle" est une architecture permettant de stocker et gèrer des données structurées sous forme de tables reliées les unes aux autres. (0.25 point)
- Dans une table d'une base de données SQL est composée de lignes, on appelle les lignes les "entités" ou "individus" et les colonnes sont appelées "attributs" ou "variables". (0.25 point)
- Mathématiquement, une base de donnée SQL est un sous-ensemble du produit cartésien des attributs. (0.25 point)
- L'appelation SQL fait référence au SQL (Structured Query Language), qui est une façon programmatique d'écrire ou de lire le contenu de ce type de base de données, dont le vrai nom est "relationnelle". (0.25 point)
- les différentes opérations sur une BDD relationnel sont souvent décrites selon l'acronyme CRUD (CREATE, READ, UPDATE, DELETE). (0.25 point)
- pour optimiser les performances de lecture (contre un coup d'écriture), on utilise un structure de données appelée un index sur un attribut (0.25 point)

(Total ACID 0.5 point)
- Il y a un acronyme des propriétés qui garantissent un traitement fiable des transactions de base de données: ACID (Atomicité, Cohérence, Isolation, Durabilité) (0.25 point)
- Atomicité: les transactions sont traitées comme des unités atomiques, ce qui signifie qu'elles sont soit entièrement terminées, soit n'ont aucun effet.
- Cohérence: les transactions font passer la base de données d'un état cohérent à un autre. Il s'agit d'appliquer des contraintes d’intégrité et d’intégrité des données.
- Isolation: les transactions sont exécutées indépendamment les unes des autres, en séquence, garantissant que l'exécution simultanée des transactions ne produit pas de résultats inattendus.
- Durabilité: Une fois qu'une transaction est validée, ses effets sont permanents, même en cas de panne du système. Il garantit la persistance des données.

(Total normalisation et sharding 1.5)
- Les bases de données relationnelles peuvent avoir du mal à scale, du coup on emploie en général une subdivision de la base de données, appelé sharding. (0.25 point)
- Il y a le sharding horizontal, qui divise les données selon les lignes (typiquement selon un index), et le sharding vertical, qui divise les données selon les colonnes (typiquement en groupant les attributs qui servent à des analyses communes). (0.25 point)
- Il y a une façon d'organiser une base de données relationnelle de manière à éviter beaucoup d'erreurs (anomalies d'update, insertion, update) (0.25 point)
- On peut vouloir dénormaliser une BDD relationnelle, typiquement si on doit lire de l'information complexe plus rapidement (0.25 point)

Au choix 4 des suivantes, si plus, on perd des points:
- Base de données de documents : Stocke des données dans des documents JSON ou XML. Exemple : MongoDB.
- Base de données clé-valeur : Stocke des données sous forme de paires clé-valeur. Exemples : Amazon DynamoDB.
- Base de données orientée colonnes : Stocke des données dans des colonnes plutôt que des lignes. Exemples : Apache Cassandra, HBase.
- Base de données orientée graphe : Modélise les données en utilisant des nœuds et des arêtes pour représenter les relations. Exemples : Neo4j, Amazon Neptune.
- Base de données de temps réel : Ces bases de données sont conçues pour gérer des données en temps réel et sont couramment utilisées dans les applications de suivi, de surveillance et de streaming. Exemple : InfluxDB.
- Base de données de recherche : Ces bases de données sont optimisées pour la recherche en texte intégral. Exemple : Elasticsearch.
- Base de données objet : Ces bases de données sont conçues pour stocker des objets directement, souvent utilisées dans les applications orientées objet. Exemple : db4o.
- Base de données de séries temporelles : Optimisées pour le stockage et la récupération de données de séries temporelles, couramment utilisées dans la surveillance et les applications IoT. Exemple : OpenTSDB.



3) Décrivez les rôles respectifs, composants principaux et le fonctionnement de Apache Hadoop (1 point), Spark (1,5 point), et Kafka (1 point) (3.5 points)

- Apache Hadoop est une architecture pour distribuer de la donnée dans plusieurs ordis en réseau de manière à ce qu'elle soit accessible même si un ou plusieurs des ordis ne marche pas (0.25 point)
- HDFS (Hadoop File System): la partie qui s'occupe du stockage et de l'accès aux fichiers par blocs répliqués et distribués (0.25 point)
- YARN (Yet Another Resource Negociator): la partie qui s'occupe de la gestion des ressources (mémoire et calcul) des tâches et des processus dans les diverses machines du système (0.25 point)
- MapReduce: la partie qui s'occupe du calcul à proprement parler, en organisant des morceaux de calcul par machine (Map) puis en agrégeant les résultats (Reduce) (0.25 point)

- Apache Spark est un framework de calcul distribué pour traiter la donnée de divers types de base de données, compatible avec Hadoop, mais pas uniquement (0.25 point)
- La structure de données fondamentale de Spark est le Resilient Distributed Dataset, qui est une collection d'objets, potentiellement distribuée (0.25 point)
- Spark Core: fournit de l'I/O basique, gère la répartition des tâches, gère la planification des tâches,
gère la mémoire et récupération des pannes. (0.25 point)
- Spark SQL: permet d'exécuter des requêtes de type SQL sur des données structurées (et potentiellement distribuées) (0.25 point)
- Spark Streaming: librairie qui fonctionne sur des flux de données (c'est-à-dire pour un traitement "presque" en temps réel, par exemple en recevant des éléments de Kafka) (0.25 points)
- MLlib: une librairie de machine learning, offrant des algos de classification, de régression, de clustering, etc. (0.25 point)
- GraphX: librairie pour de l'analyse de graphes de grande échelle. (0.25 points)

- Apache Kafka est une architecture pour gérer de la transimission et stockage temporaire de données par événement, via un modèle publisher/subscriber (0.25 point)
- Kafka organise les échanges de données selon 3 types d'ordinateurs les Producers (qui génèrent la donnée) les Brokers (qui reçoivent la donnée, la stockent et la transfèrent) et les Consumers (qui lisent la donnée des Brokers). (0.25 point)
- La donnée dans Kafka est organisée en Topics. Il s'agit de fils de messages binaires, divisés en partitions dans plusieurs machines. (0.25 point)
- Lire la donnée d'un topic Kafka se fait depuis une adresse composée de Broker-Partition-Offset, par un système de fenêtrage. (0.25 point)



## Exercice II) QCM

### Correction

II) 1) La force d'un réseau peer-to-peer (pair-à-pair) est sa capacité à:
  V distribuer les données
  V distribuer les calculs
  V distribuer les transferts
  F aider à centraliser des calculs complexes dans un réseau
  V aider à décentraliser des calculs complexes dans un réseau

II) 2) La différence entre TCP et UDP c'est:
  V l'envoyeur d'un message TCP attend une réponse du receveur, mais pas pour un envoyeur UDP
  - l'envoyeur d'un message UDP attend une réponse du receveur, mais pas pour un envoyeur TCP
  - les deux attendent une réponse du receveur, mais la forme et l'envoi cette réponse diffèrent
  - autre chose

II) 3) Lors d'une "ingestion" de données, on peut vouloir aller chercher:
  V de la donnée brute avec des capteurs physiques
  V de la donnée numérisée à l'aide de formulaire
  V de la donnée numérisée mais dans des machines difficile d'accès
  V de la donnée papier
  V de la donnée déjà présente dans un système informatique

II) 4) Un cache mémoire est un mécanisme:
  F permettant de gagner en taille mémoire, contre le fait de perdre en vitesse de récupération
  V permettant de gagner en vitesse de récupération mémoire, contre le fait de perdre en taille
  V applicable dans un contexte local
  V applicable dans un contexte distribué

II) 5) La raison pour laquelle on peut faire des représentations graphiques partielles de tables relationnelles comme des nuages de points dans une représentation similaire à celle des espaces vectoriels est:
  - parce que la donnée dans un tableau est forcément celle d'un espace vectoriel
  - les axes de l'espace représentent les lignes du tableau
  V parce que l'une comme l'autre sont fondées sur le concept de produits cartésiens d'ensembles
  - en réalité, on ne peut pas toujours représenter la donnée d'un tableau dans un espace vectoriel

II) 6) Un graphe est dit "orienté" si:
  V les liens y sont unidirectionnels
  - les liens vont toujours dans les deux sens
  - les barres peuvent être positives ou négatives
  - on associe une valeur aux noeuds et aux liens
  - on associe une valeur aux barres

II) 7) Le théorème CAP exprime:
  - une façon mathématique de faire une table de données relationnelle résistante au changement
  V un tradeoff entre disponibilité et cohérence des machines et leur contenu dans le cas de partition d'un réseau
  - une technique de gestion de la concurrence dans un programme distribué via l'atomisation des transactions
  - autre chose

II) 8) Une série temporelle représente une donnée de type:
  - int vers int
  - int vers float
  V int vers tout type de données
  - float vers int
  - float vers float
  - float vers tout type de données

II) 9) Un type en informatique c'est:
  V un espace mathématique utilisé pour définir la nature d'un objet dans un programme 
  V un encodage spécifique de la donnée en question
  V une contrainte sur ce que peut être une variable
  F quelque chose qui n'est pas transformable comme le serait une variable par une fonction
  V quelque chose que tout objet dans un langage possède

II) 10) Un index dans une base de données est en général encodé sous forme de:
  - arbre binaire quelconque simple
  - hashmap simple
  - BTree simple
  - arbre binaire quelconque avec liste doublement chaînée
  - hashmap avec liste doublement chaînée
  V BTree avec liste doublement chaînée

II) 11) La notation "grand O" en algorithmique représente:
  - le nombre moyen d'opérations nécessaires, pour toute quantité d'input
  - le nombre moyen d'opérations nécessaires, mais seulement quand la quantité d'inputs à traiter est grand
  - le nombre total d'opérations nécessaires, pour toute quantité d'input
  - le nombre total d'opérations nécessaires, mais seulement quand la quantité d'inputs à traiter est grand
  - l'ordre de grandeur d'opérations nécessaires, pour toute quantité d'input
  V l'ordre de grandeur d'opérations nécessaires, mais seulement quand la quantité d'inputs à traiter est grand

II) 12) Docker est une technologie permettant de:
  V faire tourner la couche applicative d'un OS virtuel dans un autre OS
  F faire tourner un OS virtuel dans un autre OS
  F déployer des machines virtuelles massivement dans un cloud
  F organiser des machines virtuelles en clusters
  F centraliser la gestion de diverses machines automatiquement

II) 13) Dans le contexte de la cryptographie asymétrique:
  - on encrypte un message avec une clef privé que l'on envoie, accompagné d'une clef publique, pour que le receveur puisse lire le message
  - deux individus partagent la même clef publique et la même clef privée afin de s'envoyer des messages que seul eux deux peuvent lire
  V on envoie une clef publique avec laquelle le receveur de cette clef encrypte son message, qu'il nous envoie encrypté, et que l'on décrypte avec notre clef privée
  - autre chose

II) 14) Laquelle des suites de commandes suivantes correspond à l'ordre d'exécution normal d'une requête de type lecture SQL ?
  V SELECT, FROM, JOIN, ON, WHERE, GROUPBY, HAVING, ORDERBY, LIMIT
  - SELECT, FROM, GROUPBY, HAVING, ORDERBY, LIMIT, JOIN, ON, WHERE
  - SELECT, FROM, JOIN, ON, HAVING, ORDERBY, WHERE, GROUPBY, LIMIT
  - SELECT, FROM, LIMIT, JOIN, ON, WHERE, GROUPBY, ORDERBY, HAVING
  - SELECT, FROM, GROUPBY, ORDERBY, JOIN, ON, WHERE, HAVING, LIMIT
  - SELECT, FROM, GROUPBY, JOIN, ON, HAVING, ORDERBY, LIMIT, WHERE
  - autre chose

II) 15) Quand vaut-il mieux créer un index sur un attribut en SQL ?
  V quand le contenu de la colonne change peu souvent
  F quand le contenu de la colonne change souvent
  V quand la colonne est fréquemment utilisée pour sélectionner les rangées
  V quand on a une requête exacte que l'on fait tout le temps

II) 16) Un tenseur c'est:
  V une série d'arrays imbriqués, contenant des éléments du même type
  F une structure dans un GPU
  V un type de vecteur particulier, si ses éléments sont des float
  V une structure à dimension dynamique

II) 17) Laquelle des technologies suivantes permet de faire des caches ?
  V Redis
  - Hadoop
  - Spark
  - Hive
  - Kafka

II) 18) Un disque dur énorme, plein de vidéos, c'est de la donnée:
  - non-structurée
  - semi-structurée
  - structurée
  V ça dépend

II) 19) Le propre des vecteurs dans un K-espace vectoriel c'est:
  F d'être des listes finies de nombres
  V de pouvoir être additionnés
  V de pouvoir être dilatés (scaled)
  F de pouvoir être multipliés
  F d'avoir un angle entre eux
  V de ne pas avoir un angle entre eux

II) 20) Sélectionnez les techniques de monitoring de système informatique
  V logging
  V write-ahead logging
  V tracing
  F tracking
  F backtracing
  F backtracking
  F stack tracing
  V resource tracking
  F caching
