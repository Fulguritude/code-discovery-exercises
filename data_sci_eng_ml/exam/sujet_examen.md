# Big Data & Analytics Advanced

# Examen final (2023-10-23, durée <3h15)

## Consignes générales

**Chaque exercice devrait durer environ 50 minutes à 1 heure**. Chacun des exercices pourrait mériter beaucoup plus de temps et de soin. Ce n'est pas le but ici. Il vaut mieux mettre un timer strict, aller à l'essentiel, et ne pas trop s'étendre.

**Lisez le sujet dans son ensemble d'abord, et prévoyez du temps pour la relecture avant l'envoi des copies.** Après 3h d'examen, vous aurez une dizaine de minutes pour rendre votre travail sur Teams via le mécanisme des "devoirs" (assignments). Ce travail sera formé d'une archive zip contenant 3 dossiers (un par exercice), chacun contenant vos réponses textuelles, les éventuelles photos ou images de diagrammes (avec légende), ou vos fichiers de code. Assurez-vous que vos fichiers de code, en particulier, soient bien organisés, de manière à ce que votre code puisse tourner dès que j'ouvre l'archive et lance la commande `python3 main.py` dans le bon dossier; ou alors qu'une variable `DATASET_FILEPATH` soit mise en évidence pour que je mette le chemin du mien.

