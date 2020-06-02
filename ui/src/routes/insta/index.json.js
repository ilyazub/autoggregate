import { tmpdir } from "os";

import got from "got";
import Keyv from "keyv";
import { KeyvFile } from "keyv-file";

const username = "oksanazub_photography";

const cache = new Keyv({
  store: new KeyvFile({
    filename: `${tmpdir()}/keyv-file/insta/${username}.json`,
  }),
});

export async function get(req, res) {
  res.writeHead(200, {
    "Content-Type": "application/json",
  });

  const { body: posts } = await got(
    encodeURI(`https://www.instagram.com/${username}/?__a=1`),
    {
      cache,
      responseType: "json",
    }
  );

  const contents = JSON.stringify(
    posts.graphql.user.edge_owner_to_timeline_media.edges.map(({ node }) => ({
      id: node.id,
      image: node.display_url,
      slug: node.accessibility_caption,
    }))
  );

  res.end(contents);
}
