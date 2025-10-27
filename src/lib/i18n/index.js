import { register, init, getLocaleFromNavigator, locale } from 'svelte-i18n';

register('en', () => import('./en.json'));
register('it', () => import('./it.json'));

// Get initial locale synchronously
const getInitialLocale = () => {
  // Try to get from localStorage first (client-side)
  if (typeof window !== 'undefined') {
    const stored = localStorage.getItem('language');
    if (stored && (stored === 'en' || stored === 'it')) {
      return stored;
    }
  }
  // Fallback to 'en'
  return 'en';
};

init({
  fallbackLocale: 'en',
  initialLocale: getInitialLocale(),
});

// Ensure locale is always set
locale.subscribe((currentLocale) => {
  if (!currentLocale) {
    locale.set('en');
  }
});