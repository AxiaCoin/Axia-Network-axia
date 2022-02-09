# this file copies from scripts/docker/Dockerfile and changes only the binary name
FROM docker.io/library/ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG IMAGE_NAME

LABEL io.axia.image.authors="devops-team@axiacoin.network" \
	io.axia.image.vendor="AXIA Technologies" \
	io.axia.image.title="${IMAGE_NAME}" \
	io.axia.image.description="Injected adder-collator Docker image" \
	io.axia.image.source="https://github.com/axia/axia/blob/${VCS_REF}/scripts/docker/collator_injected.Dockerfile" \
	io.axia.image.revision="${VCS_REF}" \
	io.axia.image.created="${BUILD_DATE}" \
	io.axia.image.documentation="https://github.com/axia/axia/"

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
# add user and link ~/.local/share/adder-collator to /data
	useradd -m -u 1000 -U -s /bin/sh -d /adder-collator adder-collator && \
	mkdir -p /data /adder-collator/.local/share && \
	chown -R adder-collator:adder-collator /data && \
	ln -s /data /adder-collator/.local/share/axia

# add adder-collator binary to docker image
COPY ./adder-collator /usr/local/bin

USER adder-collator

# check if executable works in this container
RUN /usr/local/bin/adder-collator --version

EXPOSE 30333 9933 9944
VOLUME ["/adder-collator"]

ENTRYPOINT ["/usr/local/bin/adder-collator"]
