> [!CAUTION]
> Depriciated packages | Project isn't in working state.


<div align="center"><img src="https://github.com/kinxyo/CooperAI/assets/90744941/e74a03da-cac1-4467-be82-0680976ddded" alt="CooperLogo" /> <p>Aims to compliment traditional therapy by leveraging AI.</p> <h1>Cooper</h1>  </div>

Therapy, being the much needed thing in our society, still isn't effectively reaching everyone that needs it.

**Common reasons being:**

- Feeling that one's problems are trivial and not worth seeking therapy for.
- Preference of wanting to seek a more anonymous exchange rather than an in-person meet.
- Traditional therapy can be costly therefore it's not sustainable to seek it customarily.
- Lack of availability in one's locality.

So, the purpose of this project was to try and bridge the gap left by traditional therapy.

<br>

![CooperDemo](https://github.com/kinxyo/CooperAI/assets/90744941/a39c5232-43e8-4901-977b-f09f66efa87c)

> [!NOTE]
> No AI should replace human entirely (except for journalist). Cooper only serves to complement the traditional therapy. <br>
> It cannot replicate the same level of human connection, empathy, and intuition that a human therapist can provide (at least not as of now 👀).

![CooperStaticDemo](https://github.com/kinxyo/CooperAI/assets/90744941/5a28def3-0bba-4266-a76a-cce579534f94)

---

## Build Steps

> [!NOTE]
> Use package manager of your choice.
> I'm using Bun for demonstration.

### Clone

```bash
git clone https://github.com/kinxyo/CooperAI.git
```

### Install modules

```bash
bun install
```

### Edit `tauri.conf.json`

```json
"build": {
    "beforeBuildCommand": "bun build",
    "beforeDevCommand": "bun run dev",
  },
```

### Run

```bash
bun run tauri dev
```
