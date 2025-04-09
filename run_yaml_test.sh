#!/bin/bash

# Archivo YAML a procesar
YAML_FILE=$1

# Extraer únicamente las líneas de los bloque `code: |`
BLOCKS=$(awk '
    /code: \|/ {flag=1; next}                                   # Activar flag al encontrar `code: |` y limpiar el bloque
    /stack_size:/ {block = block "\n" $0; next}               # Acumular las líneas del bloque `stack_size:`
    /^- name:/ || /expected_output:/ || /expected_stack:/ {     # Desactivar flag al encontar otra sección
        if (flag) {print block; block=""}                       # Imprimir el bloque acumulado
        flag=0
    }
    flag {block = block "\n" substr($0, 3)}                     # Acumular las líneas del bloque `code: |`
    END {if (flag) print block}                                 # Imprimir el último bloque acumulado
' "$YAML_FILE")

# echo "Blocks:"
# echo $BLOCKS

# Extaer el output esperado y el stack esperado
EXPECTED_OUTPUT_LINES=()
while IFS= read -r line; do
    EXPECTED_OUTPUT_LINES+=("$line")
done <<< "$(grep "expected_output:" "$YAML_FILE" | sed -E 's/expected_output: "(.*)"/\1/')"

EXPECTED_STACK_LINES=()
while IFS= read -r line; do
    EXPECTED_STACK_LINES+=("$line")
done <<< "$(grep "expected_stack:" "$YAML_FILE" | sed -E 's/expected_stack: \[(.*)\]/\1/' | sed 's/, / /g' | sed 's/^ *//;s/ *$//')"

# Función para procesar un bloque
process_block() {
    local block_number=$1
    local current_block=$2
    local block_name=$3

    echo "Executing block $block_number:"
    echo $current_block
    echo "-----------------------------"

    # Extraer el tamaño del stack para este bloque
    STACK_SIZE_LINE=$(echo "$current_block" | grep "stack_size:" | sed -nE 's/stack_size: *([0-9]+).*/\1/p' | xargs)
    
    # Guardar el bloque actual en un archivo temporal
    echo -e "$current_block" | grep -v "stack_size:" | sed 's/^[[:space:]]*//' > block.fth

    # Ejecutar el programa con el bloque actual
    if [[ -n "$STACK_SIZE_LINE" ]]; then
        RAW_OUTPUT=$(cargo run block.fth stack-size=$STACK_SIZE_LINE 2>&1)
    else
        RAW_OUTPUT=$(cargo run block.fth 2>&1)
    fi

    echo "Raw output:"
    echo $RAW_OUTPUT

    OUTPUT=$(echo "$RAW_OUTPUT" | grep -vE "Executing instruction|Finished|Running|Compiling")
    
    echo "Filtered output:"
    echo "$OUTPUT"

    # Leer el último estado del stack
    STACK=$(tail -n 1 stack.fth 2>/dev/null | xargs)

    # Obtener el stack esperado para este bloque
    EXPECTED_STACK_LINE=$(echo "${EXPECTED_STACK_LINES[$((BLOCK_NUMBER -1))]}" | xargs)
    
    # Obtener el output esperado para este bloque
    EXPECTED_OUTPUT=$(echo -e "${EXPECTED_OUTPUT_LINES[$((BLOCK_NUMBER - 1))]}" | xargs)
    
    if [[ -z "$EXPECTED_OUTPUT" ]]; then
        echo "Warning: No expected output for block $block_number"
    fi

    echo "STACK_SIZE_LINE: $STACK_SIZE_LINE"
    echo "STACK:"
    echo "$STACK"

    # Comparar el stack esperado con el stack actual
    if [[ "$STACK" != "$EXPECTED_STACK_LINE" ]]; then
        echo "Test failed for $YAML_FILE at block $block_number"
        echo "Expected stack: $EXPECTED_STACK_LINE"
        echo "Actual stack: $STACK"
        TEST_FAILED=true
        return
    fi

    # Normalizar el output actual y el esperado
    NORMALIZED_OUTPUT=$(echo -e "$OUTPUT" | tr -s '\n' ' ' | sed 's/[[:space:]]\+$//')
    NORMALIZED_EXPECTED_OUTPUT=$(echo -e "$EXPECTED_OUTPUT" | sed 's/[[:space:]]\+/ /g')

    echo "Normalized output: '$NORMALIZED_OUTPUT'"
    echo "Normalized expected output: '$NORMALIZED_EXPECTED_OUTPUT'"

    # Comparar el output normalizado
    if [[ "$NORMALIZED_OUTPUT" != "$NORMALIZED_EXPECTED_OUTPUT" ]]; then
        echo "Test failed for $YAML_FILE at block $block_number"
        echo "Expected output: $NORMALIZED_EXPECTED_OUTPUT"
        echo "Actual output: $NORMALIZED_OUTPUT"
        TEST_FAILED=true
        return
    fi

    # Limpiar el bloque actual y el archivo temporal
    > block.fth
}

# Leer el archvo por cada bloque
BLOCK_NUMBER=0
CURRENT_BLOCK="" # Variable auxiliar para acumular el bloque actual
IFS=''
while IFS= read -r BLOCK; do

    # Si se encuentra una línea vacía, se procesa el bloque acumalado
    if [[ -z "$BLOCK" ]]; then
        if [[ -n "$CURRENT_BLOCK" ]]; then
            BLOCK_NUMBER=$((BLOCK_NUMBER + 1))
            BLOCK_NAME=$(grep -A 1 "name:" "$YAML_FILE" | sed -n "${BLOCK_NUMBER}p" | sed 's/name: "\(.*\)"/\1/')
            process_block "$BLOCK_NUMBER" "$CURRENT_BLOCK" "$BLOCK_NAME"
            CURRENT_BLOCK=""
        fi
    else
        # Acumular la línea actual en CURRENT_BLOCK
        CURRENT_BLOCK="${CURRENT_BLOCK}${BLOCK}\n"
    fi

    if [[ "$TEST_FAILED" == true ]]; then
        exit 1
    fi
    
done <<< "$BLOCKS"

# Procesar el último bloque acumulado si existe
if [[ -n "$CURRENT_BLOCK" ]]; then
    BLOCK_NUMBER=$((BLOCK_NUMBER + 1))
    BLOCK_NAME=$(grep -A 1 "name:" "$YAML_FILE" | sed -n "${BLOCK_NUMBER}p" | sed 's/name: "\(.*\)"/\1/')
    process_block "$BLOCK_NUMBER" "$CURRENT_BLOCK" "$BLOCK_NAME"
fi

if [[ "$TEST_FAILED" == true ]]; then
    exit 1
fi

echo "Test passed for $YAML_FILE"