FROM axiaio/runtime-linux-production:bullseye

# Install required dependencies
RUN apt-get update && apt-get install ca-certificates  -y

COPY axia /opt/axia/bin/axia
COPY specs /opt/specs

WORKDIR /opt/axia
CMD /opt/axia/bin/axia $ARGS
