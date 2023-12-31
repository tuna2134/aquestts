FROM debian:bookworm-slim AS downloader

WORKDIR /src/downloader

RUN apt-get update && apt-get install -y jq curl
RUN curl https://api.github.com/repos/tuna2134/aquestts/releases | jq .[0].name
RUN export VERSION=$(curl https://api.github.com/repos/tuna2134/aquestts/releases | jq -r .[0].name) && \
  echo $VERSION && \
  curl -L -o bot https://github.com/tuna2134/aquestts/releases/download/${VERSION}/bot

FROM debian:bookworm-slim

WORKDIR /src/app

COPY --from=downloader /src/downloader/bot .

CMD ["./bot"]
