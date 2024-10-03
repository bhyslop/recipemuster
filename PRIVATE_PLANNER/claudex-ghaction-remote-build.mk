# Makefile rules for container management

# Ensure GITHUB_PAT is set in your environment variables
ifndef GITHUB_PAT
$(error GITHUB_PAT is not set. Please set it in your environment variables)
endif

REPO_OWNER := $(shell git config --get remote.origin.url | sed -n 's/.*github.com[:\/]\(.*\)\/\(.*\)\.git/\1/p')
REPO_NAME := $(shell git config --get remote.origin.url | sed -n 's/.*github.com[:\/]\(.*\)\/\(.*\)\.git/\2/p')

.PHONY: trigger-build list-images delete-image

trigger-build:
	@echo "Triggering GitHub Action to build containers..."
	@curl -X POST \
		-H "Authorization: token $(GITHUB_PAT)" \
		-H "Accept: application/vnd.github.v3+json" \
		https://api.github.com/repos/$(REPO_OWNER)/$(REPO_NAME)/dispatches \
		-d '{"event_type": "build-containers"}'
	@echo "Build triggered. Waiting for completion..."
	@while true; do \
		status=$$(curl -s -H "Authorization: token $(GITHUB_PAT)" \
			-H "Accept: application/vnd.github.v3+json" \
			https://api.github.com/repos/$(REPO_OWNER)/$(REPO_NAME)/actions/runs \
			| jq -r '.workflow_runs[0].status'); \
		if [ "$$status" = "completed" ]; then \
			echo "Build completed."; \
			break; \
		elif [ "$$status" = "null" ]; then \
			echo "No active workflow found. Please check manually."; \
			break; \
		else \
			echo "Build status: $$status"; \
			sleep 30; \
		fi \
	done

list-images:
	@echo "Listing images in the container registry..."
	@curl -s -H "Authorization: token $(GITHUB_PAT)" \
		-H "Accept: application/vnd.github.v3+json" \
		https://api.github.com/user/packages?package_type=container \
		| jq -r '.[] | select(.repository.full_name=="$(REPO_OWNER)/$(REPO_NAME)") | .name'

delete-image:
	@echo "Enter the name of the image to delete:"
	@read -p "Image name: " image_name; \
	echo "Are you sure you want to delete $$image_name? (y/N)"; \
	read -p "Confirm: " confirm; \
	if [ "$$confirm" = "y" ] || [ "$$confirm" = "Y" ]; then \
		package_id=$$(curl -s -H "Authorization: token $(GITHUB_PAT)" \
			-H "Accept: application/vnd.github.v3+json" \
			https://api.github.com/user/packages?package_type=container \
			| jq -r '.[] | select(.name=="'$$image_name'") | .id'); \
		if [ -n "$$package_id" ]; then \
			curl -X DELETE -H "Authorization: token $(GITHUB_PAT)" \
				-H "Accept: application/vnd.github.v3+json" \
				https://api.github.com/user/packages/container/$$image_name; \
			echo "Image $$image_name deleted."; \
		else \
			echo "Image $$image_name not found."; \
		fi \
	else \
		echo "Deletion cancelled."; \
	fi
