.config
    VEC PROGS 5
.code
INICIO:
# Actualiza la pantalla
    INDEX PRG
    CA PROGS
    TS PANT

# Delay
    CA CORTO
    TS CICLOS
    TC DELAY

# Chequea si se presionó un boton
# El boton derecho aumenta el numero de programa, el izquierdo lo disminuye
    CA BTNRGT
    EXTEND
    BZF S1
    INCR PRG 
S1:
    CA BTNLFT
    EXTEND
    BZF S2
    EXTEND
    DIM PRG

# Chequea que PRG no se pase del valor maximo, si es asi, lo restablece a este
S2:
    CS PRG # Carga -PRG 
    AD MAXPRG # Suma MAXPRG
    COM # Niega el acumulador.
    #Luego de las previas 3 operaciones, se encuentra almacenado el valor de [PRG - MAXPRG], que solo sera positivo si PRG > MAXPRG
    EXTEND
    BZMF S3
    CA MAXPRG
    TS PRG

# Si se presiona el boton, se ejecutara uno de los programas de la lista, segun el valor de PRG
S3:
    CA BTN1
    EXTEND
    BZF INICIO
    INDEX PRG
    TCF PDIR
    
# Lista de programas pre-cargados
PDIR:
    TCF BLINK
    TCF FOR
    TCF IF
    TCF PLAYER
    TCF LABERINTO

.data
MAXPRG: # Almacena el maximo de programas - 1
    DEC 4
PROGS:  # Diferentes valores de la pantalla segun el programa seleccionado
    DEC 1
    DEC 3
    DEC 7
    DEC 15
    DEC 31