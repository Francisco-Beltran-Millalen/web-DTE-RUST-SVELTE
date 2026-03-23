import adapter from '@sveltejs/adapter-static';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	kit: {
		adapter: adapter({
			// SPA mode: all routes fall through to 200.html
			fallback: '200.html'
		})
	}
};

export default config;
