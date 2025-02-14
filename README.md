<h1 align="center"><code>trf</code>: Multimodal AI in the terminal</h1>

<h3 align="center">Supports OpenAI, DeepInfra, Google, Hyperbolic, and others</h3>

## Examples

- [Chat](#chat-in-bash)
- [Text to Speech](#text-to-speech-in-bash)

### Chat in Bash

We can chat straight from the command line.
For example, via the DeepInfra API:

```sh
$ DEEPINFRA_KEY="<KEY>"; echo "hi there" | trf chat
```

This defaults to the `meta-llama/Llama-3.3-70B-Instruct` model.
We can also create a Bash script to provide some default settings to the chat.
For example, create a file called `chat.sh` with the following content:

```bash
#!/usr/bin/env bash

export OPENAI_KEY="$(cat /path/to/key)"

trf chat --model="gpt-4o"
```

and add it to your PATH.
Now, we can use it like this:

```sh
$ echo "This is a test. Respond with 'hello'." | trf chat
hello
```

Or we can run a spellcheck on a file:

```sh
$ echo "Do you see spelling errors in the following text?"; cat myfile.txt | trf chat
```

Here is a more complex example.
For example, create a file called `writing-tips.sh` with the following content:

```bash
#!/usr/bin/env bash
set -euo pipefail

export DEEPINFRA_KEY="$(cat /path/to/key)"

PROMPT="
You are a helpful writing assistant.
Respond with a few suggestions for improving the text.
Use plain text only; no markdown.

Here is the text to check:

"
MODEL="deepseek-ai/DeepSeek-R1-Distill-Llama-70B"

(echo "$PROMPT"; cat README.md) | trf chat --model="$MODEL"
```

### Text to Speech in Bash

We can read a file out loud from the command line.
For example, with the OpenAI API:

```sh
$ OPENAI_KEY="$(cat /path/to/key)"; cat myfile.txt | trf tts | vlc - --intf dummy
```

Here, we set the key, print the file `myfile.txt` to stdout, pipe it to `trf` to generate mp3 audio, and pipe that to `vlc` to play it.
The `--intf dummy` is optional; it just prevents `vlc` from opening a GUI.

One way to make this easier to use is to create a Bash script that sets the environment variable and runs the command.
For example, create a file called `spk.sh` (abbreviation for "speak") with the following content:

```bash
#!/usr/bin/env bash

# Exit on (pipe) errors.
set -euo pipefail

export OPENAI_KEY="$(cat /path/to/key)"

trf tts | vlc - --intf dummy
```

After adding `spk.sh` to your PATH, you can use it like this:

```sh
$ cat myfile.txt | spk
```

### Other Text to Speech Commands

```sh
$ DEEPINFRA_KEY="$(cat /path/to/key)"; cat myfile.txt | trf tts | vlc -
```

```sh
$ DEEPINFRA_KEY="$(cat /path/to/key)"; cat myfile.txt | trf tts --output myfile.mp3
```

## Philosophy

The philosophy of this project is mainly to not handle state.
Like curl or ffmpeg, this should make it easier to use in scripts and to share examples online.
Settings are done via command line arguments and environment variables.
