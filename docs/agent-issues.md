# Managed agent issue workflow

Create a bounded issue using the **Agent-ready change** template. Include evidence, acceptance criteria, and a validation plan. A maintainer applies `agent-candidate` when the issue is ready; creating the issue alone does not start the agent. Applying the label starts one DigitalOcean Harness Runtime session, which opens a regular PR against `main` and then stops. Only a maintainer merges it. Reapplying the label starts a new session, so first check whether a PR already exists.

The workflow is `.github/workflows/agent-issue-trigger.yml`. It runs only when `HARNESS_AUTOMATION_ENABLED=true` is set as a repository Actions variable. It needs these repository Actions secrets:

- `DIGITALOCEAN_ACCESS_TOKEN`: a token allowed to create and remove Harness Runtime sessions.
- `HARNESS_INFERENCE_API_KEY`: the DigitalOcean Inference model key.
- `HARNESS_GITHUB_TOKEN`: a GitHub credential restricted to `opensensor/lightNVR`, with issue read and contents/pull request write access. Use a GitHub App installation token or a fine-grained personal access token. Do not use an account-wide CLI token.

The checked-in manifest uses Kimi K3. The runner resolves secrets and removes the session when it finishes. DigitalOcean may retain the environment config created for each session; periodically delete unused configs in Harness Runtime.

Pull requests run the integration tests, sanitizer build, static analysis, and documentation link checks. The release workflows are triggered only by tags or manual dispatch. There is no merge automation for this repository.
