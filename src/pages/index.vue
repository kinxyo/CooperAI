<script setup>
	/* IMPORTS */
    import { invoke } from "@tauri-apps/api";

	/* DECLARATIONS */
	const conversation = ref([]);
	const input = ref("");

	/* MISC. FUNCTIONS */
	

	/* TAURI COMMANDS */
	const somevar = await invoke("greet", { name: "kinjalk" });	

	/* API FUNCTIONS */
	async function sendQuery() {
		let convo = {
			user: input.value,
			ai: "",
		};
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
		convo.ai = response.value;
		conversation.value.push(convo);
	}

</script>

<template>
	<main>
		<section class="header">
			<h1 style="color: palevioletred; font-size: xx-large;">Eliza</h1>
			<!-- <h3>{{conversation}}</h3> -->
			<hr>
		</section>

		<div class="main">
			<section class="content">
					<TransitionGroup name="res" tag="ul" class="ul">
						<li v-for="i in conversation" :key="i" class="chat">
							<div class="linetaken-ai">
							<p>{{i.ai}}</p>
							</div>
							<div class="linetaken-user">
								<p>{{i.user}}</p>
							</div>
						</li>
					</TransitionGroup>
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
	height: fit-content;
	width: 80%;
	padding: 5px;
	display: flex;
	flex-direction: column;
}
.header {
	height: fit-content;
	text-align: center;
}
.main {
	margin-top: 20px;
	height: 70vh;
	display: flex;
	gap: 10px;
	flex-direction: column;
}
.content {
	display: flex;
	flex-direction: column-reverse;
	flex-wrap: nowrap;
	flex: 0;
	max-width: 100%;
	min-height: 80%;
	max-height: 80%;
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
		padding: 10px;
		font-family: Arial, Helvetica, sans-serif;
	}
</style>
