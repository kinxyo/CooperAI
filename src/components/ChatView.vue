<script setup lang="ts">

/* darkMode */
const darkMode = useState('darkMode');

/* AI Status */
const ai_thinking = ref(false);

/* AI Data */
interface Conversation {
	role: string;
	content: string;
}
const conversation: Ref<Conversation[]> = ref([]);
const query = useState('query', () => "");

const block = useState("limit");

async function sendQuery() {
	
	if (!query.value) return;

	ai_thinking.value = true;
	
	let prompt = query.value; 
	query.value = ''; 
	
	await chat_api(prompt, conversation.value);
	// test_chat(query.value, conversation.value);
	
	ai_thinking.value = false;
	console.log("meanwhile: ", block.value);
}

</script>

<template>
	<main>
		<section class="content" :style="darkMode === true && { backgroundImage: 'radial-gradient(#444cf7 0.5px, var(--primary-dark) 0.5px)' }" >
			
			<TransitionGroup name="texts" tag="ul">
				<li v-for="(i, index) in conversation" :key="index">
					<p 
					
					id="user" 
					v-if="i.role === 'user'" 
					:style="darkMode === true && { backgroundColor: 'rgb(17, 88, 105)', color: '#e2e8f0' }" >
					
					{{ i.content }}
					
					</p>
					
					<p 
					
					:class="{ 'error-message': i.content.trim().toUpperCase().startsWith('ERROR:') }"
					id="ai" 
					v-if="i.role === 'assistant'"
					:style="darkMode === true && { backgroundColor: '#334155', color: '#cbd5e1' }" >
						{{ i.content }}
					
					</p>
				</li>
				
				<Spinner v-if="ai_thinking" />
				
			</TransitionGroup>
			
		</section>

		<Tools v-if="!block" @sendPressed="sendQuery" />

	</main>
</template>

<style scoped>

	/* ELEMENT STYLING */

    main {
		display: flex;
		flex-direction: column;
		justify-content: space-between;

		min-height: 94vh;
		max-height: 94vh;
		min-height: 94dvh;
		max-height: 94dvh;

		overflow: hidden;

		padding: 1rem;
		
	}
	
	.content {
		display: flex;
		flex-direction: column-reverse;
		flex-wrap: nowrap;
		flex: 0;
		
		overflow: scroll;

		min-height: 78vh;
		max-height: 78vh;		
		min-height: 78dvh;
		max-height: 78dvh;		

		padding: 1rem;
		border-radius: 1rem;
		box-shadow: inset 0px 0px 10px rgba(0,0,0,0.5);
		
		background-image:  radial-gradient(#31d722 0.5px, transparent 0.5px), radial-gradient(var(--primary-dark) 0.5px, #e8ecf0 0.5px);
		background-size: 20px 20px;
		background-position: 0 0,10px 10px;

	}
	
	
	ul {
		padding: 0;
		margin: 0;
	}
	
	li {
		display: flex;
		flex-direction: column;
		
		text-align: none;
		list-style-type: none;
	}

	p {
		padding: 13px;
		margin: 0;
		margin-bottom: 8px;
		max-width: 400px;
		
		height: fit-content;
		font-size: var(--chat-size);
		border-radius: 15px;
		border: 1px solid rgba(0, 0, 0, 0.114);
		
		transition: all 0.2s ease-in;
	}

	.error-message {
 	   background-color: rgb(148, 5, 34) !important;
	   color: var(--slate-300);
	}


	/* ID's */

	#user {
		align-self: flex-end;
		word-break: break-all;
		
		background-color: rgb(95, 168, 211);
	}

	#ai {
		align-self: flex-start;
		
		background-color: rgb(228, 232, 232);
	}
	
	/* TRANSITIONS */
	
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