FROM docker.io/library/ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG IMAGE_NAME

LABEL io.axia.image.authors="devops-team@axia.io" \
	io.axia.image.vendor="Axia Technologies" \
	io.axia.image.title="${IMAGE_NAME}" \
	io.axia.image.description="Axia: a platform for web3" \
	io.axia.image.source="https://github.com/axiatech/axia/blob/${VCS_REF}/scripts/docker/axia_injected_debug.Dockerfile" \
	io.axia.image.revision="${VCS_REF}" \
	io.axia.image.created="${BUILD_DATE}" \
	io.axia.image.documentation="https://github.com/axiatech/axia/"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl1.1 \
		ca-certificates && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user and link ~/.local/share/axia to /data
	useradd -m -u 1000 -U -s /bin/sh -d /axia axia && \
	mkdir -p /data /axia/.local/share && \
	chown -R axia:axia /data && \
	ln -s /data /axia/.local/share/axia

# add axia binary to docker image
COPY ./axia /usr/local/bin

USER axia

# check if executable works in this container
RUN /usr/local/bin/axia --version

EXPOSE 30333 9933 9944
VOLUME ["/axia"]

ENTRYPOINT ["/usr/local/bin/axia"]
