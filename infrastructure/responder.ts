import { iam, lambda, apigateway, apigatewayv2, cloudwatch } from "@pulumi/aws";
import { envvars, base_name } from "./config";
import * as pulumi from "@pulumi/pulumi";
import { FileArchive } from "@pulumi/pulumi/asset";

const name = `${base_name}-responder`;

const role = new iam.Role(name, {
  name,
  assumeRolePolicy: JSON.stringify({
    Version: "2012-10-17",
    Statement: [
      {
        Action: "sts:AssumeRole",
        Effect: "Allow",
        Principal: {
          Service: "lambda.amazonaws.com",
        },
      },
    ],
  }),
});

new iam.RolePolicyAttachment(name, {
  policyArn: iam.ManagedPolicy.AWSLambdaBasicExecutionRole,
  role: role.name,
});

export const responder = new lambda.Function(name, {
  name,
  handler: "bootstrap",
  memorySize: 128,
  runtime: "provided.al2",
  role: role.arn,
  timeout: 60,
  reservedConcurrentExecutions: 1,
  code: new FileArchive("./placeholder.zip"),
  environment: {
    variables: {
      // TODO: pull from secrets manager
      TODOIST_API_TOKEN: "<unset>",
      ...envvars,
    },
  },
});

// By default, Lambdas will retry twice for a total number of 3 executions
// when invoked asyncronously.
new lambda.FunctionEventInvokeConfig(name, {
  functionName: responder.arn,
  maximumRetryAttempts: 0,
});

const functionUrl = new lambda.FunctionUrl(name, {
  functionName: responder.name,
  authorizationType: "NONE",
  cors: {
    allowCredentials: true,
    allowOrigins: ["*"],
    allowMethods: ["GET"],
    allowHeaders: ["date", "keep-alive"],
    exposeHeaders: ["keep-alive", "date"],
    maxAge: 86400,
  },
});

export const lambdaUrl = functionUrl.functionUrl;
export const lambdaArn = responder.arn;
