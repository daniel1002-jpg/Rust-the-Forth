#!/bin/bash

# Archivo YAML a procesar
YAML_FILE=$1

# Extraer únicamente las líneas de los bloque `code: |`
CODES=$(awk '
    /code: \|/ {flag=1; next}
    /^- name:/ || /expected_output:/ || /expected_stack:/ {flag=0}           # Desactivar flag al encotrar un nuevo bloque de prueba
    flag {print}                                                             # Imprimir solo so el flag está activo y la línea no está vacía
' "$YAML_FILE")

# Crear un archivo temporal con las instrucciones
echo "$CODES" | sed 's/^ *//' > tmp.fth

#Dividir el stack esperado en líneas
EXPECTED_OUTPUT=$(grep "expected_output:" "$YAML_FILE" | sed 's/expected_output: "\(.*\)"/\1/')
EXPECTED_STACK=$(grep "expected_stack:" "$YAML_FILE" | sed -E 's/expected_stack: \[(.*)\]/\1/' | sed 's/, / /g' | sed 's/^ *//;s/ *$//')

# Si el stack esperado es "[]", conviértelo en una cadena vacía
if [[ "$EXPECTED_STACK" == "[]" ]]; then
    EXPECTED_STACK=""
fi

# Convertir el formato de EXPECTED_STACK (de '1, 2, 3, 4, 5' a '1 2 3 4 5')
EXPECTED_STACK=$(echo "$EXPECTED_STACK" | sed 's/, / /g' | sed 's/^ *//;s/ *$//')

# Dividir el stack esperado en líneas
EXPECTED_STACK_LINES=()
while IFS= read -r line; do
    EXPECTED_STACK_LINES+=("$line")
done <<< "$(grep "expected_stack:" "$YAML_FILE" | sed -E 's/expected_stack: \[(.*)\]/\1/' | sed 's/, / /g' | sed 's/^ *//;s/ *$//')"
# IFS=$'\n' read -d '' -r -a EXPECTED_STACK_LINES <<< "$EXPECTED_STACK"

# Leer el archivo línea por línea
LINE_NUMBER=0
while IFS= read -r LINE; do
    LINE_NUMBER=$((LINE_NUMBER + 1))
    echo "Executing line $LINE_NUMBER: $LINE"
    echo "$LINE" > line.fth # Crear un archivo temporal con la línea actual

    sleep 0.1

    # Ejecutar el programa con la línea actual
    OUTPUT=$(cargo run line.fth 2>/dev/null)

    # Comparar la salida con el valor esperado
    if [[ "$OUTPUT" != "$EXPECTED_OUTPUT" ]]; then
        echo "Test failed for $YAML_FILE at line $LINE_NUMBER"
        echo "Expected output: $EXPECTED_OUTPUT"
        echo "Actual output: $OUTPUT"
        exit 1
    fi

    # Comparar el stack actual con el valor esperado
    STACK=$(tail -n 1 stack.fth 2>/dev/null | xargs) # Leer el último estado del stack
    echo "Debug: Raw stack line from stack.fth:"
    cat stack.fth
    echo "Debug: Processed stack line: $STACK"
    
    EXPECTED_STACK_LINE=$(echo "${EXPECTED_STACK_LINES[$((LINE_NUMBER -1))]}" | xargs)
    echo "Debug: Line number: $LINE_NUMBER"
    # echo "Debug: Expected stack lines array: ${EXPECTED_STACK_LINES[@]}"
    echo "Debug: Raw expected stack line: ${EXPECTED_STACK_LINES[$((LINE_NUMBER -1))]}"
    echo "Debug: Processed expected stack line: $EXPECTED_STACK_LINE"



    # Si el estado esperado es un stack vacío (representado como "[]"), se lo convierte en una cadena vacía
    if [[ "$EXPECTED_STACK_LINE" == "[]" ]]; then
        EXPECTED_STACK_LINE=""
    fi

    if [[ -z "$STACK" ]]; then
        STACK=""
    fi

    echo "Expected stack line: $EXPECTED_STACK_LINE"
    echo "Actual stack line: $STACK"
    if [[ "$STACK" != "$EXPECTED_STACK_LINE" ]]; then
        echo "Test failed for $YAML_FILE at line $LINE_NUMBER"
        echo "Expected stack: $EXPECTED_STACK_LINE"
        echo "Actual stack: $STACK"
        exit 1
    fi

    echo " "
done < tmp.fth

echo "Test passed for $YAML_FILE"