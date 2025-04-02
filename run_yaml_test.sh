#!/bin/bash

# Archivo YAML a procesar
YAML_FILE=$1

# Extraer los valores de 'code', 'expected_output' y 'expected_stack' usando 'yq'
CODE=$(grep "expected_output:" "$YAML_FILE" | sed -n '/code: |/,$p' | sed '1d' | sed '/expected_output:/,$d')
EXPECTED_OUTPUT=$(grep "expected_output:" "$YAML_FILE" | sed 's/expected_output: "\(.*\)"/\1/')
EXPECTED_STACK=$(grep "expected_stack:" "$YAML_FILE" | sed 's/expected_stack: \[\(.*\)\]/\1/')

# Crear un archivo temporal con las instrucciones
echo "$CODE" > tmp.fth

# Ejecutar el programa con las instrucciones
OUTPUT=$(cargo run --temp.fth 2>/dev/null)

# Comparar la salida con el valor esperado
if [[ "$OUTPUT" != "$EXPECTED_OUTPUT"]]; then
    echo "Test failed for $YAML_FILE"
    echo "Expected output: $EXPECTED_OUTPUT"
    echo "Actual output: $OUTPUT"
    exit 1
fi

# Comparar el stack final con el valor esperado
STACK=$(tail -n 1 stack.fth | xargs) # Leer el último estado del stack
if [[ "$STACK" != "$EXPECTED_STACK" ]]; then
    echo "Test failed for $YAML_FILE"
    echo "Expected stack: $EXPECTED_STACK"
    echo "Actual stack: $STACK"
    exit 1
fi

echo "Test passed for $YAML_FILE"