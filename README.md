# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder.
If you chose to develop with the router feature, you will also have a `views` folder.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # The entrypoint for the app.
│  ├─ components/
│  │  ├─ mod.rs # Defines the components module
│  │  ├─ hero.rs # The Hero component for use in the home page
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Tailwind

The project uses Tailwind CSS v3 for styling. To set up and compile the CSS:

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install Tailwind CSS v3:

```bash
npm install -D tailwindcss@^3
```

3. Generate the CSS file (one-time or when classes change):

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css
```

4. For development with auto-rebuild on file changes, use the `--watch` flag:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

**Note:** The project is configured to scan `./src/**/*.{rs,html,css}` and `./dist/**/*.html` files for Tailwind classes (see `tailwind.config.js`). If you add new Tailwind utility classes to your Rust components, re-run the Tailwind compiler to update `assets/tailwind.css`.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

