FROM docker.io/library/ubuntu:20.04

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG AXIA_VERSION
ARG AXIA_GPGKEY=9D4B2B6EB8F97156D19669A9FF0812D491B96798
ARG GPG_KEYSERVER="hkps://keys.mailvelope.com"

LABEL io.axia.image.authors="devops-team@axia.io" \
	io.axia.image.vendor="Axia Technologies" \
	io.axia.image.title="axia/axia" \
	io.axia.image.description="Axia: a platform for web3. This is the official Axia image with an injected binary." \
	io.axia.image.source="https://github.com/axiatech/axia/blob/${VCS_REF}/scripts/dockerfiles/axia_injected_release.Dockerfile" \
	io.axia.image.revision="${VCS_REF}" \
	io.axia.image.created="${BUILD_DATE}" \
	io.axia.image.documentation="https://github.com/axiatech/axia/"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
		libssl1.1 \
		ca-certificates \
		gnupg && \
	useradd -m -u 1000 -U -s /bin/sh -d /axia axia && \
# add repo's gpg keys and install the published axia binary
	gpg --recv-keys --keyserver ${GPG_KEYSERVER} ${AXIA_GPGKEY} && \
	gpg --export ${AXIA_GPGKEY} > /usr/share/keyrings/axia.gpg && \
	echo 'deb [signed-by=/usr/share/keyrings/axia.gpg] https://releases.axia.io/deb release main' > /etc/apt/sources.list.d/axia.list && \
	apt-get update && \
	apt-get install -y --no-install-recommends axia=${AXIA_VERSION#?} && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/* ; \
	mkdir -p /data /axia/.local/share && \
	chown -R axia:axia /data && \
	ln -s /data /axia/.local/share/axia

USER axia

# check if executable works in this container
RUN /usr/bin/axia --version

EXPOSE 30333 9933 9944
VOLUME ["/axia"]

ENTRYPOINT ["/usr/bin/axia"]
