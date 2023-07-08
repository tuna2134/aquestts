FROM debian:bookworm-slim AS downloaded

WORKDIR /src/downloader

RUN apt-get update && apt-get install -y jq wget
RUN export VERSION=$(curl https://api.github.com/repos/tuna2134/aquestts/releases | jq .[0].name)
RUN wget --trust-server-names https://github.com/tuna2134/aquestts/releases/download/$VERSION/bot

FROM debian:bookworm-slim

WORKDIR /src/app

COPY --from=downloader /src/downloader/bot .

CMD ["./bot"]
