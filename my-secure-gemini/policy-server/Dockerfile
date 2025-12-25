# --- ビルド用ステージ ---
FROM haskell:9.6 AS builder
WORKDIR /opt/policy-server
COPY . .
RUN cabal update && cabal build

# --- 実行用ステージ（軽量化） ---
FROM ubuntu:22.04
WORKDIR /opt/app
# ビルドしたバイナリだけを持ってくる
COPY --from=builder /opt/policy-server/dist-newstyle/build/*/ghc-*/policy-server-*/x/policy-server/build/policy-server/policy-server .
EXPOSE 8000
CMD ["./policy-server"]