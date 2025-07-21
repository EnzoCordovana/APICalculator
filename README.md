# APICalculator

API Calculator en Rust + Docker

## Routes

### Opération de base

- `POST` | `/api/v1/add` - Addition de deux nombres
- `POST` | `/api/v1/substract` - Soustraction
- `POST` | `/api/v1/multiply` - Multiplication
- `POST` | `/api/v1/divide` - Division

### Opérations avancées

- `POST` | `/api/v1/power` - Puissance ($x^y$)
- `POST` | `/api/v1/sqrt` - Racine carrée
- `POST` | `/api/v1/log` - Logarithme
- `POST` | `/api/v1/sin` / `/api/v1/cos` / `/api/v1/tan` - Fonctions trigonométriques

### Routes utilitaires

- `GET` | `/api/v1/status` - Vérification du statut de l'API
- `GET` | `/api/v1/opérations` - Liste des opérations disponibles