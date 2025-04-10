#!/bin/bash

# Archivo YAML a procesar
YAML_FILE=$1

# Variables para el resumen global
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Extraer únicamente las líneas de los bloque `code: |`
BLOCKS=$(awk '
    /code: \|/ {flag=1; next}                                   # Activar flag al encontrar `code: |` y limpiar el bloque
    /expected_output:/ {block = block "\n" $0; next}               
    /stack_size:/ {block = block "\n" $0; next}               # Acumular las líneas del bloque `stack_size:`
    /name:/ {block = block "\n" $0; next}
    /expected_stack:/ {     # Desactivar flag al encontar otra sección
        if (flag) {print block; block=""}                       # Imprimir el bloque acumulado
        flag=0
    }
    flag {block = block "\n" substr($0, 3)}                     # Acumular las líneas del bloque `code: |`
    END {if (flag) print block}                                 # Imprimir el último bloque acumulado
' "$YAML_FILE")

EXPECTED_STACK_LINES=()
while IFS= read -r line; do
    EXPECTED_STACK_LINES+=("$line")
done <<< "$(grep "expected_stack:" "$YAML_FILE" | sed -E 's/expected_stack: \[(.*)\]/\1/' | sed 's/, / /g' | sed 's/^ *//;s/ *$//')"

# Función para procesar un bloque
process_block() {
    local block_number=$1
    local current_block=$2

    TEST_NAME=$(echo "$current_block" | grep "name:" | sed -E 's/name: "(.*)"/\1/' | sed 's/\\n.*//' | tr -d '"')

    echo "-----------------------------"
    echo "Executing test $TEST_NAME:"
    echo "-----------------------------"

    # Extraer el tamaño del stack para este bloque
    STACK_SIZE_LINE=$(echo "$current_block" | grep "stack_size:" | sed -nE 's/stack_size: *([0-9]+).*/\1/p')
    STACK_SIZE_LINE=$(echo "$current_block" | grep -oP '(?<=stack_size: )\d+' || echo "")

    # Extraer el output esperado
    EXPECTED_OUTPUT=$(echo "$current_block" | grep -oP '(?<=expected_output: ").*(?=")' || echo "")
    
    # Guardar el bloque actual en un archivo temporal
    echo -e "$current_block" | grep -v "stack_size:" | sed '/expected_output:/,/"/d' | grep -v "name:" | sed 's/^[[:space:]]*//' > block.fth

    # Ejecutar el programa con el bloque actual
    if [[ -n "$STACK_SIZE_LINE" ]]; then
        RAW_OUTPUT=$(cargo run block.fth stack-size=$STACK_SIZE_LINE 2>&1)
    else
        RAW_OUTPUT=$(cargo run block.fth 2>&1)
    fi

    OUTPUT=$(echo "$RAW_OUTPUT" | grep -vE "Executing instruction|Finished|Running|Compiling")
    
    # Leer el último estado del stack
    STACK=$(tail -n 1 stack.fth 2>/dev/null | xargs)

    # Obtener el stack esperado para este bloque
    EXPECTED_STACK_LINE=$(echo "${EXPECTED_STACK_LINES[$((BLOCK_NUMBER -1))]}" | xargs)
    
    # Comparar el stack esperado con el stack actual
    if [[ "$STACK" != "$EXPECTED_STACK_LINE" ]]; then
        echo -e "\e[31m❌ Test failed for $TEST_NAME\e[0m"
        echo "Expected stack: $EXPECTED_STACK_LINE"
        echo "Actual stack: $STACK"
        FAILED_TESTS=$((FAILED_TESTS + 1))
        return
    fi

    # Comparar el output esperado (si está definido)
    if [[ -n "$EXPECTED_OUTPUT" ]]; then
        # Normalizar el output actual y el esperado
        NORMALIZED_OUTPUT=$(echo -e "$OUTPUT" | tr -s '\n' ' ' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')
        NORMALIZED_EXPECTED_OUTPUT=$(echo -e "$EXPECTED_OUTPUT" | tr -s '\n' ' ' | sed 's/^[[:space:]]*//;s/[[:space:]]*$//')

        # Comparar el output normalizado
        if [[ "$NORMALIZED_OUTPUT" != "$NORMALIZED_EXPECTED_OUTPUT" ]]; then
            echo -e "\e[31m❌ Test failed for $TEST_NAME\e[0m"
            echo "Expected output: $NORMALIZED_EXPECTED_OUTPUT"
            echo "Actual output: $NORMALIZED_OUTPUT"
            FAILED_TESTS=$((FAILED_TESTS + 1))
            return
        fi
    fi

    echo -e "\e[32m✅ Test passed for $TEST_NAME\e[0m"
    PASSED_TESTS=$((PASSED_TESTS + 1))

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
            process_block "$BLOCK_NUMBER" "$CURRENT_BLOCK"
            CURRENT_BLOCK=""
        fi
    else
        # Acumular la línea actual en CURRENT_BLOCK
        CURRENT_BLOCK="${CURRENT_BLOCK}${BLOCK}\n"
    fi
    
done <<< "$BLOCKS"

# Procesar el último bloque acumulado si existe
if [[ -n "$CURRENT_BLOCK" ]]; then
    BLOCK_NUMBER=$((BLOCK_NUMBER + 1))
    process_block "$BLOCK_NUMBER" "$CURRENT_BLOCK"
fi

# Mostrar resumen del archivo
# echo "-----------------------------------------------"
# echo "Summary for $YAML_FILE:"
# echo "Total test: $BLOCK_NUMBER"
# echo "Passed: $PASSED_TESTS"
# echo "Failed: $FAILED_TESTS"
# echo "-----------------------------------------------"

# Actualizar el resumen global
TOTAL_TESTS=$((TOTAL_TESTS + BLOCK_NUMBER))

# Acumular el resumen del archivo en el archivo global
echo "-----------------------------------------------" >> tests_summary.log
echo "Summary for $YAML_FILE:" >> tests_summary.log
echo "Total tests: $BLOCK_NUMBER" >> tests_summary.log
echo "Passed: $PASSED_TESTS" >> tests_summary.log
echo "Failed: $FAILED_TESTS" >> tests_summary.log
echo "-----------------------------------------------" >> tests_summary.log

if [[ $FAILED_TESTS -gt 0 ]]; then
    exit 1
fi