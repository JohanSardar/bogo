# Stripe (test mode)

Lets your agent look at your Stripe test data, and makes it ask your phone
before it creates a payment link, refund, invoice, customer, product, price
or coupon, or changes a subscription.

```sh
bogo upstream add stripe
```

**Server:** Stripe's hosted MCP server, `https://mcp.stripe.com`. There is
nothing to install, and also **no version to pin**: Stripe changes the server
on its own schedule. Every other recipe pins its server, so this is an
exception. Any tool Stripe adds later needs approval until you choose
otherwise.

The recipe takes **test-mode keys only**: `sk_test_…` or `rk_test_…`. It
refuses a live key.

## What runs and what waits

| Waits for your phone |
|---|
| `create_payment_link`, `create_refund` |
| `create_invoice`, `finalize_invoice` |
| `create_customer`, `create_product`, `create_price`, `create_coupon` |
| `cancel_subscription`, `update_subscription` |

For the server's other tools, the recipe asks you, suggesting from Stripe's
annotations; with `--yes`, they all stay at approve. In BOGO the tools are
named `stripe__TOOL`.

## Sign in to Stripe

Take a test-mode key from
[dashboard.stripe.com/test/apikeys](https://dashboard.stripe.com/test/apikeys).
A restricted key (`rk_test_…`) with only the access the agent needs is best.
Give it to the recipe. It goes in `secrets.toml`, and BOGO sends it to Stripe
as the `Authorization` header.

## Check that it works

1. `bogo doctor` shows `ok    upstream stripe starts`.
2. Ask your agent to list your test customers, if you allowed that tool.
3. Ask it to create a $1 test payment link. Your phone shows
   `stripe__create_payment_link` with its arguments. Deny it, and no link is
   made.

## What has been checked

The recipe's policy and its test-key check are in the code. A live run
against Stripe has not been recorded. See [what is verified](../status.md).
