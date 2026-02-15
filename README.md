# MCP Server for Personal Mailer

An example project for a rust-based MCP ([Model Context Protocol](https://modelcontextprotocol.io/introduction)) server for personal mailer. It enables the users to send emails simply making prompts to the LLM chat interfaces such as Cursor and Copilot.

This is implemented by using the official [rust sdk](https://github.com/modelcontextprotocol/rust-sdk) for MCP (in active development).
It mainly uses the crate [lettre](https://crates.io/crates/lettre) as the SMTP client under the hood.

## Usage

This section describes some use cases to show that the MCP server is more interesting than the traditional email programs.


Traditional email programs require the user know how to input the data to the program in order to work (e.g. sending an email). Here, we can just use simple words and the LLM will help us to create the input data so that it can be fitted into the "program".

**Example 1:**

```text
send an formal email to Bob (bob@domain.com). invite him to my friend Chris's wedding. closing with my name "Alvin"
```

<details>
<summary>Sample Output</summary>

LLM:
```text
Email sent successfully!
```

SMTP:
```text
From: test@test.com
Subject: Invitation to Chris's Wedding
Content-Type: text/plain; charset=utf-8
To: bob@domain.com
Content-Transfer-Encoding: quoted-printable
Date: Sun, 18 May 2025 12:59:33 +0000
X-Peer: ('127.0.0.1', 58769)

Dear Bob,

I hope this message finds you well. I am writing to formally invite you to =
my friend Chris's wedding. It would be a pleasure to have you join us for t=
his special occasion.

Please let me know if you are able to attend.

Best regards,
Alvin
```

</details>


**Example 2:**

```text
add a email template named "greeting". It is a greeting to a new customer.
```

<details>
<summary>Sample Output</summary>

LLM:
```text
Email template created successfully!
```

Template:
```text
Hello {customer_name},

Welcome to our service! We are excited to have you as a new customer.

Best regards,
The Team
```

</details>

```text
send greeting email to customer "abc"
```

<details>
<summary>Sample Output</summary>

LLM:
```text
Email sent with template successfully!
```

SMTP:
```text
From: test@test.com
Subject: Welcome to our service!
Content-Type: text/plain; charset=utf-8
To: abc@abc.com
Content-Transfer-Encoding: 7bit
Date: Sun, 18 May 2025 12:28:52 +0000
X-Peer: ('127.0.0.1', 58333)

Hello abc,

Welcome to our service! We are excited to have you as a new customer.

Best regards,
The Team
```

</details>


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