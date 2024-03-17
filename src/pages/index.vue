<script setup lang="ts">
	
	import { invoke } from "@tauri-apps/api";

	const darkMode = useState("darkMode")

	const token: ref<string> = ref("");
	const problem: ref<string> = ref("");

	async function register_token() {
		
		if (token.value) {

			try {
				await invoke("register_new_token", {
				token: token.value,
			});
		
			navigateTo("/cooper");
		
			} catch (response) {
				problem.value = response;
			}

			token.value = "";

		} else {
		
			problem.value = "Did not enter token!";
		
		}
	}

	/*
			
	//is this more readable?
	
	try {
		await invoke("register_new_token", { token: token.value });
		navigateTo("/cooper");
	} catch (response) { problem.value = response; }
	
	*/
</script>

<template>
	<main class="home" :style="darkMode === true && { backgroundColor: 'var(--primary-dark)', backgroundImage: 'radial-gradient(#31d722 0.5px, transparent 0.5px), radial-gradient(var(--primary-dark) 0.5px, var(--primary-dark) 0.5px)' } ">
		
		<div class="note">
			
			<h2>
				Note
				<span style="text-shadow: 0px 0px 10px rgb(188, 144, 31)"> ⚠️</span>
			</h2>
			
			<p>
				Currently, Cooper uses OpenAI's GPT model for generating response.
				<br />
				Hence, an <b>API key is needed</b> for Cooper to run. <br />
				This is not a release version, so expect this to change.
			</p>
		
		</div>

		<div class="session">
		
			<p style="color: crimson; text-align: center;" v-if="problem">{{ problem }}</p>

			<div style="display: flex; align-items: center">
				<input
					v-model="token"
					class="dark"
					type="password"
					placeholder="Enter API Key"
					@keyup.enter="register_token"
				/>
				<SendButton @click="register_token" />
			</div>
		
		</div>

		<div class="about" align="center">
			Work In
			<span class="credits">Progress</span>
		</div>
	
	</main>
</template>

<style scoped>

	/* CLASS STYLING */

	.home {
		display: grid;
		place-items: center;

		min-height: 100%;

		background-color: var(--primary-light);
		background-image: radial-gradient(#31d722 0.5px, transparent 0.5px), radial-gradient(var(--primary-dark) 0.5px, var(--primary-dark) 0.5px);
		background-size: 20px 20px;
		background-position: 0 0, 10px 10px;
	}

	.note {
		padding: 1rem;
		border-radius: 1rem;
		
		opacity: 0.9;
		box-shadow: inset 0px 0px 10px rgba(0, 0, 0, 0.5);
		
		background-size: 20px 35px;
		background-position: 0 0, 0 0, 10px 18px, 10px 18px, 0 0, 10px 18px;
		background-color: rgb(116, 89, 21);
		background-image: linear-gradient(
				30deg,
				#af7f18 12%,
				transparent 12.5%,
				transparent 87%,
				#af7f18 87.5%,
				#af7f18
			),
			linear-gradient(
				150deg,
				#af7f18 12%,
				transparent 12.5%,
				transparent 87%,
				#af7f18 87.5%,
				#af7f18
			),
			linear-gradient(
				30deg,
				#af7f18 12%,
				transparent 12.5%,
				transparent 87%,
				#af7f18 87.5%,
				#af7f18
			),
			linear-gradient(
				150deg,
				#af7f18 12%,
				transparent 12.5%,
				transparent 87%,
				#af7f18 87.5%,
				#af7f18
			),
			linear-gradient(
				60deg,
				#af7f1877 25%,
				transparent 25.5%,
				transparent 75%,
				#af7f1877 75%,
				#af7f1877
			),
			linear-gradient(
				60deg,
				#af7f1877 25%,
				transparent 25.5%,
				transparent 75%,
				#af7f1877 75%,
				#af7f1877
			);
	}

	.note h2 {
		padding: 0.6rem;
		border-radius: 1rem;
		
		text-align: center;
		font-weight: 900;
	}

	.about {
		padding: 0.5rem;
		border-radius: 0.5rem;

		width: 100%;
		
		background-color: black;
		color: white;

		transform: scale(0.8);
	}

	.credits {		
		color: transparent;
		background: radial-gradient(circle at center, #ed64a6 0%, #7f1dfe 100%);
		
		background-clip: text;  /* always declared after `color` and `background` */
		
		background-size: 200% 200%;  /* always declared after `background-clip` */
		animation: Highlighter 2s ease infinite;
	}

	/* ELEMENT STYLING */
	
	p {
		padding: 1rem;
		border-radius: 1rem;
		
		backdrop-filter: blur(10px);
		
		font-weight: 500;
		font-family: monospace;
	}

	/* ANIMATION */

	@keyframes Highlighter {
		0% { background-position: 0% 50%; }

		50% { background-position: 100% 50%; }

		100% { background-position: 0% 50%; }
	}
</style>