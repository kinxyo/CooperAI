/* for testing from my local fastapi server */

interface Conversation {
    role: string;
    content: string;
}

	export default async function test_chat(input: string, conversation: Conversation[]) {
        const { data: response } = await useAsyncData("data", async () => {
			return await $fetch("http://127.0.0.1:8000/cooper", {
				method: "post",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					user_input: input,
				}),
			});
		});
		if (response) {
            const responseString = response as unknown as string;
            conversation.push({
                role: "assistant",
				content: responseString,
			});
		} else {
			conversation.push({
                role: "assistant",
				content:
                "No response from server :( check the server status",
			});
		}
    }