# Auto Gippty
### A ChatGPT-powered CLI tool that generate backend code for you! Including APIs, and schemas.

# How to run
1. Create .env file
2. Within the .env file, pasting in your OpenAI credentials:
```plaintext
OPEN_AI_ORG=YOUR_OPEN_AI_ORG_ID
OPEN_AI_KEY=YOUR_OPEN_AI_KEY
```
3. Update Paths
Update constants in the src/helpers/general path.

These should match where you have your web_template project saved. Recommend to save your web_template in the same folder as this project.

Web template project: https://github.com/coderaidershaun/rust-web-server-template.git

These should link to a code template which you want your web server to use and the main.rs file where it will attempt to execute new code it writes.

4. Run `cargo build` and `cargo run`
