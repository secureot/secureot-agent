#!/bin/bash

echo "🔹 Instalando SecureOT Insight Agent..."
sleep 1

# Actualizar paquetes del sistema
echo "🔹 Actualizando paquetes..."
sudo apt update && sudo apt upgrade -y

# Instalar Rust si no está presente
if ! command -v cargo &> /dev/null
then
    echo "🔹 Instalando Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Crear estructura de directorios
echo "🔹 Configurando estructura de archivos..."
mkdir -p secureot-agent/{src,config,docs,logs,scripts}

# Clonar el repositorio si no está presente
if [ ! -d "secureot-agent" ]; then
    echo "🔹 Clonando repositorio..."
    git clone https://github.com/tu-repo/secureot-agent.git
fi

# Instalar dependencias
echo "🔹 Instalando dependencias de Rust..."
cargo install prometheus tokio axum pnet native-tls serde toml clap

# Compilar el proyecto
echo "🔹 Compilando el agente..."
cd secureot-agent
cargo build --release

# Configurar permisos para scripts
chmod +x scripts/*.sh

echo "✅ Instalación completada. Puedes ejecutar el agente con:"
echo "   ./target/release/secureot-agent --dual"
// Actualizacion de codigo 2025-06-08
// Actualizacion de codigo 2025-06-02
// Actualizacion de codigo 2025-05-30
// Actualizacion de codigo 2025-05-25
// Actualizacion de codigo 2025-05-22
// Actualizacion de codigo 2025-05-20
// Actualizacion de codigo 2025-05-18
// Actualizacion de codigo 2025-06-07
// Actualizacion de codigo 2025-06-02
