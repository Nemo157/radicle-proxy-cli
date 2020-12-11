# radicle-proxy-cli

A CLI interface to the `radicle-proxy` subsystem of [Radicle
Upstream](https://github.com/radicle-dev/radicle-upstream).

**WARNING: The API of `radicle-proxy` is likely not considered stable, so
third-party tools like this connecting to it may break at any time.**

## Project Links

* Project: `rad:git:hwd1yre85oenojqfpzt1ihd36enmupg4dnb1o78rsos8h957bpxxbzn5g4a`
* Primary seed: `hybjnt4saqzz77zih7b6jp1ro8kzq3w6owh6nuawrkw6y6js4ssrd4@radicle.nemo157.com:4001` (<https://radicle.nemo157.com>)
* Discussion, Issues, PRs: `#radicle-proxy-cli:nemo157.com` ([via matrix.to](https://matrix.to/#/#radicle-proxy-cli:nemo157.com))

## Installation

First, have Radicle already working, and pull this project (what a recursive
dependency :think:).

Then install via the Radicle git repo:

```console
> PATH="$HOME/.radicle/bin:$PATH" \
  cargo install \
  --config=net.git-fetch-with-cli=true -Zunstable-options \
  --git rad://hyy7poyurp67tb14oa17e7d3wn4ieg4yg1r3h3q8kkc3prc7ux4hpy@hwd1yre85oenojqfpzt1ihd36enmupg4dnb1o78rsos8h957bpxxbzn5g4a.git \
  --branch prīmum
    Updating git repository `rad://hyy7poyurp67tb14oa17e7d3wn4ieg4yg1r3h3q8kkc3prc7ux4hpy@hwd1yre85oenojqfpzt1ihd36enmupg4dnb1o78rsos8h957bpxxbzn5g4a.git`
Password for 'rad://radicle@hwd1yre85oenojqfpzt1ihd36enmupg4dnb1o78rsos8h957bpxxbzn5g4a.git':
  Installing radicle-proxy-cli v0.1.0 (rad://hyy7poyurp67tb14oa17e7d3wn4ieg4yg1r3h3q8kkc3prc7ux4hpy@hwd1yre85oenojqfpzt1ihd36enmupg4dnb1o78rsos8h957bpxxbzn5g4a.git?branch=prīmum#c9da47b0)
    Updating crates.io index
   Compiling version_check v0.9.2
   Compiling syn v1.0.54
   Compiling libc v0.2.81
[...]
   Compiling cookie_store v0.12.0
   Compiling ureq v1.5.4
   Compiling radicle-proxy-cli v0.1.0 (/home/nemo157/.cargo/git/checkouts/_empty-aeefdeafa789b168/c9da47b)
    Finished release [optimized] target(s) in 45.38s
  Installing /home/nemo157/.cargo/bin/rad
   Installed package `radicle-proxy-cli v0.1.0 (rad://hyy7poyurp67tb14oa17e7d3wn4ieg4yg1r3h3q8kkc3prc7ux4hpy@hwd1yre85oenojqfpzt1ihd36enmupg4dnb1o78rsos8h957bpxxbzn5g4a.git?branch=prīmum#c9da47b0)` (executable `rad`)
```

## Setup

First, build and start `radicle-proxy`:

```console
> git clone https://github.com/radicle-dev/radicle-upstream
Cloning into 'radicle-upstream'...
[...]
Resolving deltas: 100% (11096/11096), done.
> cd radicle-upstream/proxy
> git checkout v0.1.4  # Latest release this has been tested against, you did read that warning above, right?
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `/home/nemo157/.cargo/shared-target/debug/radicle-proxy`
 INFO  coco::git_helper > Copied git remote helper to: "/home/nemo157/.radicle/bin/git-remote-rad"
 INFO  api::process     > starting API
``` 

Then, setup a keystore:

```console
TODO
```

Now you can start working with the CLI:

```console
> rad identities list
Please enter radicle passphrase:
🌟 Nemo157: rad:git:hwd1yrerta6rfsmdpfyqmn8n63cgw93hwe9obr8bb378ga9m1nek9qpfimy
```
