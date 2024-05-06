import rookie from "@rookie-rs/api";

function createHeaders() {
  // Get all Github cookies from all browsers
  const cookies = rookie.load(["github.com"]);
  // Create cookie header
  const cookie = cookies
    .map((c) => decodeURIComponent(`${c.name}=${c.value}`))
    .join(";");
  const userAgent =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";
  return {
    "User-Agent": userAgent,
    Cookie: cookie,
  };
}

const headers = createHeaders();
const res = await fetch("https://github.com/settings/profile", { headers });
const html = await res.text();
// Parse username from HTML
const username =
  html.match(/<a href="\/(.+)" class="btn.+>/)?.[1] ??
  `Not Logged In. Response URL: ${res.url}`;
console.log(`Github username: ${username}`);
