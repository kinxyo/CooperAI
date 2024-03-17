export default defineNuxtConfig({
	devtools: { enabled: false },
	ssr: false,
	srcDir: "src",
	css: ['~/assets/css/main.css'],
	routeRules: {
		"/": { prerender: true },
		"/cooper": { prerender: true },
	},
	modules: ["nuxt-icon"], //these icons need internet to load, so I have to arrange an offline setup.
	app: {
		pageTransition: { name: "page", mode: "out-in" },
	},
});
