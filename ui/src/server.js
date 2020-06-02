import sirv from "sirv";
import polka from "polka";
import compression from "compression";
import * as sapper from "@sapper/server";
import { v4 as uuidv4 } from "uuid";
import helmet from "helmet";

const { PORT, NODE_ENV } = process.env;
const dev = NODE_ENV === "development";

polka()
  .use((req, res, next) => {
    res.locals = {};

    res.locals.nonce = uuidv4();
    next();
  })
  .use(
    compression({ threshold: 0 }),
    sirv("static", { dev }),
    sapper.middleware(),
    helmet({
      contentSecurityPolicy: {
        directives: {
          scriptSrc: ["'self'", (req, res) => `'nonce-${res.locals.nonce}'`],
        },
      },
    })
  )
  .listen(PORT, (err) => {
    if (err) console.log("error", err);
  });
