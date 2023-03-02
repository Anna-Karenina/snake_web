FROM rust:1.66.0


RUN curl -sL https://deb.nodesource.com/setup_18.x | bash - && \
  apt-get install -y nodejs && \
  apt-get clean && \
  rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN apt-get -y update && apt-get -y install nginx

COPY ./nginx.conf /etc/nginx/conf.d/default.conf

CMD ["/usr/sbin/nginx", "-g", "daemon off;"]

WORKDIR /app
ADD src ./src
ADD web_app ./web_app
COPY Cargo.toml Cargo.lock ./

RUN mkdir -p /pkg
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN wasm-pack build --target web 

WORKDIR /app/web_app/
RUN npm install
EXPOSE 80
RUN npm run build 



