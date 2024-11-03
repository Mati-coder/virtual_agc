.code
# Subrutina que controla el movimiento
# Los botones arriba y abajo disminuyen y aumentan Y, respectivamente
# Los botones derecha e izquierda aumentan y disminuyen X, respectivamente
# Al final se chequea si X o Y superaron su valor maximo. Si es asi, se restablecen a MAXXY
MOVIMIENTO:
    CCS BTNDWN

    INCR Y

    CA BTNUP
    EXTEND
    BZF B1
    EXTEND
    DIM Y
B1:
    CCS BTNRGT

    INCR X

    CA BTNLFT
    EXTEND
    BZF B2
    EXTEND
    DIM X
B2:
    CS X
    AD MAXXY
    COM
    EXTEND
    BZF B3
    CA MAXXY
    TS X
B3:
    CS Y
    AD MAXXY
    COM
    EXTEND
    BZF B4
    CA MAXXY
    TS Y
B4:
    RETURN

# Subrutina que ejecuta CICLOS bucles para perder tiempo
DELAY:
    CA CICLOS
DELAYL:
    TS I
    CCS I

    TCF DELAYL

    RETURN

# Subrutina que carga un 0 en todas las filas de la pantalla
LIMPPANT:
    CA ANCHOPANT
BUCLELP:
    TS I
    
    CA ZERO
    INDEX I
    TS PANT

    CCS I

    TCF BUCLELP

    RETURN

.data
ANCHOPANT: # Ancho de la pantalla - 1
    DEC 7
MAXXY: # Valor maximo de X e Y
    DEC 7

