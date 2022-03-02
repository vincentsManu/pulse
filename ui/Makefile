.PHONY: build docker-build release

APP_NAME?=pulstats-ui

#########
# build #
#########

build: release docker-build

docker-build:
	npm run build &&\
	docker build -t $(APP_NAME) ./

release:
	npm version patch