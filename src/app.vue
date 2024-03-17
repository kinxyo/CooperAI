<script setup>

	import { useMouse } from "@vueuse/core";

	const { x, y } = useMouse();

	/* When closing a topic */
	const darkMode = useState("darkMode", () => window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches);
	const block = useState("limit", () => false)

</script>

<template>
	
	<main class="body">
		
		<div :style="{ left: `${x}px`, top: `${y}px` }" class="new-mouse"></div>

		<NuxtLayout>
			<NuxtPage />
		</NuxtLayout>
	
	</main>

</template>

<style>
	.body {
		display: flex;
		flex-direction: column;	
		
		/* using `height` because I want min=max */
		height: 100vh; 
		height: 100dvh;
		
		overflow: hidden;
		
		border-radius: var(--radius);
		box-shadow: 1px 10px 20px rgba(28, 27, 27, 0.288);
		background-color: transparent;
		
		cursor: none;
	}

	.new-mouse {
		position: absolute;
		height: 25px;
		width: 25px;
		
		border-radius: 9999px;
		
		opacity: 0.7;
		filter: blur(5px);
		transform: translate(-50%, -50%);
		
		background: radial-gradient(circle at center, #eb146a 0%, #0fffa3 100%);
		background-size: 400% 400%;
		animation: newMouse 3s ease-out infinite;
		
		pointer-events: none;
	}

	@keyframes newMouse {
		0% { background-position: 0% 50%; }

		50% { background-position: 100% 50%; }

		100% { background-position: 0% 50%; }
	}

	/* TRANSITION ~ PAGE */
	
	.page-enter-active, .page-leave-active { transition: all 0.4s; }
	
	.page-enter-from, .page-leave-to { opacity: 0; filter: blur(1rem); }

	/* FONT */
	
	@font-face { font-family: 'Query'; src: url('/fonts/SpaceGrotesk.woff2') format('woff2'); }

</style>