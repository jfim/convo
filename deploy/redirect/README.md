# convo:// redirect service

A tiny, stateless HTTP service that 302-redirects a normal `https://` URL into a
`convo://` deep link:

```
https://<host>/claude-code/<slug>/<session>[#<anchor>]
        -> convo://claude-code/<slug>/<session>[#<anchor>]
```

## Why

Many applications (Obsidian, chat clients, ticket systems, ...) refuse to make a
custom-scheme link like `convo://` clickable. A plain `https://` link that
redirects into the scheme works everywhere, because the **browser** performs the
custom-scheme handoff — and it is allowed to do that on the user's machine.

You can self-host this, or point your links at someone else's instance. It is
the same for every user, holds no state, and never sees any conversation
content (conversations stay local to each user's machine).

## Security properties

This is safe to expose publicly, by design:

- **Not an open redirect.** The `convo://` scheme is hardcoded in the config, so
  the service can only ever redirect into the convo handler. It cannot be turned
  into a redirect to an arbitrary `https://` site, which is what makes open
  redirects useful for phishing.
- **Shape-validated.** Only `/claude-code/<slug>/<uuid>` is accepted; everything
  else returns 404, so the domain can't be used as an arbitrary `convo://`
  emitter.
- **No logging.** Access logging is turned off. The path contains visitors'
  project slugs (which reveal usernames / directory names) and session ids —
  data that is not yours to collect when hosting for others.
- **No conversation access.** The service only emits a string. It never reads
  any `.jsonl`; conversation files live only on each user's desktop. A full
  compromise of the box leaks nothing about anyone's conversations.
- **Defense in depth in the handler.** Even a malformed link that slips through
  is re-validated by the convo app, which rejects path traversal and only ever
  opens `~/.claude/projects/<slug>/<uuid>.jsonl`.

Note: the `<session>` in the URL only *names a local file path*; it grants no
access to the conversation. A leaked URL is not a data breach — at worst it lets
someone craft a link that, if the recipient clicks it, opens one of **their own**
local conversations.

## Hosting it

### 1. DNS

Point a hostname at the server:

```
convo.example.com.  A     203.0.113.10
convo.example.com.  AAAA  2001:db8::10     # if you have IPv6
```

### 2a. nginx

1. Copy [`nginx.conf`](nginx.conf) into your server config (e.g.
   `/etc/nginx/sites-available/convo`).
2. Set `server_name` and the two `ssl_certificate*` paths (a `*.example.com`
   wildcard cert is fine).
3. Add the rate-limit zone to the **`http { }`** context (e.g. in
   `/etc/nginx/nginx.conf`), since it cannot live inside `server { }`:
   ```nginx
   limit_req_zone $binary_remote_addr zone=convo:1m rate=10r/s;
   ```
4. Test and reload:
   ```sh
   sudo nginx -t && sudo systemctl reload nginx
   ```

### 2b. Caddy (alternative — automatic TLS, less to configure)

1. Put [`Caddyfile`](Caddyfile) where Caddy reads it; set the hostname.
2. Validate and reload:
   ```sh
   caddy validate --config /etc/caddy/Caddyfile
   sudo systemctl reload caddy
   ```
   Caddy obtains and renews the certificate automatically.

### 3. Verify

```sh
curl -sI "https://convo.example.com/claude-code/-home-user-proj/00000000-0000-4000-8000-000000000000"
# Expect:
#   HTTP/2 302
#   location: convo://claude-code/-home-user-proj/00000000-0000-4000-8000-000000000000

# A bad shape must 404:
curl -so /dev/null -w '%{http_code}\n' "https://convo.example.com/etc/passwd"   # -> 404
```

Anchors (`#<message-uuid>`) need no server support: browsers keep the fragment
client-side and re-append it to the redirect target automatically.

## Using it from the convo plugin

Once a redirect host is up, set `CONVO_REDIRECT_HOST` in your environment and the
`link` skill will emit `https://$CONVO_REDIRECT_HOST/claude-code/<slug>/<session>`
instead of the raw `convo://` link (which is the clickable form for Obsidian &
friends). Leave it unset to get the raw `convo://` link as before.
