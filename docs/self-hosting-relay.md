# Running your own relay

BOGO needs no relay setup: every install uses the free public relay,
`https://bogo.eilmer.si:8443`. The relay only ever sees ciphertext. It never
holds a key that opens a request, a decision or a pairing, so using the public
one costs you no privacy. Run your own if you want to control where those
ciphertexts live, or not rely on someone else's server.

Whichever way you run it, point BOGO at it with one setting in
`~/.bogo/bogo.toml`, then restart the daemon:

```toml
[relay]
url = "https://relay.example.com"
```

The phone needs no setting. While `bogo approver add` runs, the Mac's local
advert names your relay (`bogo1-…@relay.example.com`), and the phone uses it
for that Mac from then on. A phone already paired through another relay keeps working only
on that relay, so add the Mac again after switching.

## On a VPS (Docker)

Needs Docker with Compose, ports 80 and 443 open, and a DNS name pointing at
the machine. With no domain, use `<your-ip-with-dashes>.sslip.io` (for example
`203-0-113-7.sslip.io`), which already resolves to that address.

```sh
curl -fsSLO https://raw.githubusercontent.com/JohanSardar/bogo/main/deploy/relay/compose.yaml && DOMAIN=relay.example.com docker compose up -d
```

This builds the relay from this repository and puts [Caddy](https://caddyserver.com)
in front of it, which gets and renews a Let's Encrypt certificate on its own.
Then set `url = "https://relay.example.com"`.

## On a cloud service

Any container service that gives you an HTTPS address works (Google Cloud Run,
Fly.io, Render, Railway and others). Build the image from this repository:

```sh
docker build -t bogo-relay -f deploy/relay/Dockerfile https://github.com/JohanSardar/bogo.git#main
```

and run it on the service with port 8080, the environment variable
`BOGO_RELAY_SERVICE` set to the service's host with `:443` (for example
`bogo-relay-abc123.a.run.app:443`), one instance (pairing rooms live in the
relay's memory), and, if the service has one, a persistent volume at `/data`. Without a volume the relay forgets undelivered messages when it
restarts; the Mac and phone send them again. Set `url` to the HTTPS address
the service gives you.

## On a Mac

From this repository, with a free [Cloudflare quick tunnel](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/do-more-with-tunnels/trycloudflare/)
giving it an HTTPS address (install `cloudflared` first, for example with
`brew install cloudflared`):

```sh
cargo run --release -p bogo-relay & cloudflared tunnel --url http://127.0.0.1:8080
```

`cloudflared` prints an `https://….trycloudflare.com` address; set `url` to
it. A quick tunnel's address changes every time it starts, so this suits
trying things out. For a lasting address use a named Cloudflare tunnel or
Tailscale Funnel and give the relay that name, for example
`cargo run --release -p bogo-relay -- --service my-mac.example.ts.net:443`,
or run the relay with your own certificate:
`bogo-relay --listen 0.0.0.0:8443 --service relay.example.com:8443 --cert cert.pem --key key.pem`.

## Check it works

```sh
curl https://relay.example.com/healthz
```

answers `{"status":"ok","push":false}`. `push` says whether it wakes phones.

## Waking phones

A relay wakes phones only with an APNs push key for the app's bundle ID. The
public relay has one. Yours can only use one from the Apple developer team that
publishes the app, so for the TestFlight app there are two choices: give the
Mac that key in `[apns]` (see [configuration](configuration.md)) and it sends
its own pushes through any relay, or go without pushes and open BOGO to see
waiting requests. If you build and sign the app yourself, give your relay your
own key:

```sh
bogo-relay --apns-key AuthKey.p8 --apns-key-id KEYID --apns-team-id TEAMID --apns-topic your.bundle.id
```

or set `BOGO_RELAY_APNS_KEY`, `BOGO_RELAY_APNS_KEY_ID`,
`BOGO_RELAY_APNS_TEAM_ID` and `BOGO_RELAY_APNS_TOPIC`. The relay reads the key
once at start and never logs it. Each push says only "Approval requested".

## What the relay does with no flags

`bogo-relay` alone serves plain HTTP on `127.0.0.1:8080`, keeps its SQLite
database in `./bogo-relay.sqlite`, keeps undelivered messages for 30 days and
at most 1000 per mailbox. Put HTTPS in front of it, or give `--cert` and
`--key` to serve HTTPS itself. Sign-ins are bound to the name clients use:
give it with `--service host:port` (or `BOGO_RELAY_SERVICE`). Only a relay on
loopback behind something that routes by the requested hostname, such as a
Cloudflare quick tunnel, may leave it out and take the name from each
request; behind anything that forwards other hostnames (Tailscale Funnel, a
catch-all proxy), give `--service`. `bogo-relay --help` lists every flag. The
public relay's own deployment is in [deploy/relay](../deploy/relay/README.md).
