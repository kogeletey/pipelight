<span>
<h1>
<img width="125px" alt="pipelight_logo" src="https://pipelight.dev/images/pipelight.png"/>
<p>Pipelight - Automation pipelines but easier.</p>
</h1>
</span>

Pipelight can be used in so many ways that I keep this README cold-blooded and short as fuck
for you to quickly grab its concept.

You should checkout the [Documentation](https://pipelight.dev) for a much friendly approach and a deeper understanding.

## A lightweight software

Pipelight is a 6Mb binary, to be used in the terminal.

It aims to automates boring and repetitive tasks.

You fold your bash commands into a `Pipeline {Step { Command }}` written in **Typescript** (Yaml or Toml),
and it executes the pipeline on some events.

## Define pipelines

Create a `pipelight.ts` file on your project root directory.
Then use and combine your favorite syntax flavors.

### Option API

Use a verbose and declarative syntax.

```ts
const my_pipeline: Pipeline = {
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

### Composition API

Use the provided sweet shorthands, or make your owns.

```ts
const my_pipeline = pipeline("build website", () => [
  step("clean directory", () => ["rm -rf ./dist"]),
  step("build", () => ["pnpm install", "pnpm lint", "pnpm build"]),
]);
```

## Automatic triggers

Add automatic triggers to your pipeline.
Run tests on file change.
Push to production on new tag...

```ts
pipeline.trigger({
  tags: ["v*"],
  actions: ["watch", "pre-push"],
});
```

## Pretty and Verbose logs

```sh
pipelight logs -vvv
```

<img width="500px" alt="pretty logs" src="https://pipelight.dev/images/example_log_level_4.png"/>

## Try it fast (ArchLinux)

```sh
paru -S pipelight-git
```

```sh
touch pipelight.ts
```

Past this skeleton in your file.

```ts
// pipelight.ts
const my_pipeline: Pipeline = {
  name: "template",
  steps: [
    {
      name: "clean directory",
      commands: ["ls"],
    },
    {
      name: "build",
      commands: ["pwd"],
    },
  ],
};
export default {
  pipelines: [my_pipeline],
};
```

```sh
pipelight run template
```

```sh
pipelight logs
```

### Contacts and Community

Join the **[Discord server](https://discord.gg/swNRD3Xysz)**

Licensed under GNU GPLv2
Copyright (C) 2023 Areskul
