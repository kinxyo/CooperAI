import { invoke } from "@tauri-apps/api";

interface Conversation {
	role: string;
	content: string;
}

const limit = useState("limit");

export const chat_api = async (user_input: string, conversation: Conversation[]) => {

	/*
		`conversation` is an objects, and objects are passed by reference, not by value.
		`limit`, on the other hand, is a primitive hence it's passed by value, not by reference which is why `Ref<>` is needed.
	*/

	conversation.push({
		role: "system",
		content: "You're a mental-health therapist, you're really concerned about the growing agony in today's world, and intent to help people come out of it. You provide insightful responses to people's query. You way of communication is very natural and you avoid long paragraphs until needed.",
	});

	conversation.push({
		role: "user",
		content: user_input,
	});

	const response: string = await invoke("wrapper_answer", {
		chats: conversation,
	});

	if (response) {

		conversation.push({
			role: "assistant",
			content: response,
		});
		
		if (response.trim().toLowerCase() === 'topic closed. press `ctrl + r` to restart.') {
			console.log("blocking chat");
			limit.value = true;
			console.log(limit.value);
		}

	} else {
		conversation.push({
			role: "assistant",
			content:"New error unlocked! Backend is not responding with anything, reinstall the application.",
		});
	}
}