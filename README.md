# Rust API o2switch

L'objectif de ce projet est de compiler et d'exécuter une API écrite en [Rust](https://www.rust-lang.org/), sur un hébergement [o2switch](https://www.o2switch.fr/).

## Éxecuter du code écrit en Rust

Pour ce qui est du back-end, o2switch ne supporte [officiellement](https://faq.o2switch.fr/faq/quels-sont-langages-supportes-php-node-ruby-python) que les langages : PHP, Node, Ruby, Python.

Une première idée était de compiler le programme Rust en WASM, pour pouvoir l'exécuter avec Node.
Mais Node.js, depuis la version 10.0, supporte l'exécution d'addons précompilés.

Cette solution offre de meilleures performances que le WASM, mais n'est pas aussi portable.
Ce n'est pas une contrainte dans notre cas.

Le Rust peut facilement être compilé en addons Node.js grâce à [napi.rs](https://napi.rs/).

## Déploiement sur o2switch

Pour que notre serveur puisse fonctionner correctement sur o2switch, il faut savoir qu'ils utilisent [Phusion Passenger](https://www.phusionpassenger.com/) pour gérer les différentes applications, sur un serveur.

Plus précisément, pour les applications Node.js, ils utilisent du ["reverse port binding"](https://www.phusionpassenger.com/library/indepth/nodejs/reverse_port_binding.html#).
Pour limiter les conflits et les attributions de ports sur le serveur hôte, le serveur doit écouter sur un socket unix "passenger".

```js
if (typeof PhusionPassenger !== "undefined") {
  PhusionPassenger.configure({ autoInstall: false });
}

if (typeof PhusionPassenger !== "undefined") {
  app.listen("passenger");
} else {
  app.listen(3000);
}
```

En réalité, Phusion Passenger va détecter cet appel, au niveau de Node, créer un socket unix spécifique et l'attribuer à notre serveur.

Étant donné que ce mécanisme se fait au niveau de Node, il n'est pas possible de faire écouter notre serveur Rust directement sur "passenger" comme notre serveur node le ferait.

La solution est donc de démarrer un serveur http node, de récupérer l'adresse du socket unix et de le fermer directement pour démarrer notre serveur Rust et le faire écouter sur ce même socket.

Cette solution a été testée et fonctionne correctement.

## Développement de l'API

L'API développée en Rust se trouve donc dans le dossier `src/`.
La fonction qui va être appelée par Node se trouve dans `src/lib.rs`.

Pour développer votre API, ajouter de nouvelles routes, vous pouvez vous référer à la documentation de la librairie que vous utilisez, dans mon cas [axum](https://docs.rs/axum/latest/axum/).

Il est normalement possible d'adapter ce projet à n'importe quelles autres librairies Rust qui permet de développer des APIs.
