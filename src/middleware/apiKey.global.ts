import { invoke } from "@tauri-apps/api";

export default defineNuxtRouteMiddleware(async (to, from) => {
	
	const redirection: boolean = await invoke("middleware_layer");

	if (to.path === "/" && from.path !== "/cooper") {
	
		if (redirection) {
			console.log("Welcome back! :)");
			return navigateTo("/cooper");
		}
	
	}

	if (to.path === "/cooper") {
	
		if (!redirection) {
			
			console.log("OAI_KEY needed to proceed.");			
			
			return navigateTo("/");
		}
	
	}

});
