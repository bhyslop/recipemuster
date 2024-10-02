SUMMARY: I want a Github Action and some support scripts that build my project's containers within github servers and post the successfully built images to the github container registry.

The action must trigger builds of all files found in repo subdirectory `RBM-recipes`: every file there is a dockerfile.

At the beginning of each attempt to build all such images, the Github Action must
derive a postfix from the invocation time on the github server with the following linux command or similar:
   ```
   date +'%Y%m%d__%H%M%S'
   ```

The Build Label for each build should be thus the filename found in `RBM-recipes` minus any `.dockerfile` or `.recipe` extension but then appended with the above datestamp.

At the beginning of each build, create a History Subdirectory.
The History Subdirectory must be in repo root directory `RBM-transcripts` and must include the Build Label.
Copy the dockerfile verbatim into the History Subdirectory at the beginning of the build.
Store the textual transcript of the build attempt in file `history.txt` under the History Subdirectory.
At the conclusion of the build, whether successful or not, the History Subdirectory must be committed to the github repository.

If a build is successful, the image must be posted to the github repo's container registry area.

One support script is a command that triggers the whole action and then blocks until the action has completed.

Another support script lists all images resident in the repo's container registry area.

Another support script causes the deletion of an image from the repo's container registry area.
