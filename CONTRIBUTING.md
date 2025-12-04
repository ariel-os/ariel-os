# Contributing to Ariel OS

Welcome and thanks for your interest in contributing to Ariel OS! We appreciate all ways of contributing to the project, code and non-code.


## Core information

- We use the GitHub pull request workflow
    - PRs can be submitted as work-in-progress by marking them as draft
    - Please indicate a ready-for-review PR with the button in GitHub
    - We use [Conventional Commits](https://www.conventionalcommits.org/en)
    - We use [DCO](#Developer-Certificate-of-Origin) to sign off commits.
- Please check our [Coding Conventions](https://ariel-os.github.io/ariel-os/dev/docs/book/coding-conventions.html)
- If you want to add support for a new board or a new chip, please check [the Developer Guide][adding-board-support]
- Ariel OS is dual licensed under the [Apache-2.0](./LICENSE-APACHE) and [MIT](./LICENSE-MIT) licenses

## Developer Certificate of Origin

To make a good faith effort to ensure licensing criteria are met,
Ariel OS requires the Developer Certificate of Origin (DCO) process
to be followed.

The Developer Certificate of Origin (DCO) is a lightweight way for contributors
to certify that they wrote or otherwise have the right to submit the code
they are contributing to the project.
Here is the full [text of the DCO], reformatted for readability:

>Developer's Certificate of Origin 1.1
>
>By making a contribution to this project, I certify that:
>
> 1. The contribution was created in whole or in part by me and I
>    have the right to submit it under the open source license
>    indicated in the file; or
>
> 2. The contribution is based upon previous work that, to the best
>    of my knowledge, is covered under an appropriate open source
>    license and I have the right under that license to submit that
>    work with modifications, whether created in whole or in part
>    by me, under the same open source license (unless I am
>    permitted to submit under a different license), as indicated
>    in the file; or
>
> 3. The contribution was provided directly to me by some other
>    person who certified (1), (2) or (3) and I have not modified
>    it.
>
> 4. I understand and agree that this project and the contribution
>    are public and that a record of the contribution (including all
>    personal information I submit with it, including my sign-off) is
>    maintained indefinitely and may be redistributed consistent with
>    this project or the open source license(s) involved.

If you can certify the above,
you just add a line at the bottom of your commit messages saying:

    Signed-off-by: Random J Developer <random@developer.example.org>

using a known identity. This will be done for you automatically
if you use `git commit -s`.
The email address used in your Sign-off
must match the email address used for by commit author.

To permanently add the 'Sign-off' to your commits contributed to Ariel OS,
it is easiest to use a commit template.
Create a simple text file, e.g. `.git/commitmessage` with the following content:

    Signed-off-by: Random J Developer <random@developer.example.org>

and enable it in your git config:

   git config --local commit.template .git/commitmessage

### Altering existing commits

When altering an existing commit from (a) different contributor(s),
you must add your own 'Sign-off' line, without removing the existing ones.

## Roadmap

We use a [roadmap](https://github.com/ariel-os/ariel-os/issues/242) to track issues. If you are looking for an issue to contribute, please check our roadmap and our [current list of issues](https://github.com/ariel-os/ariel-os/issues).


## Ask for help

We are happy to hear from you and help you. The best way to reach us is to ask on our [Matrix chat room](https://matrix.to/#/#ariel-os:matrix.org).

[text of the DCO]: https://developercertificate.org/
[adding-board-support]: https://ariel-os.github.io/ariel-os/dev/docs/book/adding-board-support.html
