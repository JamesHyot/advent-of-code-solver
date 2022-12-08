# default

## About

This project is designed to be a Vuetify-powered website that solves the programming problems of [Advent of Code](https://adventofcode.com/) based on the input provided by the user. The UI is made with Vuetify but the solving part will happen with Rust WebAssembly, not because this is the best way but as a mean to grasp the benefits of doing so.

## Project setup

```
# Build the Rust code to WebAssembly (you might need to [install Rust](https://doc.rust-lang.org/stable/book/ch01-01-installation.html#installing-rustup-on-windows) first)
wasm-pack build

# yarn
yarn

# npm
npm install

# pnpm
pnpm install
```

### Compiles and hot-reloads for development

```
# yarn
yarn dev

# npm
npm run dev

# pnpm
pnpm dev
```

### Compiles and minifies for production

```
# yarn
yarn build

# npm
npm run build

# pnpm
pnpm build
```

### Customize configuration

See [Configuration Reference](https://vitejs.dev/config/).
