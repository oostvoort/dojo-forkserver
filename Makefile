REPO = oostvoort/dojo-forkserver

# Example: make docker-build version=v1.1.0
docker-build:
	docker build -t $(REPO):$(version) -t $(REPO):latest .

docker-run:
	docker run -p 3000:3000 -p 5050:5050 -p 8080:8080 $(REPO):$(version)

# Example: make docker-push version=v1.1.0
docker-push:
	docker push $(REPO):latest
	docker push $(REPO):$(version)
