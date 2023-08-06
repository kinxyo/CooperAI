<!-- I wish I had the time & energy to organize this -->

<script setup>
	/* IMPORTS */
	import { invoke } from "@tauri-apps/api";
	import { useMouse, useWindowSize } from "@vueuse/core";
	import { appWindow } from "@tauri-apps/api/window";

	/* DECLARATIONS */
	const conversation = ref([]);
	const user_input = ref("");
	const waiting = ref(false);
	const { x, y } = useMouse();
	const { width, height } = useWindowSize();
	const night = ref(true);
	const limit = ref(0);
	const stop = ref(false);

	/* COMPUTED PROPERTIES */
	const dx = computed(() => Math.abs(x.value - width.value / 2));
	const dy = computed(() => Math.abs(y.value - height.value / 2));
	const distance = computed(() =>
		Math.sqrt(dx.value * dx.value + dy.value * dy.value)
	);

	/* API FUNCTION */
	async function sendQuery() {
		if (!user_input.value) return;

		limit.value += 1;
		if (limit.value <= 12) {
			conversation.value.push({
				role: "system",
				content: "You're a mental health therapist",
			});

			conversation.value.push({
				role: "user",
				content: user_input.value,
			});

			/* rust */
			const response = await invoke("get_response", {
				chats: conversation.value,
			});
			if (response) {
				waiting.value = false;
				conversation.value.push({
					role: "assistant",
					content: response,
				});
			} else {
				waiting.value = false;
				conversation.value.push({
					role: "assistant",
					content:
						"Sorry, there seems to be an issue with the server. Please try again later.",
				});
			}
			/* fastapi (for testing) */
			// const { data: response } = await useAsyncData("data", async () => {
			// 	return await $fetch("http://127.0.0.1:8000/eliza", {
			// 		method: "post",
			// 		headers: {
			// 			"Content-Type": "application/json",
			// 		},
			// 		body: JSON.stringify({
			// 			user_input: input.value,
			// 		}),
			// 	});
			// });
			// if (response.value) {
			// 	convo.ai = response.value;
			// 	conversation.value.push(convo);
			// 	waiting.value = false;
			// } else {
			// 	convo.ai = "Sorry, there seems to be an issue with the server. Please try again later.";
			// 	conversation.value.push(convo);
			// 	waiting.value = false;
			// }
		} else {
			waiting.value = false;
			stop.value = true;
		}
	}
</script>

<template>
	<main
		:style="
			night === true && {
				backgroundColor: 'rgba(33, 29, 29, 0.985)',
			}
		">
		<div
			:style="{
				left: `${x}px`,
				top: `${y}px`,
			}"
			class="mouse-event"></div>

		<section class="header">
			<div class="titlebar">
				<Icon
					style="cursor: pointer"
					@click="appWindow.close()"
					id="close"
					name="icon-park-solid:red-cross"
					:color="night ? 'rgba(208, 20, 67, 0.89)' : 'slateblue'" />
				<Icon
					style="cursor: pointer"
					@click="night = !night"
					id="theme"
					name="icon-park-solid:dark-mode"
					color="black" />
			</div>
			<h1 style="font-size: xx-large">CooperAI</h1>
		</section>

		<div class="main">
			<section class="content">
				<TransitionGroup name="texts" tag="ul">
					<li v-for="i in conversation" :key="i">
						<p
							v-if="i.role === 'user'"
							:style="
								night === true && {
									backgroundColor: 'rgb(9, 80, 138)',
									color: 'rgb(171, 171, 171)',
								}
							"
							id="user">
							{{ i.content }}
						</p>
						<p
							v-if="i.role === 'assistant'"
							:style="
								night === true && {
									backgroundColor: 'rgb(171, 171, 171)',
									color: 'rgba(33, 29, 29, 0.985)',
								}
							"
							id="ai">
							{{ i.content }}
						</p>
					</li>
					<div id="loading" v-if="waiting">
					
						<span class="loadingmsg">AI IS THINKING</span>
						<div class="spinner"></div>
					
					</div>
					<p id="loading" v-if="stop">
						the developer of this app is not rich so he can't afford to pay for
						the api anymore. sorry for the inconvenience.
					</p>
				</TransitionGroup>
			</section>

			<section class="tools">
				<input
					placeholder="Chat with Cooper..."
					@keyup.enter="
						waiting = true;
						sendQuery();
						user_input = '';
					"
					type="text"
					v-model="user_input" />
				<button
					@click="
						waiting = true;
						sendQuery();
						user_input = '';
					">
					<Icon name="zondicons:send" color="black" />
				</button>
			</section>
		</div>
		<section
			style="
				display: flex;
				height: fit-content;
				justify-content: center;
				font-family: Poppins;
				align-items: center;
			"
			class="footer">
			<h6 :style="night === true && 'color: rgb(171, 171, 171)'">
				© CooperAI. All rights reserved
			</h6>
		</section>
	</main>
