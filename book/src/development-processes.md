# Development Processes

## CI (Continuous Integration)

There are three main CI workflows that are relevant for PRs: `CI`, `Build`, and docs preview.

### The `CI` Workflow

The `CI` workflow runs various checks and linters and runs host-side crate tests.
It is relatively quick (usually runs in 3 minutes or less, with many checks running in just a few seconds), and is run for *every* PR.

> Before requesting a review, linting errors should usually be addressed.

### The `Build` Workflow

The `Build` workflow is more involved and makes sure every application in `examples/` and `tests/` compiles successfully for every builder in the configured set of laze builders.

#### PR Labels

The set of builders is selected by attaching a `ci-build:*` label to the PR.
If no such label is attached, the workflow will fail *quickly*.
On top of a label for each MCU (sub)family, there are a few other labels with different sizes of sets: currently these are `ci-build:small` and `ci-build:full`.
The `ci-build:skip` label additionally allows to skip almost everything in the `Build` workflow.
As this workflow is more costly and takes much longer (about 10–45 minutes depending on the set of builders and on runner contention), selecting the right label is important to make sure the changes compile correctly while managing the load on the runners.

> When opening a draft PR, it is recommended to *not* attach a label at first, to limit the load on the runners.
> *Before* marking the PR as ready for review, a label should be attached (and the workflow manually re-run through the web interface) to check that the changes compile as expected: as this can take some time, not *all* jobs need to be green yet, but at least a few should.
> When re-running the workflow, it is essential to select “Re-run all jobs” instead of “Re-run failed jobs” so that the `check-labels` job re-runs, otherwise its output—the attached label—might get incorrectly reused.
>
> The `ci-build:skip` label can be used for documentation and comment-only PRs.
> As it completely skips compiling the applications, it should be used carefully, and the reviewer(s) must make sure it is used appropriately.
>
> If the changes affect only one of the MCU (sub)families, the corresponding label can be used: it will compile for every builder in that MCU (sub)family.
>
> If the changes can affect multiple HALs, the `ci-build:small` label should currently be used.
>
> In some cases, changes are unlikely enough to affect the HALs differently but are not specific to one of the HAL either: in this case the label with the fewest builders can be used—currently this is `ci-build:rp`.
>
> Except in special circumstances, the same label should be used for the pre-merge CI run, and the merge queue/`main` runs: i.e., it should *not* be changed right before merging the PR.

#### Caching Behavior

When a CI job is successful it will write to a cache that is keyed on a hash of the file tree.
This allows it to re-run in mere seconds if changes to the future PR only affect the commit chain—e.g., squashing or editing commit messages—while leaving the tree as-is.

> This means that, when needing to edit the commit chain without changing the tree, it is better to *wait* for the jobs to complete before editing it, to benefit from this caching behavior. Otherwise what was run but not committed to the cache just gets thrown away.

### The Docs Preview

This workflow builds the documentation—currently the rustdoc docs and the book—deploys it, and adds a bot comment to the PR.
This comment contains links that allow accessing the deployed documentation.

> When making a PR with changes that affect the documentation, the preview should be checked to make sure the docs are rendered as expected.

## Release Checklist

The following steps must be followed when preparing a new release of `ariel-os`:

1. Check whether deprecated items should be removed, if any.
1. Update the version numbers of the crates that need to be bumped.

    <div class="warning">
        <ul>
            <li>The crates in <code>/src/lib/</code> are managed separately and their version numbers should <em>not</em> be bumped.</li>
            <li>The <code>ariel-os-sensors</code> crate's version is decoupled from the rest of the OS, as every sensor driver relies on it, and bumping it may result in fragmenting the entire ecosystem of sensor drivers.</li>
        </ul>
    </div>

1. Update the changelog manually, going through merge commits, especially focusing on PRs with the [`breaking`][issue-label-breaking] and [`changelog:highlight`][issue-label-changelog-highlight] labels, and skipping those with the [`changelog:skip`][issue-label-changelog-skip] label.
   If PR descriptions contain the string `BREAKING CHANGE` (in line with the [Conventional Commits][conventional-commits-spec] specification), these may be highlighted in the changelog.

   The title of the PR updating the changelog should start with `chore(release):` (so it could automatically be ignored by tools later).
1. Create a git tag in the format `v{version}`.
1. No `ariel-os*` crates are currently published on [crates.io][crates-io].

[issue-label-breaking]: https://github.com/ariel-os/ariel-os/issues?q=state%3Aopen%20label%3Abreaking
[issue-label-changelog-highlight]: https://github.com/ariel-os/ariel-os/issues?q=state%3Aopen%20label%3Achangelog%3Ahighlight
[issue-label-changelog-skip]: https://github.com/ariel-os/ariel-os/issues?q=state%3Aopen%20label%3Achangelog%3Askip
[crates-io]: https://crates.io
[conventional-commits-spec]: https://www.conventionalcommits.org/en/v1.0.0/
