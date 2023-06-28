<script setup>
	/* IMPORTS */
    import { invoke } from "@tauri-apps/api";

	/* DECLARATIONS */
	const ai_bubble = ref([]);
	const user_bubble = ref([]);
	const input = ref("");

	/* MISC. FUNCTIONS */
	

	/* TAURI COMMANDS */
	const somevar = await invoke("greet", { name: "kinjalk" });	

	/* API FUNCTIONS */
	async function sendQuery() {
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

	// const { data: response, error } = await useFetch("http://127.0.0.1:8000/"); //✅

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
				<div class="ai-bubble">
					<transitionGroup name="res" id="ai-msg" tag="ul">
						<li v-for="i in ai_bubble" :key="i" class="chat">{{i}}</li>
					</transitionGroup>
					<transitionGroup name="res" id="user-msg" tag="ul">
						<li v-for="k in user_bubble" class="chat">{{k}}</li>
					</transitionGroup>
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
li {
	border: 2px solid green;
	margin-bottom: 20px;
}
main {
	border: 10px solid peru;
	border-radius: 15px;
	height: 80vh;
	padding: 5px;
	width: 700px;
	display: flex;
	flex-direction: column;
}
.header {
	/* border: 2px solid yellow; */
	height: fit-content;
	text-align: center;
}
.main {
	margin-top: 20px;
	/* border: 2px solid green; */
	display: grid;
	height: 100vh;
	display: flex;
	justify-content: space-between;
	flex-direction: column;
	gap: 20px;
}
#user-msg {
	margin-top: 45px;
}
.content {
	display: flex;
	flex-direction: column;
	flex-wrap: wrap;
	height: 80%;
	padding: 10px;
	font-size: larger;
	word-wrap: break-word;
	/* border: 2px solid orange; */
}
.ai-bubble {
	/* border: 2px solid slateblue; */
	display: flex;
	justify-content: space-between;
	/* flex-direction: row-reverse; */
}
.chat {
	list-style-type: none;
	border: 2px solid yellow;
	background-color: azure;
	padding: 10px;
	border-radius: 15px;
	width: fit-content;
}
.tools {
	/* border: 2px solid yellow; */
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
