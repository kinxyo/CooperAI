<script setup>
	/* IMPORTS */
	import { invoke } from "@tauri-apps/api";
	import { useMouse, useWindowSize } from "@vueuse/core";

	/* DECLARATIONS */
	const conversation = ref([]);
	const input = ref("");
	const { x, y } = useMouse();
	const { width, height } = useWindowSize();

	/* MISC. FUNCTIONS */
	const dx = computed(() => Math.abs(x.value - width.value / 2));
	const dy = computed(() => Math.abs(y.value - height.value / 2));
	const distance = computed(() =>
		Math.sqrt(dx.value * dx.value + dy.value * dy.value)
	);
	// const size = computed(() => Math.max(300 - distance.value, 50))
	// const opacity = computed(() => Math.min(Math.max(size.value / 300, 0.7),1))

	/* TAURI COMMANDS */
	const somevar = await invoke("greet", { name: "kinjalk" });

	/* API FUNCTIONS */
	async function sendQuery() {
		let convo = {
			user: input.value,
			ai: "",
		};
		const { data: response } = await useAsyncData("data", async () => {
			return await $fetch("http://127.0.0.1:8000/eliza", {
				method: "post",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					user_input: input.value,
				}),
			});
		});
		convo.ai = response.value;
		conversation.value.push(convo);
	}
</script>

<template>
	<main>
		<div
			:style="{
				// opacity: opacity,
				left: `${x}px`,
				top: `${y}px`,
				// width: `${size}px`,
				// height: `${size}px`,
			}"
			class="mouse-event"></div>

		<section class="header">
			<h1 style="color: palevioletred; font-size: xx-large">CooperAI</h1>
			<!-- <h3>{{conversation}}</h3> -->
		</section>

		<div class="main">
			<section class="content">
				<TransitionGroup name="res" tag="ul" class="ul">
					<li v-for="i in conversation" :key="i">
						<div class="linetaken-ai">
							<p>{{ i.ai }}</p>
						</div>
						<div class="linetaken-user">
							<p>{{ i.user }}</p>
						</div>
					</li>
				</TransitionGroup>
			</section>

			<section class="tools">
				<input
					placeholder="Chat with Cooper..."
					@keyup.enter="
						sendQuery();
						input = '';
					"
					type="text"
					v-model="input" />
				<button
					@click="
						sendQuery();
						input = '';
					">
					<Icon name="zondicons:send" color="black" />
				</button>
			</section>
		</div>
	</main>
	<section style="display: flex; justify-content: center; font-family:  Poppins; align-items: center;" class="footer">
		<h6> © CooperAI. All rights reserved </h6>		
	</section>
</template>

<style scoped>
	.mouse-event {
		position: absolute;
		border-radius: 9999px;
		height: 50px;
		opacity: 0.5;
		width: 50px;
		transform: translate(-50%, -50%);
		/* background-color: palevioletred; */
		filter: blur(20px);
		/* background: linear-gradient(270deg, #eb146a 0%, #2ed193 100%);  */
		pointer-events: none;
		background: radial-gradient(circle at center, #eb146a 0%, #2ed193 100%);
		background-size: 400% 400%;
		animation: pointerM 1s ease-out infinite;
	}
	@keyframes pointerM {
		0% {
			background-position: 0% 50%;
		}

		50% {
			background-position: 100% 50%;
		}

		100% {
			background-position: 0% 50%;
		}
	}

	h1 {
		font-family: Pathway Extreme;
	}
	main {
		/* border: 10px solid peru; */
		background-color: rgba(255, 255, 255, 0.2);
		border-radius: 15px;
		height: fit-content;
		width: 99%;
		display: flex;
		flex-direction: column;
		box-shadow: 1px 10px 20px rgba(28, 27, 27, 0.288);
		border: 1px solid rgba(240, 255, 255, 0.857);
	}
	.header {
		height: fit-content;
		text-align: center;
		border-top-left-radius: 20px;
		border-top-right-radius: 20px;
		background-color: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(40px);
	}
	.main {
		border-radius: 15px;
		backdrop-filter: blur(40px);
		/* margin-top: 20px; */
		height: 70vh;
		display: flex;
		gap: 10px;
		flex-direction: column;
		flex-wrap: nowrap;
	}
	.content {
		display: flex;
		flex-direction: column-reverse;
		flex-wrap: nowrap;
		flex: 0;
		max-width: 100%;
		min-height: 74%;
		max-height: 74%;
		padding: 18px;
		font-size: large;
		overflow: scroll;
	}
	.content::-webkit-scrollbar {
		display: none;
	}
	.ul {
		margin: 0;
		padding: 0;
	}
	li {
		list-style-type: none;
		display: flex;
		flex-direction: column-reverse;
		flex-wrap: nowrap;
		overflow: hidden;
	}
	.linetaken-user {
		display: flex;
		justify-content: right;
	}
	p {
		list-style-type: none;
		background-color: rgb(228, 232, 232);
		padding: 10px;
		margin: 0;
		border-radius: 15px;
		width: fit-content;
		font-size: medium;
	}
	.linetaken-user p {
		background-color: rgb(95, 168, 211);
	}
	.tools {
		display: flex;
		justify-content: center;
		/* border: 2px solid slateblue; */
	}
	input {
		padding: 10px;
		font-size: 1rem;
		border: 0.1px solid rgba(0, 0, 0, 0.195);
		background-color: rgba(255, 255, 255, 0.2);
		border-radius: 5px;
		transition: all 0.5s;
	}
	input:focus {
		outline: none;
		font-family: Space Grotesk;
	}
	input:hover {
		animation: pop 0.4s ease infinite alternate;
	}
	@keyframes pop {
		0% {
			transform: scale(1);
		}
		100% {
			transform: scale(1.04);
		}
	}
	button {
		border-radius: 10px;
		transform: scale(0.8);
		padding: 10px;
		font-size: 1.5rem;
		border: none;
		background-color: slateblue;
		color: white;
		cursor: pointer;
		transition: all 0.5s;
		cursor: none;
		margin-left: 5px;
	}
	button:hover {
		transform: scale(0.9);
		filter: brightness(0.8);
		cursor: none;
	}
	.res-enter-from {
		opacity: 0;
		transform: translateY(30px);
	}
	.res-enter-active {
		transition: all 0.5s;
	}
</style>

<style>
	body {
		padding: 5px;
		/* margin: 0; */
		background-color: rgb(178, 190, 200);
		/* background-color: rgb(24, 25, 26); */
		/* background-image: url("https://c4.wallpaperflare.com/wallpaper/108/140/869/digital-digital-art-artwork-fantasy-art-drawing-hd-wallpaper-thumb.jpg"); */
		color: #1f1d1d;
		font-family: Arial, Helvetica, sans-serif;
		overflow: hidden;
	}
	* {
		cursor: none;
	}
</style>
