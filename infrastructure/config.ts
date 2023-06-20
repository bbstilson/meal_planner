import * as pulumi from "@pulumi/pulumi";
const cfg = new pulumi.Config();

interface Envvars {
  RUST_BACKTRACE: string;
  SUGGESTION_HISTORY_BUCKET: string;
  SUGGESTION_HISTORY_KEY: string;
}

export const base_name = "meal-planner";
export const envvars = cfg.requireObject<Envvars>("envvars");
