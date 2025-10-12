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
1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install Tailwind CSS v3 (v4 is not yet fully compatible):
```bash
npm install -D tailwindcss@^3
```
3. Run the following command in the root of the project to compile Tailwind CSS:
```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css
```

Or use watch mode during development:
```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

**Note:** The generated `assets/tailwind.css` file is committed to the repository. You only need to regenerate it if you add new Tailwind utility classes to your components.

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

