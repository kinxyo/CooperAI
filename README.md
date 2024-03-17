# CooperAI

> [!IMPORTANT]
> Initially, it was just a college project that was using GPT model to generate repsonses, however, I intent to grow the scope by integrating a custom LLM.

## About 📖

This project is a basic implementation of a _AI therapy_ application. It calls upon GPT-3.5 turbo via API to provide the AI integration.

<br>
<br>

![CooperAIDemo](https://github.com/kinxyo/CooperAI/assets/90744941/2245bc6c-f23c-4422-89e0-4e0b93f15230)

<br>

## Tech Stack ⚙️

### **Nuxt** as Frontend

_Nuxt_ is a microframework for Vue.js. It is basically a fullstack framework however it provides a better frontend development experience than Vue or any other framework, so I use it for that.

### **Tauri** as Backend

_Tauri_ is a `Rust🦀` framework for creating desktop binaries for all major OS. Its concept is similiar to Electron however Tauri requires lesser storage and memory (making it superior in some people’s book 👀).

## Significance 🏆

Therapy, being the much needed thing in our society, still isn't reaching everyone that needs it. Common reasons being:

- Feeling that one's problems are trivial and not worth seeking therapy for.
- Preference of wanting to seek a more anonymous exchange rather than an in-person meet.
- Traditional therapy can be costly therefore it's not sustainable to seek it customarily.
- Lack of availability in one's locality.

So, to bridge the gap left by traditional therapy, this project was created.

> [!NOTE]
> Any Virtual AI Therapist should not replace human therapists entirely. It only serves to complement the traditional therapy. It cannot replicate the same level of human connection, empathy, and intuition that a human therapist can provide (at least not as of now).

![image](https://github.com/kinxyo/CooperAI/assets/90744941/29f52ed9-8e73-4c7a-a8f5-4aca5807825f)

---

### Closing

**Still not production-grade**.

#### Current Status 📝

This is temporary, but the current version (which is an iteration of the initial version) requires OpenAI's API key to work. You're supposed to generate API Keys then enter here. This obviously doesn't result in a good user experience so I am working on changing this.

At first, I was encrypting the API key for security reasons but then I realized that's useless.
API Key is stored only offline in `.env` file, so it's already pretty secure.
Besides, this system change in the next release.

##### Future Scope 🌠

- [ ] Improve UI & add some themes.
- [ ] Add my doggo's photo somewhere as a tribute.
- [ ] **Eliminate dependance of API for AI integration**.
- [ ] TSL/SSL certification for implementing HTTPS.
- [ ] locally provide the remaining assets.
- [ ] Update `Tauri` version to add the ability to drag windows.
