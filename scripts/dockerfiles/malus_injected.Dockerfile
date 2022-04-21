FROM debian:bullseye-slim

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG IMAGE_NAME

LABEL io.axia.image.authors="devops-team@axia.io" \
	io.axia.image.vendor="Axia Technologies" \
	io.axia.image.title="${IMAGE_NAME}" \
	io.axia.image.description="Malus - the nemesis of axia" \
	io.axia.image.source="https://github.com/axiatech/axia/blob/${VCS_REF}/scripts/dockerfiles/malus.Dockerfile" \
	io.axia.image.revision="${VCS_REF}" \
	io.axia.image.created="${BUILD_DATE}" \
	io.axia.image.documentation="https://github.com/axiatech/axia/"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
    ca-certificates \
    curl \
    libssl1.1 \
    tini && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user
  groupadd --gid 10000 nonroot && \
  useradd  --home-dir /home/nonroot \
           --create-home \
           --shell /bin/bash \
           --gid nonroot \
           --groups nonroot \
           --uid 10000 nonroot


# add adder-collator binary to docker image
COPY ./malus /usr/local/bin

USER nonroot

# check if executable works in this container
RUN /usr/local/bin/malus --version

# Tini allows us to avoid several Docker edge cases, see https://github.com/krallin/tini.
ENTRYPOINT ["tini", "--", "/bin/bash"]
