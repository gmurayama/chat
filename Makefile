.PHONY: docker-build/multi-arch docker-build/arm64 docker-build/amd64

docker-build:
	docker buildx build --load --platform $(DOCKER_PLATFORM) -t $(DOCKER_TAG) -f Dockerfile .

docker-build/multi-arch:
	make docker-build DOCKER_PLATFORM="linux/arm64,linux/amd64"

docker-build/arm64:
	make docker-build DOCKER_PLATFORM="linux/arm64"

docker-build/amd64:
	make docker-build DOCKER_PLATFORM="linux/amd64"
