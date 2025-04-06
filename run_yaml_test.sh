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

echo "Blocks extracted from $YAML_FILE:"
echo "$BLOCKS"

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

# Leer el archvo por cada bloque
BLOCK_NUMBER=0
CURRENT_BLOCK="" # Variable auxiliar para acumular el bloque actual
IFS=''
echo "$BLOCKS" | while IFS= read -r BLOCK; do

    # Si se encuentra una línea vacía, se procesa el bloque acumalado
    if [[ -z "$BLOCK" ]]; then
        if [[ -n "$CURRENT_BLOCK" ]]; then
            BLOCK_NUMBER=$((BLOCK_NUMBER + 1))
            echo "Executing block $BLOCK_NUMBER:"
            echo "Debug: Content of BLOCK:"
            echo "$CURRENT_BLOCK"
            > block.fth
            echo -e "$CURRENT_BLOCK" > block.fth        # Sobreescribir el archivo con el bloque completo
            echo "Content of block.fth:"
            cat block.fth

            sleep 0.1

            # Ejecutar el programa con la línea actual
            OUTPUT=$(cargo run block.fth 2>&1)
            echo "Program output:"
            echo "$OUTPUT"

            # Imprimir el contenido del archivo stack.fth después de ejecutar la instrucción
            # echo "Content of stack.fth after executing line $LINE_NUMBER:"
            # cat stack.fth

            # Comparar el stack actual con el valor esperado
            STACK=$(tail -n 1 stack.fth 2>/dev/null | xargs) # Leer el último estado del stack
            # STACK=$(tac stack.fth | xargs) # Invertir el contenido del stack y procesarlo
            # echo "Debug: Raw stack line from stack.fth:"
            # cat stack.fth
            # echo "Debug: Processed stack line: $STACK"
            
            EXPECTED_STACK_LINE=$(echo "${EXPECTED_STACK_LINES[$((BLOCK_NUMBER -1))]}" | xargs)
            
            echo "Debug: Block number: $BLOCK_NUMBER"
            echo "Debug: expected stack line: ${EXPECTED_STACK_LINES[$((LINE_NUMBER))]}"
            echo "Debug: actual stack line: $STACK"



            # Si el estado esperado es un stack vacío (representado como "[]"), se lo convierte en una cadena vacía
            # if [[ "$EXPECTED_STACK_LINE" == "[]" ]]; then
            #     EXPECTED_STACK_LINE=""
            # fi

            if [[ -z "$STACK" ]]; then
                STACK=""
            fi

            echo "Expected stack line: $EXPECTED_STACK_LINE"
            echo "Actual stack line: $STACK"
            if [[ "$STACK" != "$EXPECTED_STACK_LINE" ]]; then
                echo "Test failed for $YAML_FILE at block $BLOCK_NUMBER"
                echo "Expected stack: $EXPECTED_STACK_LINE"
                echo "Actual stack: $STACK"
                exit 1
            fi

            echo " "

            # Limpiar el bloque actual y el archivo temporal
            CURRENT_BLOCK=""
            > block.fth
        fi
    else
        # Acumular la línea actual en CURRENT_BLOCK
        CURRENT_BLOCK="${CURRENT_BLOCK}${BLOCK}\n"
    fi
done

echo "Test passed for $YAML_FILE"