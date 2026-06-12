# v0.8.59 Release Triage

Generated on 2026-06-12 for `Hmbown/CodeWhale`.

Current local release branch at handoff time:

- Branch: `codex/v0.8.59-release-ready`
- Head: `ca826bb2 fix(tui): clarify Codex response errors`
- Remote: `origin=https://github.com/Hmbown/CodeWhale.git`

## Mission

Move v0.8.59 toward release by actively landing, harvesting, or closing the
remaining PR and issue queue. Do not only summarize. Use GitHub and local Git
state to decide which items are already landed, which can be directly merged,
which need a credited harvest, and which should be deferred with a clear note.

Start from `codex/v0.8.59-release-ready` unless Hunter gives a newer branch.

## Key Operating Idea: Scratch Integration Branches

Hunter's "dummy branch" idea is feasible and useful. Use it.

The scratch branch is for discovery and acceleration:

- create it from the real release branch;
- merge or cherry-pick many candidate PR heads into it;
- use conflicts, tests, and diff review to learn what composes;
- harvest only the safe result back into the release branch.

The scratch branch is not the thing to ship. It can contain noisy merge commits,
temporary conflict resolutions, and interactions between unrelated PRs.

Suggested local workflow:

```bash
git fetch --prune origin
git switch codex/v0.8.59-release-ready
git pull --ff-only
git switch -c scratch/v0.8.59-pr-train-$(date +%Y%m%d-%H%M)
```

Fetch specific PR heads into temporary refs:

```bash
git fetch origin pull/3152/head:refs/tmp/pr-3152
git fetch origin pull/3148/head:refs/tmp/pr-3148
git fetch origin pull/3150/head:refs/tmp/pr-3150
```

Dry-run conflicts before touching the scratch branch:

```bash
base=$(git merge-base origin/codex/v0.8.59-release-ready refs/tmp/pr-3152)
git merge-tree "$base" origin/codex/v0.8.59-release-ready refs/tmp/pr-3152
```

On the scratch branch, merge one candidate at a time:

```bash
git merge --no-ff refs/tmp/pr-3152
```

If it conflicts, resolve only enough to learn whether the PR should be directly
merged, cherry-picked, harvested, or closed as already landed. After the scratch
experiment, return to the release branch and apply the final result as narrow
commits.

Use direct GitHub merge only when the PR is clean against the actual landing
branch. Many PRs are clean against `main` but conflict against
`codex/v0.8.59-release-ready`.

If a scratch branch needs CI, push it only as a clearly named scratch branch or
draft PR after Hunter authorizes that branch/CI lane. Never tag or publish from
scratch.

## Credit Contract

When harvesting contributor work:

- Preserve the original author when practical.
- Add `Co-authored-by` using `.github/AUTHOR_MAP` or:

```bash
gh api users/<login> --jq '"\(.id)+\(.login)@users.noreply.github.com"'
```

- Include this line in the commit body:

```text
Harvested from PR #N by @handle
```

The repo has `.github/workflows/auto-close-harvested.yml`, which closes PRs
after such commits land on `main`. Until then, leave clear comments linking the
release-branch commit or harvest plan.

## Recently Approved PRs Already Covered

`Hmbown` submitted formal `APPROVED` reviews on these PRs on 2026-06-12. A
follow-up scratch-merge pass against `codex/v0.8.59-release-ready` found that
the release branch already contains equivalent or stronger implementations. Do
not spend another cycle re-harvesting them; close them after the release branch
lands on `main`, or leave/update triage comments if they remain open.

### PR #3152 - SSE `data:` parsing

- URL: https://github.com/Hmbown/CodeWhale/pull/3152
- Author: `wgeeker`
- Title: `fix(SSE): accept SSE data lines without space after colon`
- Hunter approval: 2026-06-12T16:10:34Z
- GitHub merge state against `main`: `BLOCKED`
- Non-success checks shown by `gh`: none
- Head: `fix/sse-data-prefix-no-space`
- Commit: `50445eb3 fix: accept SSE data lines without space after colon`
- Files: `crates/tui/src/client.rs`, `client/anthropic.rs`, `client/chat.rs`,
  `client/responses.rs`
- Release-branch result: scratch merge produced a literal zero diff.
- Covered by release commit: `2f717d33 fix(tui): accept compact SSE data
  fields`.
- Notes: release branch already has an identical `extract_sse_data_value`
  helper and equivalent tests.
- Action: no harvest needed. Keep the triage comment as credit and close after
  the release branch lands on `main`.

### PR #3148 - `--model auto` / `DEEPSEEK_MODEL`

