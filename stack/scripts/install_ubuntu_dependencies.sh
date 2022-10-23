apt update \
&& apt install -y git \
               npm \
               jq \
               docker.io \
               docker-compose \
               gettext-base \
               libpq-dev \
               default-jdk \
               cargo;

cargo install diesel_cli --no-default-features --features postgres

echo "PATH=$HOME/.cargo/bin:"'$PATH' >> ~/.bashrc

