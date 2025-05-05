# MCP Server for Personal Mailer

An example project for a rust-based MCP ([Model Context Protocol](https://modelcontextprotocol.io/introduction)) server for personal mailer. It enables the users to send emails simply making prompts to the LLM chat interfaces such as Cursor and Copilot.

This is implemented by using the official [rust sdk](https://github.com/modelcontextprotocol/rust-sdk) for MCP (in active development).
It mainly uses the crate [lettre](https://crates.io/crates/lettre) as the SMTP client under the hood.

TODOs:
- [ ] add smtp administration
- [ ] support more request types, e.g. schedule meetings.
- [ ] add recipient management

## Local Run for Testing

This section describes how to run the MCP server locally for testing purposes. 

Prerequisites:
- python module `aiosmtpd` as the SMTP server
- visual studio code with github copilot as the LLM chat interface and the MCP client

Follow the steps:
1. Run the SMTP server.
    ```sh
    python -m aiosmtpd -n -l 127.0.0.1:2525
    ```
1. Follow this [guide](https://docs.github.com/en/copilot/customizing-copilot/extending-copilot-chat-with-mcp) to set up the MCP agent in your VSCode.
1. Start the MCP agent by clicking the "start" button in the file `mcp.json` in VSCode.
1. Run the MCP server:
    ```sh
    cargo run
    ```

Finally, try typing the prompt in the chat interface, e.g. "send an email to you@domain.com with subject 'test' and body 'hello world'".