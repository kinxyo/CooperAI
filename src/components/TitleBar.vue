<script setup="ts">
	import { appWindow } from "@tauri-apps/api/window";

	const darkMode = useState('darkMode');

</script>

<template>
	<section class="header">
		<div class="titlebar" :style="darkMode === false && { background: '#222' }">
			
			<div class="left">
				<Icon
				name="icon-park-solid:red-cross"
				class="title-icon"
				style="margin-left: 6px; margin-top: 6px; transform: rotate(45deg);"
				@click="appWindow.close()"
				:color="darkMode ? 'rgba(208, 20, 67, 0.89)' : 'slateblue'" />
			</div>
			
			<div class="center">
				<p @click="navigateTo('/')">Cooper</p>
			</div>
			
			<div class="right">
				<Icon
				name="icon-park-solid:dark-mode"
				class="title-icon"
				style="margin-top: 8px; margin-right: 15px; align-self: flex-end; justify-content: end; transform: scale(1.1);"
				@click="darkMode = !darkMode"
				color="white" />
			</div>

		</div>
	</section>
</template>

<style scoped>
	/* CLASSES */

	.header, .titlebar {
		border-top-left-radius: var(--radius);
		border-top-right-radius: var(--radius);
		/* opacity: 0.9; */
	}

	.titlebar {
        display: flex;
        justify-content: space-between;
		align-items: center;
		
		width: 100%;
		padding-bottom: 0.4rem;
		
		backdrop-filter: blur(20px);
		
		user-select: none;
		
		background: linear-gradient(10deg, #1d163f 0%, #140f2c 50%, #1b1539 100%);
		background-size: 400% 400%;
		animation: TitleBarActive 3s ease infinite alternate-reverse;
    }

	.center { height: fit-content; }

	.center p {
		margin: 0;
		
		color: transparent;
		background: linear-gradient(to right, #eb146a 0%, #2ed193 100%);
		background-clip: text; /* always declared after `color` and `background` */
		
		font-weight: 700;
		
		user-select: none;
		opacity: 0.5;
	}

	.header {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;		
	}
	
	.title-icon {
		filter: brightness(0.5);
		cursor: pointer; 
	}

	/* ANIMATIONS */

	@keyframes TitleBarActive {
		0% { background-position: 0% 50%; }

		50% { background-position: 100% 50%; }

		100% { background-position: 0% 50%; }
	}

</style>
