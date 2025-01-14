<span>
<h1>
<img width="125px" alt="pipelight_logo" src="https://pipelight.dev/images/pipelight.png"/>
<p>Pipelight - Tiny automation pipelines.</p>
</h1>
</span>

You should checkout the [Documentation](https://pipelight.dev) for a much
friendly approach and a deeper understanding.

## Get help (fast)!

Pipelight is a young software and has become stable pretty recently. If you
encouter a bug or whatever difficulty, please open an issue or send a message on
[discord](https://discord.gg/swNRD3Xysz) or on telegram at
[@Areskul](https://t.me/areskul) or send a mail at areskul@areskul.com.

## A lightweight software

Pipelight is a 13Mb binary, to be used in the terminal.

It aims to automate boring and repetitive tasks.

You fold your bash commands into a `Pipeline{ Step{ Command }}` written in
**Typescript** (Yaml or Toml), and it executes the pipeline on some events.

## Define pipelines with a programming language

Create a `pipelight.ts` file on your project root directory. Then use and
combine your favorite syntax flavors.

Use a verbose and declarative syntax. (Objects API)

```ts
const my_pipeline = {
  name: "build_my_website",
  steps: [
    {
      name: "clean directory",
      commands: ["rm -rf ./dist"],
    },
    {
      name: "build",
      commands: ["pnpm install", "pnpm lint", "pnpm build"],
    },
  ],
};
```

Use the provided sweet shorthands, or make your owns. (Helpers API)

```ts
const my_pipeline = pipeline("build website", () => [
  step("clean directory", () => [`rm -rf ${build_dir}`]),
  step("build", () => ["pnpm install", "pnpm lint", "pnpm build"]),
  step("send to host", () => [`scp -r ${build_dir}`]),
  step("do stuffs on host", () => [
    ssh("host", () => ["systemctl restart nginx"]),
  ]),
]);
```

## Automatic triggers

Add automatic triggers to your pipeline. Run tests on file change. Push to
production on new tag...

```sh
# enable watcher and git hooks.
pipelight enable git-hooks
pipelight enable watcher
```

```ts
pipeline.add_trigger({
  tags: ["v*"],
  actions: ["pre-commit", "pre-push"],
});
```

## Pretty and Verbose logs

```sh
pipelight logs
```

<img width="500px" alt="pretty logs" src="https://pipelight.dev/images/log_level_error.png"/>

```sh
pipelight logs -vvvv
```

<img width="500px" alt="pretty logs" src="https://pipelight.dev/images/log_level_trace.png"/>

## Try it quick (ArchLinux)

Install

```sh
paru -S pipelight-git
```

Ensure the default configuration file.

```sh
pipelight init
```

Will generate this default typescript configuration file.

```ts
// pipelight.ts
import type { Pipeline } from "https://deno.land/x/pipelight/mod.ts";
const my_pipeline: Pipeline = {
  name: "example",
  steps: [
    {
      name: "list directory",
      commands: ["ls"],
    },
    {
      name: "get present working directory",
      commands: ["pwd"],
    },
  ],
};
export default {
  pipelines: [my_pipeline],
};
```

Try the harmless default pipeline

```sh
pipelight run
```

Explore logs

```sh
pipelight logs -vvvv
```

Licensed under GNU GPLv2 Copyright (C) 2023 Areskul