</template>

<style scoped>
	.spinner {
		height: 10px;
		width: 10px;
		border-radius: 50%;
		/* margin: 30px; */
		background-color: transparent;
		border-bottom: 3px solid palevioletred;
		animation: spin 1s linear infinite;
	}
	@keyframes spin {
		from {
			transform: rotate(0turn);
		}
		to {
			transform: rotate(1turn);
		}
	}
	#loading {
		display: flex;
		gap: 7px;
		height: fit-content;
		justify-content: center;
		align-items: flex-start;
		margin-left: 15px;
	}
	.loadingmsg {
		font-family:  Poppins;
		color: transparent;
		font-size: small;
		font-weight: bolder;
		background: linear-gradient(90deg, #fdc2be 0%, #cb103f 100%);
		background-size: 400% 400%;
		animation: loadingstate 1s ease infinite;
        background-clip: text;

		@keyframes loadingstate {
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

	}
	.titlebar {
		display: flex;
		justify-content: space-between;
	}
	#close {
		margin-left: 6px;
		margin-top: 6px;
		transform: rotate(45deg);
	}
	#close:hover {
		filter: brightness(0.5);
	}
	#theme {
		margin-top: 8px;
		margin-right: 15px;
		align-self: flex-end;
		justify-content: end;
		transform: scale(1.1);
	}

	.mouse-event {
		position: absolute;
		border-radius: 9999px;
		height: 50px;
		opacity: 0.5;
		width: 50px;
		transform: translate(-50%, -50%);
		filter: blur(20px);
		pointer-events: none;
		background: radial-gradient(circle at center, #eb146a 0%, #0fffa3 100%);
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
		color: transparent;
		background: linear-gradient(to right, #eb146a 0%, #2ed193 100%);
		background-clip: text;
		user-select: none;
	}
	main {
		background-color: rgba(255, 255, 255, 0.835);
		border-radius: 15px;
		height: 100vh;
		cursor: none;
		width: 100%;
		display: flex;
		flex-direction: column;
		box-shadow: 1px 10px 20px rgba(28, 27, 27, 0.288);
	}
	.header {
		height: fit-content;
		text-align: center;
		border-top-left-radius: 15px;
		border-top-right-radius: 15px;
		background-color: rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(40px);
	}
	.main {
		backdrop-filter: blur(40px);
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
	ul {
		margin: 0;
		padding: 0;
	}
	li {
		list-style-type: none;
		text-align: none;
		display: flex;
		flex-direction: column;
	}
	.yo {
		background-color: #eb146a;
	}
	#user {
		background-color: rgb(95, 168, 211);
		align-self: flex-end;
		margin-bottom: 8px;
		max-width: 400px;
		word-break: break-all;
	}
	#ai {
		margin-bottom: 8px;
		background-color: rgb(228, 232, 232);
		align-self: flex-start;
		max-width: 400px;
	}
	p {
		padding: 13px;
		margin: 0;
		border-radius: 15px;
		font-size: medium;
	}
	.tools {
		display: flex;
		justify-content: center;
	}
	input {
		padding: 10px;
		font-size: 1rem;
		border: 0.1px solid rgba(0, 0, 0, 0.195);
		background-color: rgba(255, 255, 255, 0.2);
		border-radius: 5px;
		transition: all 0.5s;
		cursor: none;
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
	.texts-enter-from {
		opacity: 0;
		transform: translateY(30px);
	}
	.texts-enter-active {
		transition: all 0.5s;
	}
	.texts-leave-to {
		opacity: 0;
		transform: scale(0.3);
	}
	.texts-move {
		transition: all 0.2s;
	}
</style>

<style>
	body {
		margin: 0;
		padding: 0;
		background-color: transparent;
		color: #1f1d1d;
		font-family: Arial, Helvetica, sans-serif;
		overflow: hidden;
	}
</style>
