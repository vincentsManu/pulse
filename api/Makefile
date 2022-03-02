.PHONY: test test-payload release build-mac build docker-build-mac docker-build sec-scan

APP_NAME?=pulstats
PROJECT_NAME?=manupulse
DB_PASSWORD?=yourpass
DB_SUFFIX?=_local

release:
	cargo release patch --tag-prefix "" --execute 

########
# test #
########

test:
	cargo test

test-payload:
	cargo test post_user_session_test

########
# init #
########

db-init:
	sed \
	-e "s/DB_PASSWORD/$(DB_PASSWORD)/g" \
	-e "s/DB_SUFFIX/$(DB_SUFFIX)/g" \
	./db/init.sql | \
	psql -h localhost -p 5432 -U postgres -f -

migrate:
	cargo run --bin pulstats-migration

#########
# build #
#########

build: release docker-build

docker-build:
	docker build -t $(APP_NAME) ./

#############
# build mac #
#############

build-mac: test release docker-build-mac

docker-build-mac:
	cargo build --release --target x86_64-unknown-linux-musl &&\
	mv ./target/x86_64-unknown-linux-musl/release/$(APP_NAME) ./$(APP_NAME) &&\
	mv ./target/x86_64-unknown-linux-musl/release/$(APP_NAME)-migration ./$(APP_NAME)-migration &&\
	docker build -t $(APP_NAME) -f DockerfileMac ./

#######
# sec #
#######

sec-scan:
	trivy fs ./