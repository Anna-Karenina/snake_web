FROM rust:1.66.0


RUN curl -sL https://deb.nodesource.com/setup_18.x | bash - && \
  apt-get install -y nodejs && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

WORKDIR /app
ADD src ./src
ADD www ./www
COPY Cargo.toml Cargo.lock ./

RUN mkdir -p /pkg
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack build --target web 

WORKDIR /app/www/
RUN npm install
EXPOSE 8080
RUN npm run dev


