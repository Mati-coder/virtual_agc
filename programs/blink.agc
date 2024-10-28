.code
BLINK:
    CA ANCHOPANT 
# Bucle que enciende toda la pantalla
BUCLEB1:
    TS I

    CA FILACOMP

    INDEX I
    TS PANT

    CCS I

    TCF BUCLEB1

DELAYB:
    # Delay
    CA MEDIO
    TS CICLOS
    TC DELAY

# Bucle de actualizacion de la pantalla
    CA ANCHOPANT
BUCLEB2:
    TS I
    # Carga el negativo del valor de la primer fila de la pantalla
    # Si esta esta encendida, entonces la apagará, y viceversa
    CS PANT 

    INDEX I
    TS PANT

    CCS I

    TCF BUCLEB2

    # Chequea si debe salir del programa
    CA BTN1
    EXTEND
    BZF DELAYB
    TC LIMPPANT
    TCF INICIO


.data
FILACOMP: # El valor de una fila completamene iluminada
    DEC 255