# OSS-Fuzz build container for crabvpx.
# Copied into the OSS-Fuzz repo as projects/crabvpx/Dockerfile.
FROM gcr.io/oss-fuzz-base/base-builder-rust
RUN git clone --depth 1 https://github.com/ubercodec/crabvpx $SRC/crabvpx
WORKDIR $SRC/crabvpx
COPY build.sh $SRC/
