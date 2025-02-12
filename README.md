<h1 align="center"><code>ata</code>: Ask the Terminal Anything</h1>

<h3 align="center">Use AI in the terminal</h3>

## Philosophy

The philosophy of this project is mainly to not handle state.
Like curl or ffmpeg, this should make it easier to use in scripts and to share examples online.
Settings are done via command line arguments and environment variables.

## Examples

### Text to Speech

We can create a script to read a file out loud.
For example, with the OpenAI API:

```sh
$ OPENAI_KEY="$(cat /path/to/key)"; cat myfile.txt | ata --tts | vlc - --intf dummy
```

Here, we set the key, print the file `myfile.txt` to stdout, pipe it to `ata` to generate mp3 audio, and pipe that to `vlc` to play it.
The `--intf dummy` is optional; it just prevents `vlc` from opening a GUI.

One way to make this easier to use is to create a Bash script that sets the environment variable and runs the command.
For example, create a file called `spk.sh` (read: speak) with the following content:

```bash
#!/usr/bin/env bash

# Exit on (pipe) errors.
set -euo pipefail

export OPENAI_KEY="$(cat /path/to/key)"

cat "$1" | ata --tts | vlc - --intf dummy
```

After adding `spk.sh` to your PATH, you can use it like this:

```sh
$ spk myfile.txt
```
