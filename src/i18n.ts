import { register, init } from 'svelte-i18n';

import Locales from './ts/launcher/Locales';

register('en-us', () => Locales.get('en-us'));
register('de-de', () => Locales.get('de-de'));
register('uwu', () => Locales.get('uwu'));

Locales.default().then((locale) => {
    init({
        fallbackLocale: 'en-us',
        initialLocale: locale,
    });
});
