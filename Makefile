BUILD := debug
APP_NAME := lambda-slack-notifier
JOBS := 4

build-docker-image:
	docker build -t aws-lambda-rust .
build:
	if [ ${BUILD} == "release" ]; then \
		docker run -v ${PWD}:/workspace -v /tmp/cargo-registry:/opt/cargo/registry aws-lambda-rust cargo build --jobs ${JOBS} --release; \
	else \
		docker run -v ${PWD}:/workspace -v /tmp/cargo-registry:/opt/cargo/registry aws-lambda-rust cargo build --jobs ${JOBS}; \
	fi
check-fmt:
	docker run -v ${PWD}:/workspace -v /tmp/cargo-registry:/opt/cargo/registry aws-lambda-rust cargo fmt --all -- --check
zip:
	cd target/${BUILD}/ && zip ../../package/${APP_NAME}.zip ${APP_NAME}
run:
	sam local generate-event sns notification | sam local invoke -t template.example.json SlackNotifier
clean:
	rm -rf target/ package/${APP_NAME}.zip
