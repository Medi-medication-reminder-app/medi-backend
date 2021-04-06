FROM debian:jessie AS builder

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
RUN apt-get update && apt-get install -y curl libmysqlclient-dev build-essential

# Install rust
RUN curl https://sh.rustup.rs/ -sSf | \
  sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . ./

RUN rustup default nightly

# RUN cargo build --release
RUN ROCKET_ENV=stage cargo build

FROM debian:jessie

RUN apt-get update && apt-get install -y libmysqlclient-dev

# COPY --from=builder \
#   /target/release/medi-backend \
#   /usr/local/bin/

COPY --from=builder \
  /target/debug/medi-backend \
  /usr/local/bin/

COPY --from=builder \
  /Rocket.toml \
  /root

WORKDIR /root
# CMD ROCKET_PORT=8000 DATABASE_URL=mysql://mediadmin:changethis@127.0.0.1:3306/mediusers /usr/local/bin/medi-backend
# CMD ROCKET_PORT=8000 /usr/local/bin/medi-backend
CMD ROCKET_ENV=stage /usr/local/bin/medi-backend