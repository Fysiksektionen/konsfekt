# Deploying with router port forwarding

This assumes the app runs on local hardware behind a home router, with no public IP forwarded yet.

1. **Check you're not behind CGNAT.** Compare your public IP (`curl ifconfig.me`) against the WAN/internet IP shown in your router's admin page. If they don't match, your ISP isn't giving you a real public IP and port forwarding won't work — you'd need a tunneling service instead (see [Tunneling](../README.md#tunneling---develop-with-https)), or to request a public IP from your ISP.
2. **Point DNS** for `SITE_DOMAIN` at your public IP. If your ISP doesn't give you a static IP, use a dynamic DNS service instead.
3. **Log into your router's admin page** (usually `192.168.0.1` or `192.168.1.1` — check with `ip route | grep default`), using the router's admin credentials (often on a sticker on the device, or set by whoever configured it/your ISP).
4. **Add two port forwarding rules** (may be labeled "Port Forwarding", "NAT", or "Virtual Server"): external port `80` → this machine's LAN IP (`hostname -I`) port `80`, and external port `443` → this machine's LAN IP port `443`.
5. **Set `.env`** with `SITE_DOMAIN` matching the DNS record from step 2, and fill in the other required variables (see [Docker](../README.md#docker)).
6. Run `docker compose up --build`. Caddy should obtain its certificate automatically on first boot — watch its logs for `certificate obtained successfully`. If the ACME challenge fails, double check DNS propagation and that the forwarding rules actually reach this machine (a port-checking tool like `canyouseeme.org` on port 80 is useful here).
