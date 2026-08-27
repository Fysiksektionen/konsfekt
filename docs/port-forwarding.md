# Deploying with router port forwarding

This assumes the app runs on local hardware behind a home router, with no public IP forwarded yet.

1. **Check you're not behind CGNAT.** Compare your public IP (`curl ifconfig.me`) against the WAN/internet IP shown in your router's admin page. If they don't match, your ISP isn't giving you a real public IP and port forwarding won't work — you'd need a tunneling service instead (see [Tunneling](../README.md#tunneling---develop-with-https)), or to request a public IP from your ISP.
2. **Point DNS** for `SITE_DOMAIN` at your public IP. If your ISP doesn't give you a static IP, use a dynamic DNS service instead.
3. **Log into your router's admin page** (usually `192.168.0.1` or `192.168.1.1` — check with `ip route | grep default`), using the router's admin credentials (often on a sticker on the device, or set by whoever configured it/your ISP).
4. **Add port forwarding rules** (may be labeled "Port Forwarding", "NAT", or "Virtual Server") for whatever ports the host's HTTPS proxy listens on (typically external `80` and `443` → this machine's LAN IP, shown by `hostname -I`). The container itself only exposes plain HTTP on `8080`; terminating HTTPS is the host proxy's job, not this project's.
5. **Set `.env`** with `SITE_DOMAIN` matching the DNS record from step 2, and fill in the other required variables (see [Docker](../README.md#docker)).
6. Start the container with `docker compose up -d` and point the host proxy at `127.0.0.1:8080`. Verify the domain resolves and the forwarding rules reach this machine (a port-checking tool like `canyouseeme.org` is useful here).
