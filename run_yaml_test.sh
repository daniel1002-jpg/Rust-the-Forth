#!/bin/bash

# Archivo YAML a procesar
YAML_FILE=$1

# Extraer únicamente las líneas de los bloque `code: |`
BLOCKS=$(awk '
    /code: \|/ {flag=1; next}                                   # Activar flag al encontrar `code: |` y limpiar el bloque
    /^- name:/ || /expected_output:/ || /expected_stack:/ {     # Desactivar flag al encontar otra sección
        if (flag) {print block; block=""}                       # Imprimir el bloque acumulado
        flag=0
    }
    flag {block = block "\n" substr($0, 3)}                     # Acumular las líneas del bloque `code: |`
    END {if (flag) print block}                                 # Imprimir el último bloque acumulado
' "$YAML_FILE")

# Extaer el output esperado y el stack esperado
# EXPECTED_OUTPUT=$(grep "expected_output:" "$YAML_FILE" | sed 's/expected_output: "\(.*\)"/\1/')
EXPECTED_OUTPUT_LINES=()
while IFS= read -r line; do
    EXPECTED_OUTPUT_LINES+=("$line")
done <<< "$(grep "expected_output:" "$YAML_FILE" | sed -E 's/expected_output: "(.*)"/\1/')"

EXPECTED_STACK_LINES=()
while IFS= read -r line; do
    EXPECTED_STACK_LINES+=("$line")
done <<< "$(grep "expected_stack:" "$YAML_FILE" | sed -E 's/expected_stack: \[(.*)\]/\1/' | sed 's/, / /g' | sed 's/^ *//;s/ *$//')"

# Leer el archvo por cada bloque
BLOCK_NUMBER=0
CURRENT_BLOCK="" # Variable auxiliar para acumular el bloque actual
IFS=''
while IFS= read -r BLOCK; do

    # Si se encuentra una línea vacía, se procesa el bloque acumalado
    if [[ -z "$BLOCK" ]]; then
        if [[ -n "$CURRENT_BLOCK" ]]; then
            BLOCK_NUMBER=$((BLOCK_NUMBER + 1))
            
            echo "Executing block $BLOCK_NUMBER:"
            echo $CURRENT_BLOCK
            echo "-----------------------------"
            
            # Guardar el bloque actual en un archivo temporal
            echo -e "$CURRENT_BLOCK" | sed 's/^[[:space:]]*//' > block.fth
            
            # Ejecutar el programa con el bloque actual
            RAW_OUTPUT=$(cargo run block.fth 2>&1)

            echo "Raw output:"
            echo $RAW_OUTPUT

            OUTPUT=$(echo "$RAW_OUTPUT" | grep -vE "Executing instruction|Finished|Running")
            
            echo "Filtered output:"
            echo "$OUTPUT"

            # Leer el último estado del stack
            STACK=$(tail -n 1 stack.fth 2>/dev/null | xargs)

            # Obtener el stack esperado para este bloque
            EXPECTED_STACK_LINE=$(echo "${EXPECTED_STACK_LINES[$((BLOCK_NUMBER -1))]}" | xargs)
            
            # Obtener el output esperado para este bloque
            EXPECTED_OUTPUT=$(echo -e "${EXPECTED_OUTPUT_LINES[$((BLOCK_NUMBER - 1))]}" | xargs)
            
            # EXPECTED_OUTPUT=$(awk -v block_number="$BLOCK_NUMBER" '
            #     BEGIN {count=0}
            #     /^- name:/ {count++}
            #     count == block_number {
            #         if ($0 ~ /expected_output:/) {
            #             if (match($0, /expected_output: "(.*)"/, arr)) {
            #                 print arr[1]
            #             } else {
            #                 print ""
            #             }
            #             exit
            #         }
            #     }
            #     END {if (count == block_number) print ""} # Si no hay expected_output, devolver vacío
            # ' "$YAML_FILE")

            if [[ -z "$EXPECTED_OUTPUT" ]]; then
                echo "Warning: No expected output for block $BLOCK_NUMBER"
            fi

            # Comparar el stack esperado con el stack actual
            if [[ "$STACK" != "$EXPECTED_STACK_LINE" ]]; then
                echo "Test failed for $YAML_FILE at block $BLOCK_NUMBER"
                echo "Expected stack: $EXPECTED_STACK_LINE"
                echo "Actual stack: $STACK"
                TEST_FAILED=true
                break
            fi

            # Normalizar el output actual y el esperado
            NORMALIZED_OUTPUT=$(echo -e "$OUTPUT" | tr -s '\n' ' ' | sed 's/[[:space:]]\+$//')
            NORMALIZED_EXPECTED_OUTPUT=$(echo -e "$EXPECTED_OUTPUT" | sed 's/[[:space:]]\+/ /g')

            echo "Normalized output: '$NORMALIZED_OUTPUT'"
            echo "Normalized expected output: '$NORMALIZED_EXPECTED_OUTPUT'"

            # Comparar el output normalizado
            if [[ "$NORMALIZED_OUTPUT" != "$NORMALIZED_EXPECTED_OUTPUT" ]]; then
                echo "Test failed for $YAML_FILE at block $BLOCK_NUMBER"
                echo "Expected output: $NORMALIZED_EXPECTED_OUTPUT"
                echo "Actual output: $NORMALIZED_OUTPUT"
                TEST_FAILED=true
                break
            fi

            # Limpiar el bloque actual y el archivo temporal
            CURRENT_BLOCK=""
            > block.fth
        fi
    else
        # Acumular la línea actual en CURRENT_BLOCK
        CURRENT_BLOCK="${CURRENT_BLOCK}${BLOCK}\n"
    fi
done <<< "$BLOCKS"

if [[ "$TEST_FAILED" == true ]]; then
    exit 1
fi

echo "Test passed for $YAML_FILE"