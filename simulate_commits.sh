#!/bin/bash

# Configurar nombre del usuario y correo (si es necesario)
git config user.name "cayu"
git config user.email "cayu@cayu.com.ar"

# Número de días que queremos simular actividad
DAYS=30

# Archivos de proyecto donde simularemos cambios
FILES=("src/main.rs" "src/api.rs" "src/config.rs" "README.md" "scripts/install.sh" "tests/capture_test.rs")

for i in $(seq 1 $DAYS); do
    # Seleccionar un archivo aleatorio
    FILE=${FILES[$RANDOM % ${#FILES[@]}]}

    # Simular una modificación en el archivo
    echo "// Actualizacion de codigo $(date -d "-$i days" +"%Y-%m-%d")" >> $FILE
    git add $FILE

    # Generar fecha falsa para el commit
    DATE="$(date -d "-$i days" +"%Y-%m-%d %H:%M:%S")"

    GIT_AUTHOR_DATE="$DATE" GIT_COMMITTER_DATE="$DATE" git commit -m "Update on $DATE"
done

echo "✅ Simulación completada. Puedes revisar los commits con:"
echo "   git log --pretty=format:'%h %ad | %s' --date=short"
