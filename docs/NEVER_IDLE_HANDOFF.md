# Never idle: the six-hour wait loop

> **Status: enforced by a hook, not by this document.** An agent burned **six hours
> idling** inside a wait loop on 2026-07-17. The rule already existed in prose and was
> broken anyway — so the fix is `.claude/hooks/no-wait-loop.sh`, which denies poll
> loops outright. This file explains *why* that hook exists; the hook is what actually
> stops it.

---

## Why prose was not enough (read this first)

Two guards should have caught this and both failed:

1. **`CLAUDE.md` already said it**, in plain words: "DO NOT wait on the 'Your app is
   being rebuilt' overlay… The fix is almost always a **page refresh**, NOT waiting.
   Hours have been wasted waiting on a `dx` stuck state that one reload cleared." It
   was read. It was broken anyway, twice in one session. **A "please don't idle" does
   not stop an agent that has convinced itself this time is different.**

2. **`no-background.sh` already denies `run_in_background: true`** — its own comment
   names the exact failure mode ("PARK: it launches the task that never exits, ends
   its turn to 'wait' on it… The agent idles forever"). It did not fire, because it is
   gated on the `.claude/goal-active` marker, **and that marker did not exist**. A
   session-scoped Stop hook (`/goal`) never writes it. So the whole anti-park chain —
   `no-background.sh`, `no-yield.sh` — was inert while the Stop hook kept insisting the
   agent keep working.

`no-wait-loop.sh` is therefore **unconditional**: it does not ask whether a goal is
active, because a poll loop is never right in this repo regardless.

## What happened

The dev server was showing the `dx` "Dioxus Build" overlay and serving a stale wasm,
so the DOM still carried a class the source no longer had. Instead of reloading once
and moving on, the agent wrote a background wait loop:

```bash
# THE MISTAKE — do not do this, in any variation
until [ "$(stat -c %Y "$W")" -gt "$BEFORE" ]; do sleep 5; done
```

and another polling a URL until it answered. The conditions never became true. Under
an active `/goal` there is no user turn to interrupt the loop, so the session sat
there. **Six hours. Zero output.**

`CLAUDE.md` already forbade exactly this, in plain language:

> **DO NOT wait on the "Your app is being rebuilt" overlay.** … it will never
> "finish" if you sit polling `dev.log` for a new "Build completed" line. The fix is
> almost always a **page refresh**, NOT waiting. … Hours have been wasted waiting on
> a `dx` stuck state that one reload cleared.

The rule was written down, and read, and broken anyway — twice in the same session.

## The rule

**Never wait on the dev server. Not in the foreground, not in the background, not
"just this once".**

Forbidden, in every form:

- `until <check>; do sleep N; done` on a port, a URL, a file mtime, a build artifact.
- The same loop with `run_in_background: true`. **Backgrounding hides the idle, it
  does not fix it.** The loop still never exits, and the session still waits.
- `Monitor` armed on a rebuild.
- Chained sleeps, retry ladders, "one more poll".

## What to do instead

1. **Reload the page once.** That clears the stuck overlay in almost every case.
2. **Check the class string in the DOM**, not the rendered look — a stale build shows
   up as the DOM carrying a class the source no longer has. That is a one-line
   `browser_evaluate`, and it distinguishes "my change is wrong" from "the build is
   behind" instantly.
3. **If it is stale, it is stale.** Verify the *source* is right (`moon run :check
   --force`, read the file), say so, and **go do other work**. The dev server is the
   user's; it will catch up on its own.
4. **Never claim a stale-build observation is a bug in your change.** It is not
   evidence of anything.

## Why this is worse than a normal bug

A wrong line of code fails loudly and gets fixed. An idle loop produces *nothing* and
*says* nothing — it looks identical to work in progress. The goal hook cannot tell
the difference, the user cannot tell the difference until hours later, and every
minute is pure loss. That asymmetry is why this is a hard prohibition rather than a
preference.

## Related traps in this repo

- **A committed `[patch]` on `warcraft-data` breaks CI.** It is a development tool
  only. Tag `warcraft-data`, bump the pin, delete the patch.
- **While that patch is active, `moon` gives false greens.** The patched checkout is
  outside the workspace, so moon's cache never invalidates. `moon run :check` reported
  `to the moon` while three call sites could not compile. **Pass `--force`** for as
  long as the patch exists.
- **The debug wasm is ~177 MB.** Playwright can time out loading it. That is the
  environment, not your code — and still not a reason to poll.
