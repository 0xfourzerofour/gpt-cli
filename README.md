# ChatGPT CLI

This is a command-line interface (CLI) for ChatGPT, a language model trained by OpenAI that can answer questions and generate text.

The CLI provides the following subcommands:

    init: Initialize the CLI configuration with an API key for the ChatGPT service.
    reset: Start a new chat with the ChatGPT service and reset the chat log.
    new: Ask a new question to the ChatGPT service and receive an answer.
    change-model: Change the language model that the ChatGPT service is using.

## Usage

To use the ChatGPT CLI, first install the necessary dependencies by running:

```
cargo build
```

Then, you can run the CLI with:

```
cargo run -- <SUBCOMMAND> [ARGS]
```

Where <SUBCOMMAND> is one of the four subcommands listed above, and [ARGS] are any additional arguments required by the subcommand.

For example, to initialize the configuration with an API key, you would run:

```
cargo run -- init <API_KEY>
```

And to ask a new question to the ChatGPT service, you would run:

```
cargo run -- new --QUESTION "What is the meaning of life?"
```

Configuration

The ChatGPT CLI requires an API key to connect to the ChatGPT service. You can set your API key by running the init subcommand, as described above.

The CLI also saves a configuration file to your system to store your API key and chat log. By default, this file is created using the `directories` library
License

This code is licensed under the MIT License.