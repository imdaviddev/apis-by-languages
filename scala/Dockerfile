FROM ubuntu:20.04

# Establecer el directorio de trabajo
WORKDIR /app

# Configurar la zona horaria para evitar la intervención durante la instalación
RUN echo "Etc/UTC" > /etc/timezone && \
    apt-get update && apt-get upgrade -y && \
    apt-get install -y \
    git \
    fpc \
    curl \
    build-essential \
    tzdata && \
    apt-get clean

# Clonar el repositorio de Fano CLI
RUN git clone https://github.com/fanoframework/fano-cli.git

# Configurar Fano CLI
WORKDIR /app/fano-cli
RUN ./tools/config.setup.sh

# Construir Fano CLI
RUN ./build.sh

# Copiar el binario de Fano CLI a una ubicación accesible globalmente
RUN cp bin/out/fanocli /usr/local/bin/fanocli

# Crear el directorio 'app' dentro del contenedor
RUN mkdir -p /app

# Mantener el contenedor ejecutándose en modo interactivo
CMD ["bash"]