- URL: https://github.com/Hmbown/CodeWhale/pull/3148
- Author: `hongchen1993`
- Title: `fix(exec): resolve --model auto via DEEPSEEK_MODEL env and reorder ExecArgs`
- Hunter approval: 2026-06-12T16:10:21Z
- GitHub merge state against `main`: `CLEAN`
- Non-success checks shown by `gh`: none
- Head: `fix/auto-model-agent-plan`
- Commits: `b67f4add`, `55e66a6e`
- File: `crates/tui/src/main.rs`
- Release-branch result: fully covered.
- Covered by release commit: `862cb2e3 fix(exec): preserve auto model handoff`.
- Notes: `ExecArgs` is already reordered so the prompt is last, and the release
  branch env fallback is a superset: `CODEWHALE_MODEL`, then `DEEPSEEK_MODEL`.
- Action: no harvest needed. Keep the triage comment as credit and close after
  the release branch lands on `main`.

### PR #3150 - prompt source map / context usage

- URL: https://github.com/Hmbown/CodeWhale/pull/3150
- Author: `idling11`
- Title: `feat(context): add prompt source map and context-usage report (#3143)`
- Hunter approval: 2026-06-12T16:10:10Z
- GitHub merge state against `main`: `CLEAN`
- Non-success checks shown by `gh`: none
- Head: `feat/3143-prompt-source-map`
- Commits: `e91d4984`, `97e711c4`, `c463dac4`, `b8efcde0`
- Files include `crates/tui/src/context_report.rs`, command wiring,
  `compaction.rs`, `localization.rs`, `main.rs`, and TUI UI/app files.
- Closing issue: #3143
- Release-branch result: independently implemented.
- Covered by release commit: `0986cabb feat(tui): add context source map
  report`.
- Notes: release branch already has `/context [report|json|summary]` and
  `codewhale doctor --context-json`. Gemini's string-slicing and duplicate
  `AGENTS.md` findings apply to the PR implementation, not the release branch:
  the release module does no string slicing and no `AGENTS.md` processing. The
  PR's remaining tests target its divergent API and do not transplant cleanly.
- Action: no harvest needed. Keep the triage comment as credit and close after
  the release branch lands on `main`.

## Current v0.8.59 PR Queue

Open PRs carrying `v0.8.59` at the handoff scan, updated with the
release-branch scratch triage pass.

### Merge cleanly into the release branch (await Hunter approval)

- #3062 - fix(tools): apply strict mode per schema
- #3008 - docs(prompt): clarify Constitution trust framing
- #3006 - fix update release-download timeout/error messaging
- #3003 - bump `clap_complete` (CodeQL neutral)
- #3002 - bump `rustls` (CodeQL neutral)
- #3001 - bump `reqwest` (CodeQL neutral)
- #2971 - expose matched approval rule metadata
- #2943 - normalize macOS Cmd/SUPER to CONTROL

These can be landed directly or via a small scratch train, but they did not have
Hunter approval at the 2026-06-12 re-scan (reviewDecision empty on all). Ask for
approval or review each one before merging. Lowest-risk first: docs PR #3008,
update-network fallback #3006, then the three dependency bumps
#3001/#3002/#3003.

### Already landed on the release branch (verified by scratch merge, zero residual diff)

Triage comments with the covering commits were posted on each on 2026-06-12.
Close after the release branch reaches `main`.

- #2901 - localize ToolFamily labels. Zero unique commits vs release branch.
- #3056 - hotbar number keys. Covered by `3de1d35c` with stronger tests
  (extra `needs_redraw` assertions).
- #3052 - verbosity settings. Covered by `42de833d`, which is a superset
  (case/whitespace-tolerant `is_concise_verbosity`).
- #3011 - provider source tracking + unsupported-TUI errors. Release branch
  already has `ProviderSource::Cli/Env` and `provider_is_supported_by_tui`
  with per-source error messages.
- #3009 - cli-compare Harbor harness. Release-branch script is a strict
  evolution (adds `_first_present`/`_stable_path` helpers).
- #2895 - `siliconflow_cn` provider config field. Test merge produced zero
  residual diff.

### Genuinely missing work; needs review/rebase (commented 2026-06-12)

- #3051 - feat(voice): `/voice` speech-to-text. Release branch has NO voice
  command — this is a real 711-line feature. Conflicts in `commands/mod.rs`,
  `localization.rs`, `tui/ui.rs`, and it smuggles an unrelated
  `switch_provider` change (`app.api_key`/`app.base_url` reassignment) that
  collides with the release branch's `reasoning_effort.normalize_for_provider`
  line. Asked contributor to split/rebase; Hunter decides 0.8.59 vs next.
- #3005 - refactor(config): provider metadata registry. NOT landed — real
  ~300-line net simplification delta. One semantic conflict: release branch
  registers config key `siliconflow_cn`; PR uses `siliconflow` + aliases.
  Recommend deferring the refactor to the cycle after v0.8.59 unless Hunter
  wants it in; asked for rebase onto the release branch.

### Blocked against `main` or with failing checks

- #3053 - docs: add Upgrading from deepseek-tui section. Merge state: BLOCKED.
- #3013 - detect legacy deepseek/deepseek-tui binary and print migration
  instructions. Merge state: BLOCKED; macOS test failed and Windows test was
  cancelled at scan time.
