<script setup>
	/* IMPORTS */
    import { invoke } from "@tauri-apps/api";

	/* DECLARATIONS */
	const ai_bubble = ref([]);
	const user_bubble = ref([]);
	const input = ref("");
	const session = ref(false);

	/* MISC. FUNCTIONS */
	

	/* TAURI COMMANDS */
	const somevar = await invoke("greet", { name: "kinjalk" });	

	/* API FUNCTIONS */
	async function sendQuery() {
		session.value = true;
		user_bubble.value.push(input.value);
		const {data: response} = await useAsyncData("data", async () => {
			return await $fetch("http://127.0.0.1:8000/eliza",{
				method: "post",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					user_input: input.value,
				}),
			})
		})
		ai_bubble.value.push(response.value);  
	}

</script>

<template>
	<main>
		<section class="header">
			<h1 style="color: palevioletred; font-size: xx-large;">Eliza</h1>
			<h3>{{greeting}}</h3>
			<hr>
		</section>

		<div class="main">

			<section class="content">
				<div v-show="session" class="chat-bubble">
					<TransitionGroup name="res" id="ai-msg" tag="ul" class="ul">
						<li v-for="i in ai_bubble" :key="i" class="chat">{{i}}</li>
					</TransitionGroup>
					<TransitionGroup name="res" id="user-msg" tag="ul" class="ul">
						<li v-for="k in user_bubble" class="chat">{{k}}</li>
					</TransitionGroup>
				</div>
			</section>
				
			<section class="tools">
				<input @keyup.enter="sendQuery(); input = ''" type="text" v-model="input" />
				<button @click="sendQuery(); input = ''">SEND</button>
			</section>
		
		</div>
	
	</main>
</template>

<style scoped>
main {
	border: 10px solid peru;
	border-radius: 15px;
	height: 80vh;
	padding: 5px;
	width: fit-content;
	display: flex;
	flex-direction: column;
}
.header {
	height: fit-content;
	text-align: center;
}
.main {
	margin-top: 20px;
	display: grid;
	height: 100vh;
	display: flex;
	justify-content: space-between;
	flex-direction: column;
	gap: 20px;
}
.content {
	display: flex;
	flex-direction: column-reverse;
	flex-wrap: wrap;
	height: 80%;
	padding: 18px;
	font-size: larger;
	word-wrap: none;
}
.chat-bubble {
	background: linear-gradient(90deg, #fc1b94 0%, #173736 100%); 
	padding: 10px;
	border-radius: 10px;
	backdrop-filter: opacity(0.5);
}
.ul {
	display: flex;
	justify-content: flex-end;
	flex-direction: column;
	margin: 0;
	padding: 0;
}
#user-msg {
	align-items: flex-end;
}
#ai-msg {
	justify-content: left;
	align-items: self-start;
	margin-right: 60px;
}
.chat {
	list-style-type: none;
	background-color: azure;
	padding: 10px;
	margin: 0;
	border-radius: 15px;
	width: fit-content;
}
.tools {
	text-align: center;
}
input {
	padding: 10px;
	font-size: 1.5rem;
	border: none;
	border-radius: 5px;
	transition: all 0.5s;
}
input:focus {
	outline: none;
}
input:hover {
	transform: scale(1.04);
}
button {
	padding: 10px;
	font-size: 1.5rem;
	border: none;
	background-color: slateblue;
	color: white;
	cursor: pointer;
	margin: 5px;
	transition: all 0.5s;
}
button:hover {
	transform: scale(1.09);
}
.res-enter-from {
	opacity: 0;
	transform: translateX(100px);
}
.res-enter-active {
	transition: all 0.5s;
}
</style>


<style>
	body {
		background-color: rgb(178, 190, 200);
		color: #1f1d1d;
		display: grid;
		place-items: center;
		padding: 20px;
		font-family: Arial, Helvetica, sans-serif;
	}
</style>
