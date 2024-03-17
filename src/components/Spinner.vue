<template>

	<div id="loading">
	
		<span class="loadingmsg">AI IS THINKING</span>
	
		<div class="spinner"></div>
	
	</div>

</template>

<style>
	:root {
		--spinner-size: .7rem;
	}

	.spinner {
		height: var(--spinner-size);
		width: var(--spinner-size);
		
		border-radius: 50%;
		border-bottom: 3px solid palevioletred;
		
		background-color: transparent;
		
		animation: Spin 1s linear infinite;
	}

	@keyframes Spin {
		from { transform: rotate(0turn); }
		to { transform: rotate(1turn); }
	}

	#loading {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: .6rem;
		
		height: fit-content;
	}

	.loadingmsg {
		font-size: small;
		font-weight: bolder;
		
		color: transparent;
		background: linear-gradient(to right, #eb146a 0%, #2ed193 100%);
		
		background-clip: text; /* always declared after `color` and `background` */
		
		background-size: 400% 400%; /* always declared after `background-clip` */
		animation: loadingState 1s ease infinite;
	}
	
	@keyframes loadingState {
		0% { background-position: 0% 50%; }

		50% { background-position: 100% 50%; }

		100% {background-position: 0% 50%; }
	}

</style>