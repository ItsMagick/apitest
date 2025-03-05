FROM ubuntu:latest
LABEL authors="chaorn"

ENTRYPOINT ["top", "-b"]