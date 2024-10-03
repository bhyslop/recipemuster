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
- Use a GitHub Personal Access Token (PAT) stored as an environment variable (e.g., GITHUB_PAT) for authentication in makefile rules

#### Domain
- Process all Dockerfiles found in the `RBM-recipes` subdirectory of the repository
- Support both `.dockerfile` and `.recipe` file extensions

#### Build Process
a. Generate a timestamp postfix:
   - Use the command: `date +'%Y%m%d__%H%M%S'`
   - This will be used in the Build Label

b. Create a Build Label for each image:
   - Format: `<filename>.<short_commit_hash>.<timestamp>`
   - Remove `.dockerfile` or `.recipe` extensions from the filename
   - Include the first 7 characters of the commit hash
   - This Build Label will be used as the container image tag when uploading to the container registry

c. Create a History Subdirectory:
   - Location: `RBM-transcripts/<Build Label>/`
   - Copy the Dockerfile into this directory
   - Store build transcript in `history.txt` within this directory
   - Create a file called `commit.txt` containing the full GitHub commit hash of the repo corresponding to the Dockerfile

d. Attempt to build all Docker images:
   - Failure of one build should not affect the attempt of another
   - No secrets are required for these builds
   - Build only for the architecture of the GitHub Action runner (typically x86_64)
   - Use Docker Buildx for improved build performance

e. Commit the History Subdirectory to the repository for each build attempt (regardless of build success)
   - This must be done by the GitHub Action as it orchestrates the process
   - Configure git user.email and user.name for the commit
   - There should not be merge conflicts since the GitHub Action is the exclusive creator of new History Subdirectories

f. If a build is successful:
   - Upload the image to the GitHub Container Registry using the Build Label as the image tag
   - Create a new file in the History Subdirectory called `digest.txt` containing:
     * Size of the image (in bytes)
     * Duration of the build (in seconds)

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
- The makefile containing the support rules should be delivered separately, not via the GitHub repository
- Use Docker Buildx for improved build performance and multi-platform support if needed in the future
- Consider implementing a mechanism to prevent concurrent builds of the same Dockerfile to avoid conflicts
- Add error handling and logging to the GitHub Action to provide better feedback on build failures
- Consider implementing a retention policy for old images and build histories to manage storage usage
