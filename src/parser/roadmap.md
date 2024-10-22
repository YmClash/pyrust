# Feuille de route pour l'implémentation du parser PyRust

## 1. Expressions
- [x] Expressions primaires (littéraux, identifiants)
- [x] Expressions unaires
- [x] Expressions binaires avec précédence
- [ ] Appels de fonction
- [ ] Accès aux membres (dot notation)
- [ ] Expressions lambda
- [ ] Expressions de cast de type

## 2. Déclarations
- [x] Déclarations de variables
- [x] Déclarations de constantes
- [ ] Déclarations de fonctions
- [ ] Déclarations de structures
- [ ] Déclarations d'énumérations
- [ ] Déclarations de traits
- [ ] Déclarations de classes

## 3. Statements
- [ ] Blocs de code
- [ ] Statements d'expression
- [ ] Statements de retour
- [ ] Statements if-else
- [ ] Boucles while
- [ ] Boucles for

## 4. Types
- [x] Types primitifs (int, float, bool, str, char)
- [ ] Types composés (arrays, tuples)
- [ ] Types génériques
- [ ] Types de fonction

## 5. Gestion des modes syntaxiques
- [ ] Mode accolades
- [ ] Mode indentation
- [ ] Basculement entre les modes

## 6. Gestion des erreurs
- [x] Erreurs de base
- [ ] Messages d'erreur plus détaillés
- [ ] Récupération d'erreurs pour continuer le parsing

## 7. Tests
- [ ] Tests unitaires pour chaque composant
- [ ] Tests d'intégration pour des programmes complets
- [ ] Tests de performance

## 8. Optimisations
- [ ] Optimisation du parsing des expressions
- [ ] Mise en cache des résultats intermédiaires

## 9. Fonctionnalités avancées
- [ ] Support des annotations
- [ ] Macros
- [ ] Gestion des modules et imports

## 10. Documentation
- [ ] Documentation du code
- [ ] Guide d'utilisation du parser
- [ ] Exemples de programmes PyRust

## Étapes de mise en œuvre :

1. Compléter le parsing des expressions
    - Implémenter les appels de fonction
    - Ajouter le support des expressions lambda
    - Gérer les casts de type

2. Finaliser les déclarations
    - Compléter parse_function_declaration
    - Implémenter parse_struct_declaration
    - Ajouter le support pour les énumérations, traits et classes

3. Implémenter les statements de contrôle
    - Ajouter le support pour if-else, while, et for
    - Gérer les blocs de code dans les deux modes syntaxiques

4. Étendre le système de types
    - Ajouter le support pour les types composés
    - Implémenter la gestion des types génériques

5. Améliorer la gestion des erreurs
    - Ajouter plus de contexte aux messages d'erreur
    - Implémenter la récupération d'erreurs pour un parsing plus robuste

6. Étoffer la suite de tests
    - Créer des tests unitaires pour chaque composant du parser
    - Ajouter des tests d'intégration pour des programmes PyRust complets

7. Optimiser les performances
    - Profiler le parser pour identifier les goulots d'étranglement
    - Optimiser les parties critiques du code

8. Ajouter des fonctionnalités avancées
    - Implémenter le support des annotations
    - Ajouter la gestion des macros
    - Gérer les imports et les modules

9. Documenter le projet
    - Ajouter des commentaires de documentation à chaque fonction
    - Créer un guide d'utilisation du parser
    - Fournir des exemples de programmes PyRust parsés

10. Révision et refactoring
    - Revoir le code pour assurer la cohérence
    - Refactoriser les parties complexes pour améliorer la lisibilité
    - Préparer le parser pour l'intégration avec les autres composants du compilateur