# 🚀 Roadmap "Objectif Cuisine" (Ce soir)

Pour que tu puisses débrancher ton PC et jouer depuis ton Mac avant que le petit ne se réveille de sa prochaine sieste, voici l'ordre de marche :

## Étape 1 : Le Blindage (15 min)

- [ ] Finaliser Params::sanitize() dans le Core.

- [ ] Intégrer l'appel au sanitizer dans GameProfile::launch().

- **Objectif : Aucune valeur bizarre ne doit passer, même si on bidouille le fichier de config à distance.**

## Étape 2 : Le "Sunshine Aware" (10 min)

- [ ] Ajouter la lecture des variables SUNSHINE_CLIENT_WIDTH/HEIGHT dans le main.rs.

- **Objectif : Que Gamescope s'adapte à ton Mac automatiquement.**

## Étape 3 : La Clé de Contact Headless (5 min)

- [ ] Lancer la commande : sudo kernelstub -a "video=DP-1:e" (ou le port utilisé par ta 3080 Ti).

- [ ] Redémarrer la tour.

- **Objectif : Que ton PC démarre une session COSMIC même sans écran.**

## Étape 4 : Le Test de Feu (Le moment de vérité)

- [ ] Débrancher l'écran Samsung.

- [ ] Tenter une connexion Moonlight depuis le Mac.

- [ ] Lancer un jeu via Steam.
