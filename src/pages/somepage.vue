<script setup>
  import { getAnswer } from "./repositories/chat";
  import { useChatStream } from "./composables/chat-stream.js";
  const messages = ref([]);
  const answer = ref(null);
  
  /* tauri commands */
  // const res = await invoke("greet", { name: "tauri" });
  // console.log(res);

  /* article copypaste */
  const question = ref("");
  const askQuestion = async () => {
    messages.value.push({
      role: "user",
      content: question.value,
    });
    question.value = "";
    const stream = await getAnswer({ messages: messages.value });
    answer.value = {
      role: "assistant",
      content: "",
    };
    useChatStream({
      stream,
      onChunk: ({ data }) => {
        answer.value.content += data;
      },
      onReady: () => {
        messages.value.push(answer.value);
        answer.value = null;
      },
    });
  };

  async function func1() {
    await fetch("/api/test").then((response) => response.json()).then((data) => console.log(data));
  }
</script>

<template>
  <section>
    <a href="/api/test">click here</a>
    <button @click="func1">TEST BACKEnd</button>
  </section>


  <form @submit.prevent="askQuestion">
    <ul>
      <li v-for="message in messages">
        {{ message.role }}: {{ message.content }}
      </li>
      <li v-if="answer">{{ answer.role }}: {{ answer.content }}</li>
    </ul>
    <div>
      <label>
        Question:
        <input v-model="question" type="text" />
      </label>
      <button type="submit">Ask</button>
    </div>
  </form>
</template>
