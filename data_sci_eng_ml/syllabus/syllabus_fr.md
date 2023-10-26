# Descriptif du cours

## Contenu des cours

### Cours 1 et 2
- ~4h de présentation de ce qu'est une pipeline de données moderne (de la mesure du monde physique via capteurs ou formulaires, jusqu'à l'analyse et visualization des datasets, en passant par un bon brin d'ingénierie des systèmes au milieu), 
- ~1h de maths (un peu de déblayage de théorie des ensembles (diagrammes de Venn et produits cartésiens) pour aborder l'intuition des probas, des joins, et de pourquoi on peut foutre des tableaux relationnels dans des espaces vectoriels). 
- ~1h de conseils d'ingénierie logicielle / développement logiciel (comment présenter son code niveau typage et style, un sujet sous-estimé, mais très important car big data veut en général dire big codebase, car "Scala" est quand même dans l'intitulé du cours, et car le python ajd ça se type-hint quand même sérieusement)

### Cours 3 et 4
- ~1h30 de l'algèbre linéaire "étendu" (sous-entendant "avec des axes qui ne sont pas les nombres réels", free vector space over a set, etc), afin de vraiment installer une compréhension plus profonde de comment visualiser un tableau
- ~1h30: parallélisation, distribution et concurrence (insister un brin sur ce que ça veut dire un "système distribué", d'un point de vue plus logiciel que les enjeux plus "hardware" traités en première journée)
- ~3h: modèles SQL et NoSQL (vraiment aller en profondeur sur les modèles de données qu'on peut vouloir choisir lorsqu'on doit traiter de la données en masse, et comment ce choix affecte l'archi, les algos, etc.)

### Cours 5 et 6
- ~3h: data transformation / tensor manipulation / data visualization with pandas/numpy/pytorch/matplotlib
- ~3h: principes d'un entretien d'architecte informatique: l'exercice de conception de système

### Cours 7 et 8
- ~1h30: devops: shell-scripting, deployment, OS, daemons, réseau (docker et kubernetes ?)
- ~3h: présentation des techs Apache pour le calcul distribué (Zeppelin; Hadoop, Spark, Kafka)
- ~45m: plus de démo Python (matplotlib 3D)
- ~45m: conseils généraux d'apprentissage et de carrière

### Cours 9
- ~3h: examen final
- ~3h: sujets d'état de l'art divers pour le big data (à déterminer):
  - notions de programmation sur GPU (memory transfer, IO costs; shared memory vs distributed parallelism, GPU threads and branching problems)
  - approfondissement des maths pour le SQL et NoSQL (combinatoire, algos de graphe, espace euclidiens, analyse multidimensionnelle, variables et vecteurs aléatoires)
  - curse of dimensionality, algos de data science importants pour y remédier comme SVD / PCA
  - géometrie différentielle
  - analyse topologique de la donnée
  - analyse spectrale




# Ressources pour le cours

## A) Fonctions mathématiques, algèbre linéaire

Les mathématiques des vecteurs et des fonctions sont absolument essentielles pour tous les sujets data, notamment tout le big data.

### Objectifs
- comprendre ce qu’est une fonction, ce qu’est une variable (au sens mathématique du terme)
- savoir dire quel est le domaine (espace d’input) et codomaine (espace d’output) d’une fonction
- comprendre ce qu’est une fonction à valeurs réelles,
- comprendre ce qu’est une addition, une multiplication et une composition de fonctions à valeurs réelles.
- comprendre ce qu’est un taux d’accroissement et une dérivée de fonction,
- comprendre ce qu’est un extremum local ou global d’une fonction
- savoir dessiner (grapher) une courbe de fonction réelle à partir de sa formule
- comprendre ce qu'est un vecteur, une matrice et comment les manipuler
- savoir ce qu’est une fonction linéaire, une fonction affine; leur formules et comment les visualiser intuitivement à partir de cette formule

### Ressources
- https://www.youtube.com/watch?v=fNk_zzaMoSs&list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab (toute la playlist !)
- https://www.youtube.com/watch?v=WUvTyaaNkzM&list=PLZHQObOWTQDMsr9K-rj53DwVRMYO3t5Yr (toute la playlist !)
- https://fr.khanacademy.org/math/linear-algebra
- https://fr.khanacademy.org/math/algebra/algebra-functions
- https://fr.khanacademy.org/math/algebra/linear-word-problems
- https://fr.khanacademy.org/math/algebra-basics/alg-basics-graphing-lines-and-slope
- https://www.brilliant.org/ est aussi une très, très bonne ressource, mais c'est payant
- DataCamp, parfois des trucs gratuits, d'autres payants, très bonne ressource dans l'ensemble
- Geogebra ou Desmos
- WolframAlpha
- si vous avez du mal à vous intéresser aux maths, une petite histoire de la géométrie animée (en 50 minutes au total à peu près), plus du côté histoire que maths, absolument excellente, qui plaît à tout le monde et vous offrira des repères utiles: https://www.youtube.com/watch?v=nkvVR-sKJT8



## B) Fondamentaux de la programmation en Python et du Python pour la data

### Objectifs
- savoir définir une fonction (typer sa signature), distinguer ses inputs et outputs, implémenter son contenu
- savoir écrire en Python une fonction chargée de réaliser quelques algorithmes simples (addition de vecteurs de même taille par exemple, etc.)
- comprendre comment appeler une fonction avec différents arguments
- savoir utiliser les fonctions de librairies externes en Python pour la data
- connaître les librairies usuels de la data et de l'IO en Python

### Ressources
- https://github.com/Fulguritude/code-discovery-exercises/tree/main/Python
- https://github.com/Fulguritude/code-discovery-exercises/tree/main/data_sci_eng_ml/jupyter_notebooks/fr
- Codecademy, Exercism
- StackOverflow, GeekForGeeks
- DataCamp, Towards Data Science
- https://www.youtube.com/@NeetCode
- https://neetcode.io/roadmap



## C) Big Data, architecture des données et ingénierie des systèmes

### Objectifs
- comprendre ce qu'est une pipeline de données moderne et ses enjeux
- comprendre divers types de données standards, et comment on peut les employer
- comprendre ce qu'est le calcul distribué, d'un point de vue du software et du hardware
- savoir analyser un système réel pour le modéliser mathématiquement et informatiquement
- savoir proposer une architecture de système d'information, dans le cadre du web et du traitement de données distribué

### Ressources
- la "bible": "Designing Data-Intensive Applications: The Big Ideas Behind Reliable, Scalable, and Maintainable Systems" par Martin Kleppmann, ebook dans les 30€ environ, version papier dans les 50€
- https://bytebytego.com/ ressource exceptionnelle, même si certaines choses sont payantes (leur bouquin sur la conception de systèmes est gratuit si vous vous inscrivez à leur newsletter)
- https://www.youtube.com/@ByteByteGo/videos
- https://www.youtube.com/@ByteMonk/videos
- https://www.youtube.com/@NeetCode/videos
- https://www.youtube.com/@SimplilearnOfficial/videos
- https://github.com/Fulguritude/code-discovery-exercises/blob/main/data_sci_eng_ml/Data_Lexicon.pdf
- https://www.youtube.com/watch?v=JK2MdJAWEGc (cours sur Hadoop assez approfondi en 4h)
- https://www.michael-noll.com/tutorials/ (excellente ressource pour toutes les choses importantes de l'écosystème Hadoop / Apache)


# Examen

## Examen final

Il sera sur table, en 3h, avec accès à Internet. Vous serez surveillés pour vérifier que vous ne pas passez pas par ChatGPT (quand même). Un barême plus détaillé sera donné avec le sujet.

Barême global (sur 23, donc avec 3 points bonus):
- 8 points sur une question de restitution de connaissances, complexe, mais assez générale
- 8 points sur un exercise de code (en Python) de traitement de données end-to-end
- 7 points sur une question de conception de système, et défense d'architecture

## Examen de rattrapage

Il sera sur table, si possible sans accès à Internet.

- 10 points sur plusieurs questions de restitution de connaissances variées, assez techniques, mais courtes
- 10 points sur un QCM légèrement piégeux avec malus en cas de mauvaise réponse
