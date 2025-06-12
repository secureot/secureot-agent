# Usar la imagen oficial de Rust
FROM rust:latest

# Establecer el directorio de trabajo dentro del contenedor
WORKDIR /app

# Copiar el código fuente
COPY . .

# Construir el binario
RUN cargo build --release

# Mover el ejecutable al sistema
RUN cp target/release/secureot-agent /usr/local/bin/

# Ejecutar el agente con configuración predeterminada
CMD ["secureot-agent", "--dual"]
