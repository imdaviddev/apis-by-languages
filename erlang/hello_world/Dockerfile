FROM debian:latest

# Instalar dependencias
RUN apt-get update && apt-get install -y \
    git \
    erlang \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Instalar rebar3
RUN curl -fsSL https://s3.amazonaws.com/rebar3/rebar3 -o /usr/local/bin/rebar3 && \
    chmod +x /usr/local/bin/rebar3

# Configurar el proyecto
WORKDIR /app
RUN rebar3 new app hello_world
WORKDIR /app/hello_world

# Configurar dependencias en rebar.config
RUN echo '{deps, [ {rooster, ".*", {git, "git://github.com/fbeline/rooster.git", {branch, "master"}}} ]}.' >> rebar.config

# Compilar y descargar dependencias
RUN rebar3 compile

# Copiar el código fuente
COPY src/server.erl src/server.erl

# Exponer el puerto 8080
EXPOSE 8080

# Comando para iniciar el servidor
CMD erl -pa _build/default/lib/*/ebin -boot start_sasl -s server -s reloader
