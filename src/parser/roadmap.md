# Feuille de route pour l'extension du parser PyRust

## Structures à implémenter :
- [ ] Déclarations de fonction
- [ ] Structures de contrôle (if, else, while, for)
- [ ] Structures de données (struct, enum)
- [ ] Expressions plus complexes (opérations binaires, appels de fonction)
- [ ] Blocs de code
- [ ] Importations et modules

## Pour chaque structure :
1. Définir la syntaxe pour les deux modes (indentation et accolades)
2. Implémenter le parsing dans les deux modes
3. Créer des nœuds AST appropriés
4. Écrire des tests unitaires pour chaque mode

## Gestion des modes syntaxiques :
- [ ] Implémenter un mécanisme pour basculer entre les modes
- [ ] Assurer la cohérence du mode au sein d'un même fichier
- [ ] Gérer les erreurs liées aux incohérences de mode