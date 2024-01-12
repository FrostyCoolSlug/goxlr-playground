# OpenGoXLR Web UI

In the GoXLR Utility, the web content was handled completely independently of the utility itself, over time this caused
several problems:

* Updates to the WebUI had to be manually built and bundled into the daemon crate
* 'Point in Time' code was harder to manage (having to tag the UI branch as well as the utility)
* Contributors to the UI were often not seen on the Utility
* Bug tracking was 'split' between the two projects causing general confusion


## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Project Setup

```sh
npm install
```

### Compile and Hot-Reload for Development

```sh
npm run dev
```

### Compile and Minify for Production

```sh
npm run build
```

### Lint with [ESLint](https://eslint.org/)

```sh
npm run lint
```
