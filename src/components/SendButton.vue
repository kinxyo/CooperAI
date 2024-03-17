<!-- Source: Kevin Powell -->

<script setup>
	onMounted(() => {
        (function setGlowEffectRx() {
		const glowEffects = document.querySelectorAll(".glow-effect");

		glowEffects.forEach((glowEffect) => {
			const glowLines = glowEffect.querySelectorAll("rect");
			const rx = getComputedStyle(glowEffect).borderRadius;

			glowLines.forEach((line) => {
				line.setAttribute("rx", rx);
			});
		});
	})();
    });
</script>

<template>
	<button
		class="glow-effect p-2 w-full rounded-lg bg-slate-300 font-raleway text-rose-500 hover:text-slate-300"
        > 
      
        <Icon name="zondicons:send" color="black" />

		<svg class="glow-container">
			<rect pathLength="100" stroke-linecap="round" class="glow-blur"></rect>
			<rect pathLength="100" stroke-linecap="round" class="glow-line"></rect>
		</svg>
	</button>
</template>

<style scoped>

button {
    transition: all 0.5s ease-in-out;
}
button:hover {
    scale: 1.03;    
    background-color: var(--accent);
    box-shadow: 0px 0px 20px #ffffff33;
    transition: box-shadow 1s 1.6s ease-in-out;
}

.glow-effect {
  --glow-line-color: #fff;
  --glow-line-thickness: 2px;
  --glow-line-length: 20px;
  --glow-blur-color: #fff;
  --glow-blur-size: 5px;
  --glow-offset: 0px;
  --animation-speed: 1200ms;
  /* do not change, used for calculations */
  --container-offset: 100px;
  position: relative;
}

.glow-container {
  pointer-events: none;
  position: absolute;
  inset: calc(var(--container-offset) / -2);
  width: calc(100% + var(--container-offset));
  height: calc(100% + var(--container-offset));
  opacity: 0;
  /* outline: 3px solid blue; */
}

.glow-blur,
.glow-line {
  width: calc(100% - var(--container-offset) + var(--glow-offset));
  height: calc(100% - var(--container-offset) + var(--glow-offset));
  x: calc((var(--container-offset) / 2) + calc(var(--glow-offset) / -2));
  y: calc((var(--container-offset) / 2) + calc(var(--glow-offset) / -2));
  /* rx: 1.25rem; */
  fill: transparent;
  stroke: black;
  stroke-width: 5px;
  stroke-dasharray: var(--glow-line-length) calc(50px - var(--glow-line-length));
}

.glow-effect:is(:hover, :focus) :is(.glow-line, .glow-blur) {
  stroke-dashoffset: -80px;
  transition: stroke-dashoffset var(--animation-speed) ease-in;
}

.glow-line {
  stroke: var(--glow-line-color);
  stroke-width: var(--glow-line-thickness);
}

.glow-blur {
  filter: blur(var(--glow-blur-size));
  stroke: var(--glow-blur-color);
  stroke-width: var(--glow-blur-size);
}

.glow-effect:is(:hover, :focus) .glow-container {
  animation: glow-visibility ease-in-out var(--animation-speed);
}

@keyframes glow-visibility {
  0%,
  100% {
    opacity: 0;
  }
  25%,
  75% {
    opacity: 1;
  }
}

.glow-effect[data-glow-animation="false"] {
  --glow-line-length: 50px;
}
.glow-effect[data-glow-offset="true"] {
  --glow-offset: 10px;
}

.glow-effect[data-glow-animation="grow"]:is(:hover, :focus) .glow-container {
  scale: 1.3;
  transition: scale var(--animation-speed) linear;
}
</style>