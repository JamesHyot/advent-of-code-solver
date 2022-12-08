/**
 * plugins/index.ts
 *
 * Automatically included in `./src/main.ts`
 */
// Types
import type { App } from 'vue';

// Plugins
import { loadFonts } from './webfontloader';
import vuetify from './vuetify';

export function registerPlugins(app: App) {
  loadFonts();
  app.use(vuetify);
}
