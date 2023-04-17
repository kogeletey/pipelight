import type { Config, Pipeline } from "npm:pipelight";
import {
  packagingPipelines,
  parallelPackagingPipeline,
} from "./.pipelight/config/packages.ts";
import { uploadPipeline } from "./.pipelight/config/upload.ts";

const config: Config = {
  pipelines: [
    parallelPackagingPipeline as Pipeline,
    ...packagingPipelines,
    uploadPipeline,
    {
      name: "test",
      steps: [
        {
          name: "test",
          commands: ["cargo test --package pipeline"],
        },
      ],
      triggers: [
        {
          branches: ["master"],
          actions: ["pre-push"],
        },
      ],
    },
    {
      name: "test dev",
      steps: [
        {
          name: "test",
          commands: ["cargo test"],
        },
      ],
      triggers: [
        {
          branches: ["master, dev"],
          actions: ["pre-push"],
        },
      ],
    },
  ],
};

export default config;