- #2995 - bump actions/stale from 9 to 10. Merge state: BLOCKED; version drift
  failed and CodeQL was neutral.
- #2903 - build static linux x64 binaries with musl. Merge state: BLOCKED.

### Dirty / needs rebase, conflict triage, or harvest

- #3010 - exclude Calm personality overlay from default prompt path
- #2940 - localize Cmd command output messages
- #2932 - localize mode-picker messages
- #2929 - localize pending-input preview messages
- #2926 - localize onboard-welcome and app-mode-switch messages
- #2921 - localize sidebar panel labels, status messages, and focus indicators
- #2919 - localize ConfigEdit labels and default values
- #2918 - localize ConfigSection and ConfigScope labels
- #2899 - localize SubAgents surface
- #2894 - localize composer surface
- #2879 - align Hugging Face provider docs, errors, and tests
- #2851 - refactor TUI command groups into focused implementations; draft
- #2808 - runtime-api session save, undo/retry, and snapshot endpoints; dirty,
  `needs-human`, lint failing
- #2773 - complete provider fallback chain
- #2239 - i18n Phase 1-4b wiring plus rebase compile fixes

## Milestone Issues

At scan time, milestone `v0.8.59` had 76 open issues and 35 open PRs, for 111
open milestone items. Issue priorities:

1. #3063 release tracker and release-blocker coordination.
2. Close/update already-covered approved PRs #3152, #3148, #3150 after the
   release branch reaches `main`; do not re-harvest them.
3. User-visible bugs/regressions: #3064, #3067, #3065, #3080, #3088, #3095,
   #3094, #3070, #1812, #1679, #1190, #1120, #1060, #861, #759, #1920.
4. Provider/model catalog and cost/context: #3086, #3085, #3084, #3083, #3076,
   #3075, #3073, #3072, #3071, #3066, #2574, #1310, #868.
5. Tool and runtime UX: #3146, #3145, #3144, #3143, #3102, #3079, #2886,
   #1917, #1847, #1822, #1802, #1794, #1186.
6. TUI and command polish: #3081, #3077, #3074, #3069, #2870, #2791, #1871,
   #1722, #963.
7. Docs/localization/migration: #3093, #3092, #3091, #3090, #3087, #3068,
   #3061, #3058, #1447, #1118, #683.

For each issue, first ask: "Is this already satisfied on
`codex/v0.8.59-release-ready`?" If yes, prepare a closure/update note with the
commit hash. If no, implement or harvest the smallest safe slice.

## Suggested Next Work Session

Conflict triage of #3056/#3052/#3051/#3011/#3009/#3005 was completed on
2026-06-12 (scratch branch `scratch/v0.8.59-conflict-triage-20260612`,
local only). Remaining:

1. Get Hunter's explicit approval to land the eight release-branch-clean PRs:
   #3062, #3008, #3006, #3003, #3002, #3001, #2971, #2943.
2. Land those in small batches, starting with docs/dependencies/low-risk fixes,
   and run focused checks after each batch.
3. Hunter decisions needed: include #3051 (voice) in v0.8.59 or defer; defer
   #3005 (provider registry refactor) or reconcile the `siliconflow_cn` vs
   `siliconflow`+aliases key naming and land.
4. After the release branch reaches `main`, close the verified-covered PRs:
   #3152, #3148, #3150, #2901, #3056, #3052, #3011, #3009, #2895.
5. Triage the blocked PRs (#3053, #3013, #2995, #2903) and the dirty/localize
   train, then move to milestone issues per the priority list above.
6. Update `CHANGELOG.md` and close/update linked issues only after verification.

## Useful Commands

List PRs with v0.8.59:

```bash
gh pr list -R Hmbown/CodeWhale --state open --limit 200 \
  --search 'milestone:v0.8.59' \
  --json number,title,url,headRefName,baseRefName,mergeStateStatus,reviewDecision,isDraft,statusCheckRollup
```

Find formal reviews by Hunter on open PRs:

```bash
for n in $(gh pr list -R Hmbown/CodeWhale --state open --limit 200 --json number --jq '.[].number'); do
  gh pr view -R Hmbown/CodeWhale "$n" --json number,title,url,reviews,updatedAt \
    --jq '. as $pr | [.reviews[]? | select(.author.login == "Hmbown") | {number:$pr.number,title:$pr.title,url:$pr.url,prUpdatedAt:$pr.updatedAt,state:.state,submittedAt:.submittedAt}][] | [.number,.state,.submittedAt,.prUpdatedAt,.title,.url] | @tsv'
done
```

Check whether a PR has patch-equivalent commits already on the release branch:

```bash
git log --cherry-mark --right-only --oneline \
  origin/codex/v0.8.59-release-ready...refs/tmp/pr-3152
```

Show conflicts without modifying the worktree:

```bash
base=$(git merge-base origin/codex/v0.8.59-release-ready refs/tmp/pr-3152)
git merge-tree "$base" origin/codex/v0.8.59-release-ready refs/tmp/pr-3152
```
