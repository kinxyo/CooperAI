<script setup>

	const darkMode = useState("darkMode");

</script>

<template>
	
	<main style="background-color: aliceblue;" :style="darkMode === true && { backgroundColor: 'var(--primary-dark)' }" >

		<ChatView />

	</main>

</template>