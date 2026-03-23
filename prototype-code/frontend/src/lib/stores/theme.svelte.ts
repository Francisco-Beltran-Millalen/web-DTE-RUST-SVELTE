type Theme = 'light' | 'dark';

let theme = $state<Theme>('light');

export const themeStore = {
	get current(): Theme {
		return theme;
	},

	init() {
		const saved = localStorage.getItem('theme') as Theme | null;
		const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		theme = saved ?? (prefersDark ? 'dark' : 'light');
		document.documentElement.dataset.theme = theme;
	},

	toggle() {
		theme = theme === 'light' ? 'dark' : 'light';
		document.documentElement.dataset.theme = theme;
		localStorage.setItem('theme', theme);
	},
};
