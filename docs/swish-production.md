# Setup Swish — Production

Going live with real payments requires more than flipping `SWISH_ENVIRONMENT` to `prod`. Remaining work:

- **Code**: `AppState::from` in `src/lib.rs` currently panics with `unimplemented!` when `use_swish_sandbox` is `false` — production cert loading isn't implemented yet.
- **Production merchant certificate**: obtained via [Swish Certificate Management](https://www.swish.nu) (login with Mobile BankID/BankID on card/BxID). Only the person(s) registered by the bank for this merchant/Swish number can log in and download it — this is an org/bank-agreement action, not something a developer can self-serve.
- **DigiCert Global Root G2 root CA**: must be trusted when validating Swish's production server TLS cert (different from the sandbox setup, where the provided `myCertificate.pem` doubles as both identity and root trust). Downloadable from Swish's production API docs.
- **Callback whitelisting / egress**: Swish's callback traffic to us originates from `egress.api.getswish.se` — relevant if firewalling inbound traffic. The callback endpoint (`/api/payment/swish/callback`) must be reachable over HTTPS on port 443 with a cert from a commonly recognized CA (Caddy's Let's Encrypt cert should satisfy this).
- **Verify existing cert material**: `certificates/Fysiksektionen-Merchant-1230810689-2024_04_16/` already contains a merchant p12/pem in the repo tree (git-ignored, never committed) — confirm it's current/unexpired production material before relying on it, or re-download from Certificate Management.
