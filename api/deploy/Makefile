.PHONY: deploy-latest context-select image-tag image-push manifest-kustomize-build manifests-apply release-commit

ENV?=dev

include env/base/base.env
include env/overlays/$(ENV)/$(ENV).env

export

IMAGE_NAME = $(CONTAINER_REGISTRY_URL)$(APP_NAME):$(APP_VERSION)
APP_TLS_SECRET = $(shell echo $(APP_DOMAIN) | tr "." "-")
AGE_RECIPIENTS = age1wed35yq43tg4kvwkjcc4f5nhwtjzz7ql37jh6n3jq4ep3ym7p95qfp56sq,age1us8jte2zt3n9gyvtv7fjh7zvyy008n4ne6cx3adsyl0t8dkym9es9ldryk

deploy-latest: context-select image-tag image-push manifest-kustomize-build encrypted-manifests-apply manifest-apply release-commit

###########
# HELPERS #
###########

release-commit:
	git commit -m "manifest build and deploy $(IMAGE_NAME) to $(ENV)" ./manifests/*

context-select:
	kubectl config use-context $(K8S_CONTEXT)

acr-login:
	docker login $(CONTAINER_REGISTRY_URL) \
		--username 00000000-0000-0000-0000-000000000000 \
		--password $(shell az acr login --name $(CONTAINER_REGISTRY_URL) --expose-token --output tsv --query accessToken)

############
# MANIFEST #
############

MANIFEST_FOLDER = ./manifests/$(ENV)
MANIFEST_FILE_PATH = $(MANIFEST_FOLDER)/$(PROJECT_NAME)_$(APP_NAME).yaml

manifest-apply:
	kubectl apply -f $(MANIFEST_FILE_PATH)

encrypted-manifests-apply:
	@if [ -f *.enc.yaml ]; then \
		SOPS_AGE_KEY_FILE="$(HOME)/Library/Application Support/age/keys.txt" sops --decrypt $(MANIFEST_FOLDER)/*.enc.yaml | kubectl apply -f -; \
	else \
		echo "no encrypted file to apply"; \
	fi

manifest-kustomize-build:
	kubectl kustomize ./env/overlays/$(ENV) | \
	sed \
	-e "s/APP_VERSION/$(APP_VERSION)/g" \
	-e "s/APP_DOMAIN/$(APP_DOMAIN)/g" \
	-e "s/APP_TLS_SECRET/$(APP_TLS_SECRET)/g" \
	-e "s/APP_NAME/$(APP_NAME)/g" \
	-e "s/PROJECT_NAME/$(PROJECT_NAME)/g" \
	-e "s/CONTAINER_REGISTRY_URL/$(CONTAINER_REGISTRY_URL)/g" \
	-e "s/VAR_APP_ENV/$(ENV)/g" \
	> $(MANIFEST_FILE_PATH)

#########
# IMAGE #
#########

image-tag:
	docker tag $(APP_NAME) $(IMAGE_NAME)

image-push:
	@if [ -z "$(CONTAINER_REGISTRY_URL)" ]; then \
		echo "no container registry to push to as URL is empty"; \
	else \
		docker push $(IMAGE_NAME); \
	fi


###########
# SECRETS #
###########

create-basic-auth: create-basic-auth-from-files encrypt

create-basic-auth-from-files:
	sed \
		-e "s/BASIC_AUTH_MANU/$(shell cat ./env/overlays/prod/secrets/basic-auth-manu | base64)/g" \
		-e "s/BASIC_AUTH_PULSE/$(shell cat ./env/overlays/prod/secrets/basic-auth-pulse | base64)/g" \
		-e "s/PROJECT_NAME/$(PROJECT_NAME)/g" \
	./env/overlays/prod/secrets/basic-auth-tmpl.yaml \
	> ./env/overlays/prod/secrets/basic-auth.yaml

encrypt:
	sops --encrypt --encrypted-regex '^(data|stringData)$$' \
		--age $(AGE_RECIPIENTS) ./env/overlays/prod/secrets/basic-auth.yaml \
		> ./manifests/prod/basic-auth.enc.yaml