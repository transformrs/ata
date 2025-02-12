<h1 align="center"><code>ata</code>: Ask the Terminal Anything</h1>

<h3 align="center">Use AI in the terminal</h3>

## Philosophy

The philosophy of this project is mainly to not handle state.
Like curl or ffmpeg, this should make it easier to use in scripts and to share examples online.
Settings are done via command line arguments and environment variables.

## Goals (not implemented yet)

Use the OpenAI API to generate audio from text (text to speech).

```sh
$ OPENAI_KEY=<KEY> cat myfile.txt | ata --tts
```

Here, `ata` will try to do `tts` (text to speech) on the input text.
Because only the `OPENAI_KEY` is provided, it will use some default OpenAI model to get the audio.

```sh
$ OPENAI_KEY=<KEY> cat myfile.txt | ata --tts --voice=ash
```
