# GitHub Action for Container Building and Registry Management

## Objective
Create a GitHub V3 Action and supporting makefile rules to build Docker containers and manage them in the GitHub Container Registry.

## Components
1. GitHub Action for building and uploading containers
2. Makefile rules for:
   - Triggering and monitoring the action
   - Querying build status
   - Listing registry images
   - Deleting specific registry images

## Detailed Requirements

### 1. GitHub Action

#### Trigger
- Manual trigger via GitHub UI (workflow_dispatch)
- Repository_dispatch event type 'build_containers'
- Triggerable by makefile rule from developer's workstation

#### Authentication
- GitHub Action: Use GITHUB_TOKEN secret
- Makefile rules: Use GitHub Personal Access Token (RBM_GITHUB_PAT)

#### Configuration
- Use `rbm-config.yml` in repository root with these variables:
  ```yaml
  build_architectures: x86_64
  history_dir:         RBM-history
  recipes_dir:         RBM-recipes
  recipe_pattern:      "*.dockerfile"
  timeout-minutes:     60
  concurrency:         2
  max-parallel:        2
  continue-on-error:   false
  fail-fast:           false
  ```
- Action must fail if `rbm-config.yml` is missing or improperly formatted
- Implement `timeout-minutes`, `concurrency`, `max-parallel`, `continue-on-error`, and `fail-fast` using native GitHub Actions workflow syntax

#### Build Process

1. Generate timestamp: `date +'%Y%m%d__%H%M%S'`

2. Create Build Label: `<filename>.<short_commit_hash>.<timestamp>`
   - Remove file extensions from filename
   - Use first 7 characters of commit hash
   - Use as container image tag
   - If tag already exists, fail the specific build but continue with others

3. Create History Subdirectory: `<history_dir>/<Build Label>/`
   - Verify directory doesn't exist at action start; fail if it does
   - Copy Dockerfile to this directory
   - Store complete build transcript in `history.txt`
   - Create `commit.txt` with full GitHub commit hash

4. Matrix build strategy:
   - Build Dockerfiles concurrently based on `max-parallel` setting
   - Continue other builds if one fails, unless `fail-fast` is true
   - Use `build_architectures` configuration for multi-architecture builds
   - Utilize Docker Buildx for builds

5. Commit History Subdirectory:
   - Performed by GitHub Action
   - Configure git user.email as "github-actions[bot]@users.noreply.github.com"
   - Configure git user.name as "github-actions[bot]"

6. Successful build:
   - Upload to GitHub Container Registry with Build Label tag
   - Fail action if GHCR upload fails after 3 retries
   - Create `digest.txt` with image size (in MB) and build duration (in seconds)

7. Versioning: Use only Build Label as image tag

### 2. Makefile Rule: Trigger Builds
- Initiate action using repository_dispatch event
- Monitor progress by polling GitHub API every 30 seconds
- Store query URL in `../LAST_GET_WORKFLOW_RUN.txt`
- Display real-time status updates to the user

### 3. Makefile Rule: Query Builds
- Use URL from `../LAST_GET_WORKFLOW_RUN.txt`
- Return exit code 0 if build finished (success or failure), non-zero if ongoing
- Display build status and any error messages

### 4. Makefile Rule: List Images
- List all repository's container registry images
- Use GitHub API
- Display image name, tag, size, and creation date in a tabular format

### 5. Makefile Rule: Delete Image
- Delete specified image from repository's container registry
- Prompt for confirmation, showing image details before deletion
- Use GitHub API
- Provide feedback on successful deletion or any errors encountered

## Notes
- No persistent cache management is implemented in this version
- Dockerfiles are built without secrets
- Developers are responsible for managing cleanups (old images, History Directories)
- Error handling is primarily managed through the history directory and build transcripts
