Generate a unit-tested backend in Rust using the fastest web framework Actix with AI agents communicating with each other using GPT-4

### Demo

Generates a full unit-tested actix backend using AI agents with gpt-4 model.

The demo builds an actix webserver that can track impressions and pageviews


https://github.com/AwakenedMind/actix-gpt/assets/43525127/ccde32b3-3db2-4299-9f36-0df719c344f4


### Create .env

```shell
touch .env
```

Within the .env file created, paste the following:

```plaintext
OPEN_AI_ORG=YOUR_OPEN_AI_ORG_ID
OPEN_AI_KEY=YOUR_OPEN_AI_KEY
```

### GPT-4 API Required

I have tried using gpt-3.5 model but it just isn't strong enough for the AI Agents to create the codebase, gpt-4 is required. I obtained access to the gpt-4 model by spending $1 in OpenAPI usage and when I paid for my first billing cycle OpenAI granted my account accesss to the gpt-4 model

### Update Paths

\*Update constants in the src/helpers/general path.

These should link to a code template which you want your web server to use and the main.rs file where it will attempt to execute new code it writes.

### Build Project

```shell
cargo build
```

### Run Project

```shell
cargo run
```

## ./web_template folder

AI Functions will place all backend code within this folder. By default it shows a todo list backend wri You can start the actix server normally

```shell
cargo build
```

### Run Project

```shell
cargo run
```
