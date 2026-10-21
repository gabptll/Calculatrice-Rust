Calculatrice fonctionnant dans le terminal en RUST

Pour calculer : écrivez votre équation sous le format : "a OP b OP c ... n"

L'entrée est ensuite analysée afin de séparer les thermes et les opérateurs

Opérateur : 
  -"^" (a ^ b) renvoie a puissance b
  -"*" (a * b) renvoie a fois b
  -"/" (a / b) renvoie a divisé par b
  -"%" (a % b) renvoie le reste de la division euclidienne de a par b
  -"+" (a + b) renvoie a plus b
  -"-" (a - b) renvoie a moins b

La priorité sera donnée a (encore non codée):
- ^
- *, /, %
- +, -

Fonctions :
- help / h : affiche le panneau d'aide
- quit / q : quitte le programme
- clear / clr : efface tout le terminal
- sqrt / racine : calcul la racine carrée d'un nombre

