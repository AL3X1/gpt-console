## GPT-Console

The Console application that performs queries to Chat GPT and return responses.
Can be used to safe time of investigational googling

## Setup

Before use the application you need to configure your GPT API key with Environment Variables.

With unix-based OS you can easily do it with command:

```export GPT_KEY=yourkey```

You can link application to PATH and use globally in terminal.

## Building and Execution
1. As main dependency you need to install Cargo

```curl https://sh.rustup.rs -sSf | sh```

2. run
   ```cargo build --release```
3. pick up binaries in `/target` directory.
4. run `gpt-console` binary with `./gpt-console`

**Note:** Application is not opening an actual new chat session,
but waiting for user queries and sends them to gpt api.
**Receiving response may took some time as it's not stream request.**

## Additional flags
To enable requests logging add `verbose` arg to gpt-console:
```./gpt-console verbose```