FROM golang:1.21.4-alpine AS builder

WORKDIR /build
COPY go.mod go.sum ./
RUN go mod download
COPY . ./
RUN CGO_ENABLED=0 go build -trimpath -ldflags="-s -w" -o yuki ./cmd/yuki

FROM alpine:3.13 AS compressor
WORKDIR /compress
RUN apk add --no-cache upx binutils
COPY --from=builder /build/yuki .
RUN strip yuki -o yuki-striped
# RUN upx --best --lzma yuki-striped -o yuki-compressed

FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=compressor /compress/yuki-striped /yuki
ENTRYPOINT ["/yuki"]
