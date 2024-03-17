<script setup>
const query = useState('query');

const darkMode = useState('darkMode');

const emit = defineEmits(['sendPressed']);

</script>


<template>

	<div style="display: flex; justify-content: center;">
		
		<textarea
		type="text"
		v-model="query" 
		placeholder="Chat with Cooper..."
		:class="darkMode ? 'dark' : 'light'"
		@keyup.enter="emit('sendPressed')" />
		
		<SendButton @click="emit('sendPressed')" />
	
	</div>

</template>