# GitHub Action for Container Building and Registry Management

## Objective
Create a GitHub Action and support makefile rules for use by developers to automate the building of Docker containers and manage their storage in the GitHub Container Registry.

## Main Components
1. GitHub Action for building and uploading containers
2. Makefile rule to trigger and monitor the action
3. Makefile rule to list images in the container registry
4. Makefile rule to delete a named image from the container registry

## Detailed Requirements

### 1. GitHub Action

#### Trigger
- Support manual triggering through GitHub UI (workflow_dispatch)
- Support triggering via repository_dispatch event type 'build_containers'
- Must be triggerable by makefile rule run on developer workstation

#### Authentication
- Use the built-in GITHUB_TOKEN secret for authentication in the GitHub Action
- Use a GitHub Personal Access Token (PAT) stored as an environment variable named RBM_GITHUB_PAT for authentication in makefile rules

#### Configuration
- Use a configuration file named `rbm-config.yml` in the repository root to define the following variables with their defaults:
  ```yaml
  build_architectures: x86_64
  history_dir: RBM-history
  recipes_dir: RBM-recipes
  recipe_pattern: "*.dockerfile"
  timeout-minutes: 60
  concurrency: 2
  max-parallel: 2
  continue-on-error: false
  fail-fast: false
  ```
- The `rbm-config.yml` file must be committed to the repository root. If not found, the action must fail fast.
- Apply the following configuration items in the action environment for rate limiting:
  - timeout-minutes
  - concurrency
  - max-parallel
  - continue-on-error
  - fail-fast

#### Domain
- Process all Dockerfiles found in the directory specified by `recipes_dir` in the configuration
- Use the `recipe_pattern` from the configuration to identify Dockerfile recipes

#### Build Process

a. Generate a timestamp postfix:
   - Use the command: `date +'%Y%m%d__%H%M%S'`
   - This will be used in the Build Label

b. Create a Build Label for each image:
   - Format: `<filename>.<short_commit_hash>.<timestamp>`
   - Remove file extensions specified in `recipe_pattern` from the filename
   - Include the first 7 characters of the commit hash
   - This Build Label will be used as the container image tag when uploading to the container registry
   - Check before a container build that the intended tag name is unused; if used, fail that build but not necessarily the action

c. Create a History Subdirectory:
   - Location: `<history_dir>/<Build Label>/`
   - If the history directory already exists at the beginning of the action, fail fast
   - Copy the Dockerfile into this directory
   - Store build transcript in `history.txt` within this directory
   - Create a file called `commit.txt` containing the full GitHub commit hash of the repo corresponding to the Dockerfile

d. Implement a matrix build strategy for parallel builds:
   - Use GitHub Actions matrix strategy to build multiple Dockerfiles concurrently
   - Failure of one build should not affect the attempt of another
   - No secrets are required for these builds
   - Build for the architectures specified in `build_architectures` configuration, allowing multiple architectures via a delimited list
   - Use Docker Buildx for improved build performance

e. Commit the History Subdirectory to the repository for each build attempt (regardless of build success)
   - This must be done by the GitHub Action as it orchestrates the process
   - Configure git user.email and user.name for the commit
   - There should not be merge conflicts since the GitHub Action is the exclusive creator of new History Subdirectories

f. If a build is successful:
   - Upload the image to the GitHub Container Registry using the Build Label as the image tag
   - Any attempt to use GHCR that fails needs to cause the action to fail, though it doesn't need to stop other parallel builds
   - Create a new file in the History Subdirectory called `digest.txt` containing:
     * Size of the image (in bytes)
     * Duration of the build (in seconds)

g. Additional Considerations:
   - Security scanning is expressly not to be done in this action at this time for simplicity
   - No Slack or email notifications are to be triggered by the GitHub action on completion; users are expected to use the web interface for that, for simplicity
   - There is no express process for updating the action itself; for simplicity, it is simply a repository file

### 2. Makefile Rule: Action Trigger
- Initiate the GitHub Action using the repository_dispatch event
- Block (wait) until the action completes
- Use curl or a similar tool to trigger the action and monitor its progress

### 3. Makefile Rule: List Images
- List all images currently stored in the repository's container registry
- Use the GitHub API to retrieve this information

### 4. Makefile Rule: Delete Image
- Delete a single named image from the repository's container registry
- Prompt for confirmation before deleting
- Use the GitHub API to perform the deletion

## Additional Notes

- There is no requirement to manage a persistent cache, as builds will be infrequent
- All Dockerfiles should build correctly without any secrets
- The action is not responsible for deleting old versions of images. This is handled by makefile rules provided to the developer
- Developers are responsible for all cleanups, including pruning old images and deleting their History Directories after the build
- All error handling is expected to be done via the history

