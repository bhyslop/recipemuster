# GitHub Action for Container Building and Registry Management

## Objective
Create a GitHub Action and support bash scripts to automate the building of Docker containers and manage their storage in the GitHub Container Registry.

## Main Components
1. GitHub Action for building and uploading containers
2. Support script to trigger and monitor the action
3. Support script to list images in the container registry

## Detailed Requirements

### 1. GitHub Action

#### Trigger
- Support script run on developer workstation, not every repo push.

#### Domain
- All Dockerfiles found in the `RBM-recipes` subdirectory of the repository.

#### Build Process
a. Generate a timestamp postfix:
   - Use the command: `date +'%Y%m%d__%H%M%S'`
   - This will be used in the Build Label

b. Create a Build Label for each image:
   - Format: `<filename>.<timestamp>`
   - Remove `.dockerfile` or `.recipe` extensions from the filename

c. Create a History Subdirectory:
   - Location: `RBM-transcripts/<Build Label>/`
   - Copy the Dockerfile into this directory
   - Store build transcript in `history.txt` within this directory
   - Create a file called `commit.txt` containing the GitHub commit hash of the repo corresponding to the Dockerfile

d. Attempt to build all Docker images:
   - Failure of one build should not affect the attempt of another
   - No secrets are required for these builds
   - Build only for PC architecture

e. Commit the History Subdirectory to the repository for each build (regardless of build success)

f. If a build is successful, upload the image to the GitHub Container Registry

### 2. Support Script: Action Trigger
- Initiate the GitHub Action
- Block (wait) until the action completes

### 3. Support Script: List Images
- List all images currently stored in the repository's container registry

## Additional Notes
- There is no requirement to manage a persistent cache, as builds will be infrequent.
- All Dockerfiles should build correctly without any secrets.
- The action is not responsible for deleting old versions of images. This is handled by support scripts provided to the developer.
- Developers are responsible for all cleanups, including pruning old images and deleting their History Directories after the build.