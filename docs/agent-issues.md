# Managed agent issue workflow

Create a bounded issue using the **Agent-ready change** template. Include evidence, acceptance criteria, and a validation plan. A maintainer applies `agent-candidate` when the issue is ready; creating the issue alone does not start the agent. Applying the label launches one DigitalOcean Harness Runtime session and the runner exits after launch. The agent pushes its branch to the dedicated `matteius/lightNVR` fork with a fork-scoped deploy key. Actions are disabled on the fork. A signed GitHub push webhook opens a regular PR against `opensensor/lightNVR` main. Only a maintainer merges it. Reapplying the label starts a new session, so first check whether a PR already exists.

The workflow is `.github/workflows/agent-issue-trigger.yml`. It runs only when `HARNESS_AUTOMATION_ENABLED=true` is set as a repository Actions variable. It needs these repository Actions secrets:

- `DIGITALOCEAN_ACCESS_TOKEN`: a token allowed to create Harness Runtime sessions.
- `HARNESS_INFERENCE_API_KEY`: the DigitalOcean Inference model key.
- `HARNESS_DEPLOY_KEY_B64`: the base64-encoded SSH deploy key scoped to `matteius/lightNVR`, used by the agent to push its branch. The shared PR broker stores its separate PR credential outside this repository and the agent. The public agent does not receive a GitHub API token.

The manifest approves agent tools by default, so the session needs no attached runner to answer approval prompts. The private storefront repository runs a daily housekeeping job that removes issue sessions after 24 hours without an event and deletes their saved configs. Do not remove and reapply the label on issue #647 for testing; that would launch a second session.

The checked-in manifest uses Kimi K3. The runner resolves secrets and exits after DigitalOcean accepts the prompt. A shared DigitalOcean Function opens the PR after the agent pushes.

Pull requests run the integration tests, sanitizer build, static analysis, and documentation link checks. The release workflows are triggered only by tags or manual dispatch. There is no merge automation for this repository.
