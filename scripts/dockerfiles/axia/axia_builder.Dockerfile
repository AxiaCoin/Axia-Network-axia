# This is the build stage for Axia. Here we create the binary in a temporary image.
FROM docker.io/axiatech/ci-linux:production as builder

WORKDIR /axia
COPY . /axia

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Axia binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for Axia: a platform for web3" \
	io.axia.image.type="builder" \
	io.axia.image.authors="chevdor@gmail.com, devops-team@axia.io" \
	io.axia.image.vendor="Axia Technologies" \
	io.axia.image.description="Axia: a platform for web3" \
	io.axia.image.source="https://github.com/axiatech/axia/blob/${VCS_REF}/scripts/dockerfiles/axia/axia_builder.Dockerfile" \
	io.axia.image.documentation="https://github.com/axiatech/axia/"

COPY --from=builder /axia/target/release/axia /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /axia axia && \
	mkdir -p /data /axia/.local/share && \
	chown -R axia:axia /data && \
	ln -s /data /axia/.local/share/axia && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/axia --version

USER axia

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/axia"]
