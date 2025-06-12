#!/bin/bash

LOG_DIR="secureot-agent/logs"
MAX_LOGS=10  # Número máximo de archivos de logs antes de eliminar los más antiguos

# Crear carpeta de logs si no existe
mkdir -p "$LOG_DIR"

# Nombre del archivo con fecha y hora
LOG_FILE="$LOG_DIR/secureot-agent-$(date +"%Y-%m-%d_%H-%M-%S").log"

# Mover logs activos al archivo de historial
mv "$LOG_DIR/secureot-agent.log" "$LOG_FILE"

# Crear un nuevo log vacío
touch "$LOG_DIR/secureot-agent.log"

# Eliminar logs antiguos si superan el límite
LOG_COUNT=$(ls -1 "$LOG_DIR" | wc -l)
if [ "$LOG_COUNT" -gt "$MAX_LOGS" ]; then
    ls -t "$LOG_DIR" | tail -n +$((MAX_LOGS+1)) | xargs rm -f
fi

echo "✅ Rotación de logs completada. Nuevo archivo: $LOG_FILE"
