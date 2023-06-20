import { cloudwatch, iam, lambda, s3 } from "@pulumi/aws";
import { output } from "@pulumi/pulumi";
import { envvars, base_name } from "./config";
import { lambdaUrl } from "./responder";
import { FileArchive } from "@pulumi/pulumi/asset";

const name = `${base_name}-recommender`;
const bucketArn = output(
  s3.getBucket({ bucket: envvars.SUGGESTION_HISTORY_BUCKET })
).arn;

const policy = new iam.Policy(name, {
  name,
  description: `Policy for ${name} lambda`,
  policy: {
    Version: "2012-10-17",
    Statement: [
      {
        Effect: "Allow",
        Action: [
          "s3:HeadObject",
          "s3:GetObject",
          "s3:ListBucket",
          "s3:PutObject",
        ],
        Resource: bucketArn.apply((arn) => [arn, `${arn}/*`]),
      },
      {
        Sid: "",
        Resource: "*",
        Action: [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents",
        ],
        Effect: "Allow",
      },
      {
        Action: ["ses:SendEmail", "ses:SendRawEmail"],
        Effect: "Allow",
        Resource: "*",
      },
    ],
  },
});

export const role = new iam.Role(name, {
  name,
  description: `Role used by the ${name} lambda.`,
  managedPolicyArns: [policy.arn],
  assumeRolePolicy: {
    Version: "2012-10-17",
    Statement: [
      {
        Effect: "Allow",
        Principal: { Service: "lambda.amazonaws.com" },
        Action: "sts:AssumeRole",
      },
    ],
  },
});

export const recommender = new lambda.Function(name, {
  name,
  handler: "bootstrap",
  memorySize: 128,
  runtime: "provided.al2",
  architectures: ["arm64"],
  role: role.arn,
  timeout: 5,
  reservedConcurrentExecutions: 1,
  code: new FileArchive("./placeholder.zip"),
  environment: {
    variables: {
      RESPONDER_URL: lambdaUrl,
      ...envvars,
    },
  },
});

// By default, Lambdas will retry twice for a total number of 3 executions
// when invoked asyncronously.
new lambda.FunctionEventInvokeConfig(name, {
  functionName: recommender.arn,
  maximumRetryAttempts: 0,
});

const eventRule = new cloudwatch.EventRule(name, {
  name,
  scheduleExpression: "cron(0 7 ? * SAT *)", // Every Saturday at 12 am PST
});

new cloudwatch.EventTarget(name, {
  rule: eventRule.name,
  arn: recommender.arn,
});

new lambda.Permission(name, {
  action: "lambda:InvokeFunction",
  function: recommender,
  principal: "events.amazonaws.com",
  sourceArn: eventRule.arn,
});
