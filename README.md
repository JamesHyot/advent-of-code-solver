## About

This project is designed to be a Vuetify-powered website that solves the programming problems of [Advent of Code](https://adventofcode.com/) based on the input provided by the user. The UI is made with Vuetify but the solving part will happen with Rust WebAssembly, not because this is the best way but as a mean to grasp the benefits of doing so.

## Project setup

### First build the Rust code to WebAssembly (you might need to [install Rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installing-rustup-on-windows) first)

```
wasm-pack build
```

### Then restore the packages

```
# npm
npm install
```

### Compiles and hot-reloads for development

```
# npm
npm run dev
```

### Compiles and minifies for production

```
# npm
npm run build
```

### Deploy to GitHub Pages

```
./deploy.sh
```

### Customize configuration

See [Configuration Reference](https://vitejs.dev/config/).
