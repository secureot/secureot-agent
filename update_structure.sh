#!/bin/bash
#
# Detecta cambios automáticamente y actualiza el README. 
# Genera formato de tabla Markdown para fácil visualización.
# Puede ejecutarse antes de cada commit (git hook) o manualmente.

# chmod +x update_structure.sh
# ./update_structure.sh
# Integrarlo con git hooks para actualizar antes de cada commit:
# mkdir -p .git/hooks
# echo "./update_structure.sh" > .git/hooks/pre-commit
#chmod +x .git/hooks/pre-commit

#!/bin/bash

# Directorio base del proyecto
BASE_DIR="secureot-agent"
OUTPUT_FILE="$BASE_DIR/docs/README.md"
DESC_FILE="$BASE_DIR/config/descriptions.toml"

# Encabezado del README
echo -e "# SecureOT Insight Agent\n\n## 📂 Estructura del Proyecto\n" > "$OUTPUT_FILE"

# Función para recorrer archivos y agregar descripciones
generate_structure() {
    echo '```' >> "$OUTPUT_FILE"
    
    # Leer archivo de descripciones
    declare -A descriptions
    while IFS="=" read -r key value; do
        key=$(echo $key | tr -d '[]"')  # Limpiar clave
        value=$(echo $value | tr -d '""') # Limpiar descripción
        descriptions["$key"]="$value"
    done < <(grep "=" "$DESC_FILE")

    # Generar estructura
    find "$BASE_DIR" -type d -print | sed 's|[^/]*/| ├── |g' >> "$OUTPUT_FILE"
    find "$BASE_DIR" -type f -print | while read -r file; do
        short_name=$(basename "$file")
        desc=${descriptions["$short_name"]}
        if [ -n "$desc" ]; then
            echo " │   ├── $short_name - $desc" >> "$OUTPUT_FILE"
        else
            echo " │   ├── $short_name" >> "$OUTPUT_FILE"
        fi
    done

    echo '```' >> "$OUTPUT_FILE"
}

# Ejecutar la función
generate_structure

echo "✅ Tabla de estructura con descripciones actualizada en $OUTPUT_FILE"