Si un rendu ne respecte pas cette consigne, il est possible que je retire des points. Idem si votre texte est vraiment illisible (trop d'erreurs de français). 

**Attention, si vous envoyez une archive corrompue et sortez de la salle, ou dépassez les délais autorisés, votre examen sera compté comme non-valide (zéro!), et vous serez invité(e) aux rattrapages, qui ne sont pas forcément plus sympas... Ce serait dommage. 🙃** Je vous confirmerai progressivement les archives valides que j'ai reçues, afin que vous puissiez aller manger sans stress.

A priori, l'examen est sur 23 points, avec 3 points bonus. Si certains exercices se sont avérés trop durs pour toute la classe, le barême sera peut-être revu compte-tenu des délais courts entre ce module et l'examen.

Lancez la commande de téléchargement des librairies Python (si ce n'est pas déjà fait) et le téléchargement du dataset de fleurs ( https://www.robots.ox.ac.uk/~vgg/data/flowers/102/index.html ) dès le début de l'examen pour ne pas perdre de temps.



## Exercice I) Restitution de connaissances (8 points)

**Cet exercice a pour but d'évaluer votre niveau d'abstraction et de connaissances dans votre compréhension du big data.**

### Enoncé

Décrivez les différents étapes d'une pipeline de données.

Pour chaque étape:
  - nommez cette étape, expliquez ce (ces) nom(s)
  - définissez cette étape
  - parlez des enjeux spécifiques au big data pour cette étape (concepts fondamentaux et considérations essentielles à prendre en compte)
  - décrivez le genre de tâche ou rôle qu'il faut assurer pour gérer cette étape
  - donnez un ou deux exemples concrets

*NB: nos amis les diagrammes (lisibles) sont les bienvenus.*

### Barême détaillé

- division et définition des étapes, cohérentes et complètes (2 points)
- avoir parlé des enjeux spécifiques à chaque étape (1.5 points)
- description du genre de tâche ou rôle qu'il faut assurer pour gérer chaque étape (0.75 points)
- un ou deux exemples concrets de cette étape pour un domaine choisi (0.75 points)
- une description plus détaillée de certaines étapes un peu plus complexes (1 point)
- des explications particulièrement claires (1 point)
- des mises en valeur des problèmes spécifiques au big data sur la notion de pipeline elle-même (0.5 points)
- une explication des limites de votre modèle choisi de pipeline (0.5 points)



## Exercice II) Manipulation et analyse de données (8 points)

**Cet exercice consiste à étudier un dataset relativement usuel, relativement large, de bout-en-bout.**

### Enoncé

On vous fournit un dossier contenant un dataset avec des fichiers représentant des images. Ce dossier correspond à un dataset non-structuré "large" qu'il vous convient de traiter. Avec le langage Python 3, en utilisant les librairies usuelles de la data et du traitement d'image, vous devez créer un dataset contenant des statistiques sur les images, et afficher ces statistiques. Ce tableau aura les attributs suivants (et dans cet ordre):

  - Filename: le nom du fichier initial sans extension.
  - Height: la hauteur de l'image.
  - Width: la largeur de l'image.
  - Average_Red: la valeur moyenne de rouge pour un pixel de cette image.
  - Average_Green: la valeur moyenne de vert pour un pixel de cette image.
  - Average_Blue: la valeur moyenne de bleu pour un pixel de cette image.

Vous devrez aussi fournir de quoi visualiser chaque image comme un point dans un repère 3D. Ce repère aura pour axes (nommés "Red", "Green" et "Blue") les 3 valeurs de moyenne de couleur. Vous devrez faire une analyse sommaire de cette donnée (soit en commentaire dans le code, soit dans un fichier `.txt` à part) et de quoi enregistrer ce tableau sous forme `.tsv`. Votre script, quand exécuté, devra produire automatiquement le fichier `imageinfo.tsv`, et afficher le plot. Votre code peut être divisé dans autant de fichier que vous souhaitez; cependant, votre logiciel devra être exécutable via la commande `python3 main.py`.

NB: Vous devrez coder en anglais, pas en français.

NB: Nous conseillons les librairies pandas, PIL, pytorch (ou numpy), matplotlib (avec seaborn ou holoviews, éventuellement), et la librairie standard du Python.

NB: Des points seront attribués à la division appropriée du code en fonction simples, typées et composables; ainsi que sur la qualité de la lisibilité du code.

NB: si vous produisez des choses qui me surprennent dans leur qualité (vraiment beau plot, analyse intéressante, statistiques supplémentaires), des points bonus sont envisageables. Ne comptez pas là-dessus pour autant: concentrez-vous sur le barême.

### Barême détaillé

  - lire la donnée (0,5 point)
  - transformer la donnée (2 points)
  - faire des statistiques sur la donnée (1,5 point)
  - produire un dataset standard dans son propre fichier (1 point)
  - produire une visualisation lisible et une explication sommaire de la donnée (1 point)
  - lisibilité et qualité du code (2 points)



## Exercice III) Conception de système (7 points)

**Cet exercice est un exercice de design. Il teste votre adaptivité et votre maîtrise pour un problème de big data.**

### Enoncé

Un client vous demande de lui concevoir un système informatique. Ce client est une installation militaire massive et hautement sécurisée. Cette installation s'occupe uniquement du stockage et du transport d'armes et de personnel. Le personnel reste en général entre plusieurs jours et plusieurs mois.

Le client souhaite développer et maintenir tout le système et le logiciel en interne pour des questions de sécurité. Vous êtes son seul contractant. Toute erreur d'incohérence dans la base de données est à proscrire absolument, car c'est un risque national. Toute communication physique avec l'extérieur a lieu par le biais de camions à travers quelques portails (certains réservés pour les entrées, d'autres pour les sorties) surveillés. Un système de surveillance par caméra et alarmes est actif à travers la base. Le système informatique n'est autrement pas relié à l'extérieur: les personnes accréditées contactent l'Etat-major par le seul biais de téléphones mobiles sur un réseau spécialisé (mobiles qui sont la seule chose dont vous ne vous occuperez pas). Idéalement, la performance du système devrait permettre une détection et une réponse immédiates en cas de problème.

On estime le nombre d'armes stockées à chaque instant à plusieurs millions, le nombre de soldats présents à une dizaines de milliers, et le mouvement journalier se compte en dizaine de milliers d'armes et milliers de soldats. Chaque camion déplace en moyenne une cinquantaine de soldats ou quelques centaines d'armes.

Réfléchissez aux différentes données que vous devrez modéliser, aux types d'individus qui devront interagir avec le système, aux enjeux de sécurité, à l'opération efficace du système informatique et son lien avec le système physique, etc. La description du domaine est laissée assez vague (et volontairement incomplète!) pour laisser un peu de champ libre à votre créativité. (Souvent, le besoin du client n'est pas forcément immédiatement compris par l'ingénieur, mais il faut quand même intéresser le client, et montrer que vous avez une idée de ses enjeux spécifiques.)

Proposez une architecture pour ce système, et expliquez comment celui-ci répond aux divers *types* d'enjeux qu'il faut résoudre, tout en présentant vos questionnements lors de la conception, et en justifiant votre réflexion.


### Barême détaillé

  - capacité à se poser les bonnes questions pour l'ingénierie des systèmes et l'architecture des bases de données (1 point)
  - modélisation d'un domaine sous forme de bases de données dans un système (1,5 point)
  - présentation du système dans son ensemble (entrées et sorties, hardware et software) (1,5 point)
  - comprendre les limites (tradeoffs) des réponses apportées (1 point)
  - qualité de la communication de la conception au client (diagrammes, phrases et idées claires et simples) (1 point)
  - créativité et qualité de la solution (1 point)
