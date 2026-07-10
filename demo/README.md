# Demo GIF

Generate a CLI demo GIF with [VHS](https://github.com/charmbracelet/vhs).

## Prérequis

- `vhs` installé
- `ffmpeg` installé
- `ttyd` installé
- `todo` dans le PATH

## Générer le GIF

```bash
vhs demo.tape
```

Le GIF sera créé dans `demo/demo.gif`.

## Modifier la démo

Édite `demo.tape` puis relance `vhs demo.tape`.

### Attention : IDs des tâches

Les IDs dans le JS de la landing page (`c3d4`, `e5f6`) sont simulés.
Le vrai CLI génère des IDs aléatoires (ex: `oiEc`). Si tu veux faire
`todo status <id> --set ...`, lance d'abord `todo add` pour voir le vrai ID,
puis utilise-le dans le tape.

## Commandes VHS utiles

| Commande | Description |
|----------|-------------|
| `Type "texte"` | Tape du texte |
| `` Type `texte avec "guillemets"` `` | Tape du texte avec guillemets |
| `Enter` | Appuie sur Entrée |
| `Sleep 500ms` | Attend 500ms |
| `Backspace 3` | Efface 3 caractères |
| `Ctrl+C` | Envoie Ctrl+C |
| `Hide` / `Show` | Cache/montre les frames dans le GIF |
