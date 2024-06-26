ENTRY: 
    CA LEN		    cargar el largo de la tabla en el acc
    EXTEND		
    DIM ACC		    restarle 1

LOOP:
    TS TBLINDEX		guardar en memoria el contenido del acc
    CA TBLSUM		cargar la suma total en el acc
    INDEX TBLINDEX	le suma el contenido de tblindex a la instruccion ad

    AD TABLE		le suma al acumulador el contenido de la direccion (table + tblindex)

    TS TBLSUM		guarda la suma en memoria
    CCS TBLINDEX	

    TCF LOOP

    TCF EXIT
    
LEN:            
    DEC 5
TABLE:
    DEC 2
    DEC 0
    DEC 6
    DEC 32761
    DEC 71

EXIT:
    CA TBLSUM
DONOTHING:
    TCF DONOTHING
 