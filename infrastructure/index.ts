import { recommender } from "./recommender";
import * as responder from "./responder";

recommender.environment.apply((env) => {
  Object.entries(env?.variables || {}).forEach(([k, v]) => {
    console.log(`export ${k}="${v}"`);
  });
});

export const lambdaUrl = responder.lambdaUrl;
